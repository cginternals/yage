use crate::{
    check_error,
    Context, GlFunctions,
    Program, Shader, Geometry, Transform, Camera,
    MeshRenderer, GpuObject, Drawable,
    opengl::glenum,
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
            let mut vertex_shader = Shader::new(glenum::VERTEX_SHADER);
            vertex_shader.set_code(context, VS_SRC, &[]);

            // Load fragment shader
            let mut fragment_shader = Shader::new(glenum::FRAGMENT_SHADER);
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
    fn draw(&mut self,
        context: &Context,
        camera: &Camera,
        geometry: &mut Geometry,
        transform: &Transform
    ) {
        // Calculate matrices
        let view_matrix = camera.view_matrix();
        let projection_matrix = camera.projection_matrix();
        let view_projection_matrix = camera.view_projection_matrix();
        let model_matrix = transform.transform();
        let model_view_matrix = view_matrix * model_matrix;
        let model_view_projection_matrix = view_projection_matrix * model_matrix;

        // Bind program and set uniforms
        self.program.use_program(context);
        self.program.set_uniform(context, "viewMatrix", &view_matrix);
        self.program.set_uniform(context, "projectionMatrix", &projection_matrix);
        self.program.set_uniform(context, "viewProjectionMatrix", &view_projection_matrix);
        self.program.set_uniform(context, "modelMatrix", &model_matrix);
        self.program.set_uniform(context, "modelViewMatrix", &model_view_matrix);
        self.program.set_uniform(context, "modelViewProjectionMatrix", &model_view_projection_matrix);
        check_error!();

        // Set rendering states
        context.gl().enable(glenum::DEPTH_TEST);
        context.gl().disable(glenum::CULL_FACE);

        // Draw geometry
        geometry.draw(context);
        check_error!();
    }
}

const VS_SRC: &str = "
#version 330 core
precision mediump float;
uniform mat4 modelViewProjectionMatrix;
layout (location = 0) in vec3 position;
layout (location = 1) in vec2 texcoord;
out vec2 v_texcoord;
void main() {
    gl_Position = modelViewProjectionMatrix * vec4(position, 1.0);
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
