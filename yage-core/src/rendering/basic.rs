use crate::{
    check_error,
    Context, GlFunctions,
    Program, Shader,
    MeshRenderer, GpuObject, Drawable, Transform,
};

///
/// Basic mesh renderer
///
pub struct BasicMeshRenderer {
    program: Program,
    initialized: bool,
}

impl BasicMeshRenderer {
    ///
    /// Create renderer.
    ///
    /// # Returns
    /// A new instance of BasicMeshRenderer.
    ///
    pub fn new() -> Self {
        Self {
            program: Program::new(),
            initialized: false,
        }
    }
}

impl GpuObject for BasicMeshRenderer {
    fn init(&mut self, context: &Context) {
        // Abort if already initialized
        if self.initialized {
            return;
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

        // De-Initialize OpenGL objects
        self.program.deinit(context);
        self.initialized = false;
    }
}

impl MeshRenderer for BasicMeshRenderer {
    fn draw(&mut self, context: &Context, mesh: &mut Drawable, model_matrix: &Transform) {
        // Bind program and set uniforms
        self.program.use_program(context);
        self.program.set_uniform(context, "modelViewMatrix", model_matrix);
        check_error!();

        // Set rendering states
        context.gl().enable(gl::DEPTH_TEST);
        context.gl().disable(gl::CULL_FACE);

        // Draw geometry
        mesh.draw(context);
        check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
uniform mat4 modelViewMatrix;
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    gl_Position = modelViewMatrix * vec4(position, 1.0);
    v_texcoord = texcoord;
}";

const FS_SRC: &str = "
#version 330 core
precision mediump float;
uniform sampler2D tex;
in vec2 v_texcoord;
out vec4 FragColor;
void main() {
    // FragColor = vec4(v_texcoord.x, v_texcoord.y, 0.0, 1.0);
    FragColor = vec4(texture(tex, v_texcoord).rgb, 1.0);
}";
