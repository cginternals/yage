use yage_core::{
    Context, GlFunctions,
    glenum, cgmath,
    check_error,
    Program, Shader, Buffer,
    GpuObject, Render, Update, Drawable, Animation,
    Texture, TextureLoader,
    Geometry, VertexAttribute, Primitive,
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    geometry: Geometry,
    program: Program,
    texture: Texture,
    animation: Animation<f32>,
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
            texture: Texture::new(gl::TEXTURE_2D),
            animation: Animation::new(0.0, 1.0, 2.0, true, true, true),
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
                4 * std::mem::size_of::<f32>(),
                gl::FLOAT,
                2,
                false
            );

            // Create vertex attribute for 'texcoord'
            let va_texcoord = VertexAttribute::new(
                buffer_index,
                0,
                2 * std::mem::size_of::<f32>(),
                4 * std::mem::size_of::<f32>(),
                gl::FLOAT,
                2,
                false
            );

            // Add vertex attributes
            let position_index = self.geometry.add_vertex_attribute(va_position);
            let texcoord_index = self.geometry.add_vertex_attribute(va_texcoord);

            // Create primitive
            let mut primitive = Primitive::new();
            primitive.set_attribute_binding(0, position_index);
            primitive.set_attribute_binding(1, texcoord_index);
            primitive.set_index_buffer(None);
            primitive.set_render_mode(gl::TRIANGLE_STRIP);
            primitive.set_count(4);

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

        // Create texture
        self.texture = Texture::new(gl::TEXTURE_2D);
        self.texture.init(context);
        {
            // Load texture
            TextureLoader::load(context, &mut self.texture, "data/duck.jpg");
            check_error!();
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
        self.texture.deinit(context);
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        self.animation.needs_update()
    }

    fn update(&mut self, time_delta: f64) {
        self.animation.update(time_delta);
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

        // Bind texture
        self.texture.bind_active(context, 0);
        check_error!();

        // Bind program and set uniforms
        self.program.use_program(context);
        self.program.set_uniform(context, "tex", 0);
        self.program.set_uniform(context, "color", (0.4, 0.8, 0.4));
        self.program.set_uniform(context, "animation", self.animation.get_value());
        check_error!();

        // Draw geometry
        self.geometry.draw(context);
        check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
layout (location = 0) in vec2 position;
layout (location = 1) in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_texcoord = texcoord;
}";

const FS_SRC: &str = "
#version 330 core
precision mediump float;
uniform sampler2D tex;
uniform vec3 color = vec3(1.0, 1.0, 1.0);
uniform float animation = 1.0;
in vec2 v_texcoord;
out vec4 FragColor;
void main() {
    // FragColor = vec4(v_texcoord.x, v_texcoord.y, 0.0, 1.0);
    FragColor = vec4(texture(tex, v_texcoord).rgb * color * vec3(animation), 1.0);
}";

#[rustfmt::skip]
static VERTEX_DATA: [f32; 16] = [
    -0.5,  0.5,  0.0,  1.0,
    -0.5, -0.5,  0.0,  0.0,
     0.5,  0.5,  1.0,  1.0,
     0.5, -0.5,  1.0,  0.0,
];
