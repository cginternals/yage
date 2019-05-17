use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

use crate::{
    Context,
    GL, GlFunctions,
    GpuObject,
    Uniform
};

///
/// Represents a shader program on the GPU.
///
pub struct Program {
    handle: Option<<GL as GlFunctions>::GlProgram>,
    uniform_location_cache: HashMap<String, <GL as GlFunctions>::GlUniformLocation>
}

impl Program {
    ///
    /// Create a program instance.
    ///
    /// # Returns
    /// A new instance of Program.
    ///
    pub fn new() -> Self {
        Self {
            handle: None,
            uniform_location_cache: HashMap::new()
        }
    }

    ///
    /// Load program from vertex and fragment shader files and preprocessor definitions.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `vertex_path`: Path to vertex shader file
    /// - `fragment_path`: Path to fragment shader file
    /// - `defines`: Preprocessor definitions
    ///
    pub fn load_shaders(
        &mut self,
        context: &Context,
        vertex_path: &str,
        fragment_path: &str,
        defines: &[String],
    ) {
        // Open shader files
        let mut vertex_shader_file =
            File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut fragment_shader_file =
            File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));

        // Create strings
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();

        // Read vertex shader
        vertex_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");

        // Read fragment shader
        fragment_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        // Create program
        self.set_shaders(context, &vertex_code, &fragment_code, defines)
    }

    ///
    /// Set program from vertex and fragment shader code and preprocessor definitions.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `vertex_code`: Vertex shader code
    /// - `fragment_code`: Fragment shader code
    /// - `defines`: Preprocessor definitions
    ///
    pub fn set_shaders(
        &mut self,
        context: &Context,
        vertex_code: &str,
        fragment_code: &str,
        defines: &[String],
    ) {
        // Get OpenGL functions
        let gl = context.gl();

        // Add preprocessor definitions to source code
        let vertex_code = Self::add_defines(vertex_code, defines);
        let fragment_code = Self::add_defines(fragment_code, defines);

        // Compile vertex shader
        let vertex = gl.create_shader(glenum::ShaderKind::Vertex);
        gl.shader_source(&vertex, &vertex_code);
        gl.compile_shader(&vertex);
        Self::check_compile_errors(context, &vertex, "VERTEX");

        // Compile fragment shader
        let fragment = gl.create_shader(glenum::ShaderKind::Fragment);
        gl.shader_source(&fragment, &&fragment_code);
        gl.compile_shader(&fragment);
        Self::check_compile_errors(context, &fragment, "FRAGMENT");

        // Create shader program
        if let Some(ref handle) = self.handle {
            gl.attach_shader(handle, &vertex);
            gl.attach_shader(handle, &fragment);
            gl.link_program(handle);
            Self::check_link_errors(context, handle);
        }

        // Delete shader objects as they're linked into our program now and no longer necessary
        gl.delete_shader(&vertex);
        gl.delete_shader(&fragment);
    }

    ///
    /// Get program handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlProgram> {
        self.handle.as_ref()
    }

    ///
    /// Bind program.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn use_program(&self, context: &Context) {
        context.gl().use_program(self.handle.as_ref());
    }

    ///
    /// Get uniform location.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `name`: Name of uniform
    ///
    /// # Returns
    /// Uniform location.
    ///
    pub fn uniform_location(
        &mut self,
        context: &Context,
        name: &str,
    ) -> <GL as GlFunctions>::GlUniformLocation {
        // Look up uniform location in cache
        if let Some(loc) = self.uniform_location_cache.get(name) {
            if *loc > -1 {
                #[allow(clippy::clone_on_copy)] // The type is only `Copy` on OpenGL, not WebGL
                return loc.clone();
            }
        }

        // Get uniform location
        // [TODO] Handle case when program is None
        let loc = context.gl().get_uniform_location(&self.handle.unwrap(), &name.to_string());

        // [TODO] How to check null/-1 properly depending on WebGL/OpenGL??
        // if loc == -1 {
        //     // [TODO] Trace!
        //     println!("uniform '{}' unknown for shader {:?}", name, self.id);
        // }

        // Save in cache
        #[allow(clippy::clone_on_copy)] // the type is only `Copy` on OpenGL, not WebGL
        self.uniform_location_cache.insert(name.to_string(), loc.clone());

        // Return uniform location
        loc
    }

    ///
    /// Set uniform value.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `name`: Name of uniform
    /// - `value`: Uniform value
    ///
    pub fn set_uniform<T: Uniform<T>>(&mut self, context: &Context, name: &str, value: T)
    {
        // Get uniform location
        let loc = self.uniform_location(context, name);

        // Set uniform value
        T::set_uniform(context, &loc, value);
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
    fn check_compile_errors(
        context: &Context,
        shader: &<GL as GlFunctions>::GlShader,
        type_: &str,
    ) {
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
            log_type, log_type, type_, info_log
        );
    }

    ///
    /// Check for program linking errors.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `program`: Shader program
    ///
    fn check_link_errors(
        context: &Context,
        program: &<GL as GlFunctions>::GlProgram,
    ) {
        // Get linker status
        let success = context.gl().get_program_parameter(
            program,
            glenum::ShaderParameter::LinkStatus as _
        );

        // Determine error status
        let log_type = if success == 1 { "WARNING" } else { "ERROR" };

        // Get log message
        let info_log = context.gl().get_program_info_log(program);
        if info_log.is_empty() {
            return;
        }

        // Raise error
        // [TODO] Warn
        println!("{}::PROGRAM_LINKING_{} \n{}", log_type, log_type, info_log);
    }
}

impl GpuObject for Program {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_program());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_program(handle);
            self.handle = None;
        }
    }
}
