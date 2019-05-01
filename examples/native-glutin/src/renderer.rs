use cgmath::Vector4;

use yage::core::{
    Context, GlFunctions,
    glenum,
    check_error,
    Program, Buffer, VertexArray,
    GpuObject, Render, Update
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    program: Option<Program>,
    vertex_buffer: Option<Buffer>,
    vao: Option<VertexArray>,
    frame_count: i32,
    animation: f64,
    redraw: bool
}

impl Renderer {
    ///
    /// Create a renderer instance
    ///
    /// # Returns
    /// A new instance of Renderer.
    ///
    pub fn new() -> Renderer {
        Renderer {
            initialized: false,
            program: None,
            vertex_buffer: None,
            vao: None,
            frame_count: 0,
            animation: 0.0,
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

        // Create OpenGL objects
        let gl = context.gl();

        let program = Program::from_source(&gl, VS_SRC, FS_SRC, &[]);

        let vertex_buffer = Buffer::new(&gl, glenum::BufferKind::Array as _);
        vertex_buffer.bind();
        vertex_buffer.set_data(&VERTEX_DATA, glenum::DrawMode::Static as _);

        let vao = VertexArray::new(&gl);
        vao.bind();

        vertex_buffer.attrib_enable(
            0,
            2,
            gl::FLOAT,
            false,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            0
        );

        vertex_buffer.attrib_enable(
            1,
            3,
            gl::FLOAT,
            false,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            2 * std::mem::size_of::<f32>() as i32
        );

        check_error!();

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        self.program = Some(program);
        self.vertex_buffer = Some(vertex_buffer);
        self.vao = Some(vao);
        self.initialized = true;
    }

    fn deinit(&mut self, _context: &Context) {
        // Abort if not initialized
        if !self.initialized {
            return;
        }

        // [DEBUG]
        println!("de-initializing renderer");

        // Release OpenGL objects
        self.program = None;
        self.vertex_buffer = None;
        self.vao = None;
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        false
    }

    fn update(&mut self, time_delta: f64) {
        println!("Update {}", time_delta);
        self.animation = self.animation + time_delta;
        self.redraw = true;
    }
}

impl Render for Renderer {
    fn set_viewport(&mut self, _viewport: Vector4<i32>) {
        // We don't care as the viewport is correctly set by the canvas
    }

    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn render(&mut self, context: &Context) {
        // [DEBUG]
        println!("frame #{}", self.frame_count);
        self.frame_count = self.frame_count + 1;

        context.gl().clear(glenum::BufferBit::Color as u32);

        if let Some(ref mut program) = self.program {
            program.use_program();
            let animation = program.uniform_location("animation");
            program.set_float(&animation, self.animation as f32);
        }

        if let Some(ref vao) = self.vao {
            vao.bind();
        }

        context.gl().draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
uniform float animation;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
vec3 rotate(vec3 color_in, float angle) {
    mat3 mat;

    float cosA = cos(angle);
    float sinA = sin(angle);
    mat[0][0] = cosA + (1.0 - cosA) / 3.0;
    mat[0][1] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA;
    mat[0][2] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA;
    mat[1][0] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA;
    mat[1][1] = cosA + 1./3.*(1.0 - cosA);
    mat[1][2] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA;
    mat[2][0] = 1./3. * (1.0 - cosA) - sqrt(1./3.) * sinA;
    mat[2][1] = 1./3. * (1.0 - cosA) + sqrt(1./3.) * sinA;
    mat[2][2] = cosA + 1./3. * (1.0 - cosA);

    vec3 color = mat * color_in;
    return clamp(color, vec3(0.0), vec3(1.0));
}
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = rotate(color, animation);
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
