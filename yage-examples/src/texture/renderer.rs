use yage_core::{
    Context, GlFunctions,
    glenum, cgmath,
    check_error,
    Program, Shader, Buffer, VertexArray,
    GpuObject, Render, Update,
    Texture, TextureLoader
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    program: Program,
    vertex_buffer: Buffer,
    texture: Texture,
    vao: VertexArray,
    frame_count: i32,
    animation: f64,
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
            program: Program::new(),
            vertex_buffer: Buffer::new(glenum::BufferKind::Array as _),
            texture: Texture::new(gl::TEXTURE_2D),
            vao: VertexArray::new(),
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

        // Initialize OpenGL objects
        self.program.init(context);
        self.texture.init(context);
        self.vao.init(context);
        self.vertex_buffer.init(context);

        // Create OpenGL objects
        let gl = context.gl();

        check_error!();

        // Load texture
        TextureLoader::load(context, &mut self.texture, "data/duck.jpg");
        check_error!();

        // Load shader programs
        let mut vertex_shader = Shader::new(glenum::ShaderKind::Vertex);
        vertex_shader.set_code(context, VS_SRC, &[]);

        let mut fragment_shader = Shader::new(glenum::ShaderKind::Fragment);
        fragment_shader.set_code(context, FS_SRC, &[]);

        self.program.attach(vertex_shader);
        self.program.attach(fragment_shader);

        self.vertex_buffer.bind(context);
        self.vertex_buffer.set_data(context, &VERTEX_DATA, glenum::DrawMode::Static as _);

        self.vao.bind(context);

        self.vertex_buffer.attrib_enable(
            context,
            0,
            2,
            gl::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            0
        );

        self.vertex_buffer.attrib_enable(
            context,
            1,
            2,
            gl::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            2 * std::mem::size_of::<f32>() as gl::types::GLsizei
        );

        check_error!();

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

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
        self.program.deinit(context);
        self.texture.deinit(context);
        self.vao.deinit(context);
        self.vertex_buffer.deinit(context);
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        false
    }

    fn update(&mut self, time_delta: f64) {
        self.animation = self.animation + time_delta;
        //println!("Update {}", self.animation);
        if self.animation > 1.0 {
            self.animation -= 1.0;
        }
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

        context.gl().clear(glenum::BufferBit::Color as u32);

        self.texture.bind_active(context, 0);

        self.program.use_program(context);
        check_error!();

        self.program.set_uniform(context, "tex", 0);
        self.program.set_uniform(context, "color", (0.4, 0.8, 0.4));
        self.program.set_uniform(context, "animation", self.animation as f32);
        check_error!();

        self.vao.bind(context);

        context.gl().draw_arrays(gl::TRIANGLE_STRIP, 0, 4);
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
