use yage::core::{
    Context, GlFunctions,
    glenum,
    check_error,
    Program, Buffer, VertexArray,
    GpuObject, Render
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    program: Option<Program>,
    vertex_buffer: Option<Buffer>,
    vao: Option<VertexArray>,
    frame_count: i32
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
            frame_count: 0
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

impl Render for Renderer {
    fn render(&mut self, context: &Context) {
        // [DEBUG]
        println!("frame #{}", self.frame_count);
        self.frame_count = self.frame_count + 1;

        context.gl().clear(glenum::BufferBit::Color as u32);

        if let Some(ref program) = self.program {
            program.use_program();
        }

        if let Some(ref vao) = self.vao {
            vao.bind();
        }

        context.gl().draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();
    }

    fn needs_redraw(&self) -> bool {
        true
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
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
