use cgmath::Vector4;

extern crate image;

use yage::core::{
    Context, GlFunctions,
    glenum,
    check_error,
    Program, Buffer, VertexArray,
    GpuObject, Render, Update,
    Texture
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    program: Option<Program>,
    vertex_buffer: Option<Buffer>,
    texture: Option<Texture>,
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
            texture: None,
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
        //println!("initializing renderer");

        // Create OpenGL objects
        let gl = context.gl();

        check_error!();

        // Create texture
        let mut texture = Texture::new(&gl, gl::TEXTURE_2D);
        check_error!();

        // Load image
        let image_file = image::open("data/duck.jpg");
        match image_file {
            Err(err) => panic!("Could not load image: {}", err),
            Ok(img) => {
                let data = img.raw_pixels();

                texture.init(context);
                check_error!();

                gl.active_texture(0);
                check_error!();

                texture.bind();
                check_error!();

                texture.set_image_2d(
                    0,
                    gl::RGB as _,
                    1024,
                    768,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    Some(&data)
                );
                check_error!();

                texture.generate_mipmap();
                check_error!();
            }
        }

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
            4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            0
        );

        vertex_buffer.attrib_enable(
            1,
            2,
            gl::FLOAT,
            false,
            4 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            2 * std::mem::size_of::<f32>() as gl::types::GLsizei
        );

        check_error!();

        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        self.program = Some(program);
        self.vertex_buffer = Some(vertex_buffer);
        self.vao = Some(vao);
        self.texture = Some(texture);
        self.initialized = true;
    }

    fn deinit(&mut self, _context: &Context) {
        // Abort if not initialized
        if !self.initialized {
            return;
        }

        // [DEBUG]
        //println!("de-initializing renderer");

        // Release OpenGL objects
        self.program = None;
        self.vertex_buffer = None;
        self.vao = None;
        self.texture = None;
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        false
    }

    fn update(&mut self, time_delta: f64) {
        //println!("Update {}", time_delta);
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
        //println!("frame #{}", self.frame_count);
        self.frame_count = self.frame_count + 1;

        context.gl().clear(glenum::BufferBit::Color as u32);

        /*
        if let Some(ref texture) = self.texture {
            context.gl().active_texture(0);
            texture.bind();
        }
        */

        if let Some(ref mut program) = self.program {
            program.use_program();
            let animation = program.uniform_location("animation");
            program.set_float(&animation, self.animation as f32);
            let texture1 = program.uniform_location("texture1");
            program.set_int(&texture1, 0);
        }

        if let Some(ref vao) = self.vao {
            vao.bind();
        }

        context.gl().draw_arrays(gl::TRIANGLE_STRIP, 0, 4);

        // check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
uniform float animation;
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
uniform sampler2D texture1;
in vec2 v_texcoord;
out vec4 FragColor;
void main() {
    // FragColor = vec4(v_texcoord.x, v_texcoord.y, 0.0, 1.0);
    FragColor = vec4(texture(texture1, v_texcoord).rgb, 1.0);
}";

#[rustfmt::skip]
static VERTEX_DATA: [f32; 16] = [
    -0.5,  0.5,  0.0,  1.0,
    -0.5, -0.5,  0.0,  0.0,
     0.5,  0.5,  1.0,  1.0,
     0.5, -0.5,  1.0,  0.0,
];
