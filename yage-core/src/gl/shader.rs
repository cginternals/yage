use std::fs::File;
use std::io::Read;

use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a shader on the GPU.
///
pub struct Shader {
    shader_type: glenum::ShaderKind,
    handle: Option<<GL as GlFunctions>::GlShader>
}

impl Shader {
    ///
    /// Create a shader instance.
    ///
    /// # Parameters
    /// - `shader_type`: Type of shader
    ///
    /// # Returns
    /// A new instance of Shader.
    ///
    pub fn new(shader_type: glenum::ShaderKind) -> Self {
        Self {
            shader_type,
            handle: None
        }
    }

    ///
    /// Load shader from file.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `path`: Path to source file
    /// - `defines`: Preprocessor definitions
    ///
    pub fn load(&mut self, context: &Context, path: &str, defines: &[String]) {
        // Open shader file
        let mut file = File::open(path).unwrap_or_else(
            |_| panic!("Failed to open {}", path)
        );

        // Read shader code
        let mut code = String::new();
        file.read_to_string(&mut code).expect("Failed to read shader");

        // Compile shader
        self.set_code(context, &code, defines)
    }

    ///
    /// Create shader from shader code.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `code`: Shader code
    /// - `defines`: Preprocessor definitions
    ///
    pub fn set_code(&mut self, context: &Context, code: &str, defines: &[String]) {
        // (Re-)Create shader
        self.handle = Some(context.gl().create_shader(self.shader_type));

        // Get shader
        if let Some(ref shader) = self.handle {
            // Add preprocessor definitions to source code
            let code = Self::add_defines(code, defines);

            // Compile shader
            context.gl().shader_source(shader, &code);
            context.gl().compile_shader(shader);

            // Check for compile errors
            self.check_compile_errors(context);
        }
    }

    ///
    /// Get shader handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlProgram> {
        self.handle.as_ref()
    }

    ///
    /// Add preprocessor definitions to source code.
    ///
    /// # Parameters
    /// - `source`: Original source code
    /// - `defines`: Preprocessor definitions
    ///
    /// # Returns
    /// New source code.
    ///
    fn add_defines(source: &str, defines: &[String]) -> String {
        // Insert preprocessor defines after #version if exists
        // (#version must occur before any other statement in the program)
        let defines = defines
            .iter()
            .map(|define| format!("#define {}", define))
            .collect::<Vec<_>>()
            .join("\n");
        let mut lines: Vec<_> = source.lines().collect();
        if let Some(version_line) = lines.iter().position(|l| l.starts_with("#version")) {
            lines.insert(version_line + 1, &defines);
        } else {
            lines.insert(0, &defines);
        }
        lines.join("\n")
    }

    ///
    /// Check for shader compilation errors.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `shader`: OpenGL Shader
    /// - `type_`: Shader type
    ///
    fn check_compile_errors(&self, context: &Context) {
        // Get shader
        if let Some(ref shader) = self.handle {
            // Determine shader type
            let shader_type = match self.shader_type {
                glenum::ShaderKind::Vertex => "VERTEX",
                glenum::ShaderKind::Fragment => "FRAGMENT"
            };

            // Get compile status
            let success = context.gl().get_shader_parameter(
                shader,
                glenum::ShaderParameter::CompileStatus as _
            );

            // Determine error status
            let log_type = if success == 1 { "WARNING" } else { "ERROR" };

            // Get log message
            let info_log = context.gl().get_shader_info_log(shader);
            if info_log.is_empty() {
                return;
            }

            // Raise error
            // [TODO] Warn
            panic!(
                "{}::SHADER_COMPILATION_{} of type: {}\n{}",
                log_type, log_type, shader_type, info_log
            );
        }
    }
}

impl GpuObject for Shader {
    fn init(&mut self, _context: &Context) {
        // We don't initialize the shader object here, we do it whenever something is changed
    }

    fn deinit(&mut self, context: &Context) {
        // Get shader
        if let Some(ref handle) = self.handle {
            // Destroy shader
            context.gl().delete_shader(handle);
            self.handle = None;
        }
    }
}
