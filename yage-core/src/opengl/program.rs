use std::vec::Vec;
use std::collections::HashMap;

use crate::{
    Context,
    GL, GlFunctions,
    GpuObject,
    Shader, Uniform
};

///
/// Represents a shader program on the GPU.
///
pub struct Program {
    handle: Option<<GL as GlFunctions>::GlProgram>,
    shaders: Vec<Shader>,
    linked: bool,
    uniform_location_cache: HashMap<String, <GL as GlFunctions>::GlUniformLocation>
}

impl Program {
    ///
    /// Create a shader program.
    ///
    /// # Returns
    /// A new instance of Program.
    ///
    pub fn new() -> Self {
        Self {
            handle: None,
            shaders: Vec::new(),
            linked: false,
            uniform_location_cache: HashMap::new()
        }
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
    /// Attach shader.
    ///
    /// # Parameters
    /// - `shader`: Shader to attach to the program
    ///
    pub fn attach(&mut self, shader: Shader) {
        // Move shader into list
        self.shaders.push(shader);

        // Reset status
        self.linked = false;
    }

    ///
    /// Check if program is linked.
    ///
    /// # Returns
    /// true if linked, else false.
    ///
    pub fn is_linked(&self) -> bool {
        self.linked
    }

    ///
    /// Link shader program.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn link(&mut self, context: &Context) {
        // Abort if already linked
        if self.linked {
            return;
        }

        // (Re-)Create program
        self.handle = Some(context.gl().create_program());

        // Get program
        if let Some(ref program) = self.handle {
            // Attach shaders to program
            for shader in &self.shaders {
                if let Some(ref handle) = shader.handle() {
                    context.gl().attach_shader(program, handle);
                }
            }

            // Link program
            context.gl().link_program(program);
            self.check_link_errors(context);

            // Set link status
            self.linked = true;
        }
    }

    ///
    /// Bind program.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn use_program(&mut self, context: &Context) {
        // Make sure program is linked
        self.link(context);

        // Activate shader program
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
        // Make sure program is linked
        self.link(context);

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
    /// Check for program linking errors.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `program`: Shader program
    ///
    fn check_link_errors(&self, context: &Context) {
        // Get program
        if let Some(ref program) = self.handle {
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
}

impl GpuObject for Program {
    fn init(&mut self, _context: &Context) {
        // We don't initialize the program here, we do it whenever something is changed
    }

    fn deinit(&mut self, context: &Context) {
        // Get program
        if let Some(ref handle) = self.handle {
            // Destroy program
            context.gl().delete_program(handle);

            // Reset data
            self.handle = None;
            self.linked = false;
        }
    }
}
