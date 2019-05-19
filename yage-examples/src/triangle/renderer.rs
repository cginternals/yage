use std::f32::consts::PI;

use yage_core::{
    Context, GlFunctions,
    glenum, cgmath,
    check_error,
    Program, Shader, Buffer,
    GpuObject, Render, Update, Drawable,
    Geometry, VertexAttribute, Primitive,
    Animation, ColorRotation
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    geometry: Geometry,
    program: Program,
    animation: Animation<f32>,
    color_rotation: ColorRotation,
    frame_count: i32,
    redraw: bool
}

impl Renderer {
    ///
    /// Create a renderer.
    ///
    /// # Returns
    /// A new instance of Renderer.
    ///
    pub fn new() -> Renderer {
        Renderer {
            initialized: false,
            geometry: Geometry::new(),
            program: Program::new(),
            animation: Animation::new(0.0, 2.0 * PI, 4.0, true, false, true),
            color_rotation: ColorRotation::new(),
            frame_count: 0,
            redraw: false
        }
    }
}

impl GpuObject for Renderer {
    fn init(&mut self, context: &Context) {
        // Abort if already initialized
        if self.initialized {
            return;
        }

        // [DEBUG]
        println!("initializing renderer");

        // Create geometry
        self.geometry = Geometry::new();
        {
            // Create vertex buffer
            let mut buffer = Buffer::new(glenum::BufferKind::Array as _);
            buffer.init(context);
            buffer.bind(context);
            buffer.set_data(context, &VERTEX_DATA, glenum::DrawMode::Static as _);

            // Add vertex buffer
            let buffer_index = self.geometry.add_buffer(buffer);

            // Create vertex attribute for 'position'
            let va_position = VertexAttribute::new(
                buffer_index,
                0,
                0,
                5 * std::mem::size_of::<f32>(),
                gl::FLOAT,
                2,
                false
            );

            // Create vertex attribute for 'color'
            let va_color = VertexAttribute::new(
                buffer_index,
                0,
                2 * std::mem::size_of::<f32>(),
                5 * std::mem::size_of::<f32>(),
                gl::FLOAT,
                3,
                false
            );

            // Add vertex attributes
            let position_index = self.geometry.add_vertex_attribute(va_position);
            let color_index = self.geometry.add_vertex_attribute(va_color);

            // Create primitive
            let primitive = Primitive::new(
                0,
                gl::TRIANGLES,
                3,
                None,
                0,
                &[ (0, position_index), (1, color_index) ]
            );

            // Add primitive
            self.geometry.add_primitive(primitive);
        }

        // Create shader program
        self.program = Program::new();
        self.program.init(context);
        {
            // Load vertex shader
            let mut vertex_shader = Shader::new(glenum::ShaderKind::Vertex);
            vertex_shader.set_code(context, VS_SRC, &[]);

            // Load fragment shader
            let mut fragment_shader = Shader::new(glenum::ShaderKind::Fragment);
            fragment_shader.set_code(context, FS_SRC, &[]);

            // Attach shaders
            self.program.attach(vertex_shader);
            self.program.attach(fragment_shader);
        }

        // Done
        self.initialized = true;
    }

    fn deinit(&mut self, context: &Context) {
        // Abort if not initialized
        if !self.initialized {
            return;
        }

        // [DEBUG]
        println!("de-initializing renderer");

        // De-Initialize OpenGL objects
        self.geometry.deinit(context);
        self.program.deinit(context);
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        self.animation.needs_update()
    }

    fn update(&mut self, time_delta: f64) {
        self.animation.update(time_delta);
        self.color_rotation.set_angle(self.animation.get_value());
        self.redraw = true;
    }
}

impl Render for Renderer {
    fn set_viewport(&mut self, _viewport: cgmath::Vector4<i32>) {
        // We don't care as the viewport is correctly set by the canvas
    }

    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn render(&mut self, context: &Context) {
        // [DEBUG]
        //println!("frame #{}", self.frame_count);
        self.frame_count = self.frame_count + 1;

        // Clear background
        context.gl().clear_color(0.1, 0.2, 0.3, 1.0);
        context.gl().clear(glenum::BufferBit::Color as u32);
        check_error!();

        // Bind program and set uniforms
        self.program.use_program(context);
        self.program.set_uniform(context, "color_matrix", &self.color_rotation.get_matrix());
        check_error!();

        // Draw geometry
        self.geometry.draw(context);
        check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
uniform mat3 color_matrix;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = clamp(color_matrix * color, vec3(0.0), vec3(1.0));
}";

const FS_SRC: &str = "
#version 330 core
precision mediump float;
in vec3 v_color;
out vec4 FragColor;
void main() {
    FragColor = vec4(v_color, 1.0);
}";

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];
