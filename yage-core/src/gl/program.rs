use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;
use std::rc::Rc;

use cgmath::{Matrix4, Vector3, Vector4};

// use log::{warn, trace};

use crate::{GlFunctions, GL};

/// Wrapper object for OpenGL Programs.
pub struct Program {
    gl: Rc<GL>,
    handle: <GL as GlFunctions>::GlProgram,
    uniform_location_cache: HashMap<&'static str, <GL as GlFunctions>::GlUniformLocation>,
}

impl Program {
    /// Creates program from vertex and fragment shader paths and preprocessor defines.
    pub fn from_file(
        gl: &Rc<GL>,
        vertex_path: &str,
        fragment_path: &str,
        defines: &[String],
    ) -> Program {
        // retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file =
            File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path)
            .unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
        let mut vertex_code = String::new();
        let mut fragment_code = String::new();
        v_shader_file
            .read_to_string(&mut vertex_code)
            .expect("Failed to read vertex shader");
        f_shader_file
            .read_to_string(&mut fragment_code)
            .expect("Failed to read fragment shader");

        Self::from_source(gl, &vertex_code, &fragment_code, defines)
    }

    /// Creates program from vertex and fragment shader sources and preprocessor defines.
    pub fn from_source(
        gl: &Rc<GL>,
        vertex_code: &str,
        fragment_code: &str,
        defines: &[String],
    ) -> Program {
        let vertex_code = Self::add_defines(vertex_code, defines);
        let fragment_code = Self::add_defines(fragment_code, defines);

        // compile shaders
        // vertex shader
        let vertex = gl.create_shader(glenum::ShaderKind::Vertex);
        gl.shader_source(&vertex, &vertex_code);
        gl.compile_shader(&vertex);
        Self::check_compile_errors(gl, &vertex, "VERTEX");
        // fragment Shader
        let fragment = gl.create_shader(glenum::ShaderKind::Fragment);
        gl.shader_source(&fragment, &&fragment_code);
        gl.compile_shader(&fragment);
        Self::check_compile_errors(gl, &fragment, "FRAGMENT");
        // shader Program
        let handle = gl.create_program();
        gl.attach_shader(&handle, &vertex);
        gl.attach_shader(&handle, &fragment);
        gl.link_program(&handle);
        Self::check_link_errors(gl, &handle);
        // delete the shaders as they're linked into our program now and no longer necessary
        gl.delete_shader(&vertex);
        gl.delete_shader(&fragment);

        Self {
            handle,
            gl: gl.clone(),
            uniform_location_cache: HashMap::new(),
        }
    }

    /// Util to add preprocessor defines.
    fn add_defines(source: &str, defines: &[String]) -> String {
        // insert preprocessor defines after #version if exists
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

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlProgram {
        &self.handle
    }

    /// activate the shader
    pub fn use_program(&self) {
        self.gl.use_program(Some(&self.handle))
    }

    // uniform setting functions

    pub fn set_bool(&self, location: &<GL as GlFunctions>::GlUniformLocation, value: bool) {
        self.gl.uniform_1i(location, value as i32);
    }
    pub fn set_int(&self, location: &<GL as GlFunctions>::GlUniformLocation, value: i32) {
        self.gl.uniform_1i(location, value);
    }
    pub fn set_float(&self, location: &<GL as GlFunctions>::GlUniformLocation, value: f32) {
        self.gl.uniform_1f(location, value);
    }
    pub fn set_vector3(
        &self,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector3<f32>,
    ) {
        self.gl.uniform_3fv(location, value.as_ref());
    }
    pub fn set_vector4(
        &self,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector4<f32>,
    ) {
        self.gl.uniform_4fv(location, value.as_ref());
    }
    pub fn set_vec2(&self, location: &<GL as GlFunctions>::GlUniformLocation, x: f32, y: f32) {
        self.gl.uniform_2f(location, x, y);
    }
    pub fn set_vec3(
        &self,
        location: &<GL as GlFunctions>::GlUniformLocation,
        x: f32,
        y: f32,
        z: f32,
    ) {
        self.gl.uniform_3f(location, x, y, z);
    }
    pub fn set_mat4(&self, location: &<GL as GlFunctions>::GlUniformLocation, mat: &Matrix4<f32>) {
        self.gl.uniform_matrix_4fv(location, mat.as_ref());
    }

    /// get uniform location with caching
    pub fn uniform_location(
        &mut self,
        name: &'static str,
    ) -> <GL as GlFunctions>::GlUniformLocation {
        if let Some(loc) = self.uniform_location_cache.get(name) {
            #[allow(clippy::clone_on_copy)] // the type is only `Copy` on OpenGL, not WebGL
            return loc.clone();
        }

        let loc = self.gl.get_uniform_location(&self.handle, name);
        // TODO!!: how to check null/-1 properly depending on WebGL/OpenGL??
        // if loc == -1 {
        //     // TODO!: trace!
        //     println!("uniform '{}' unknown for shader {:?}", name, self.id);
        // }
        #[allow(clippy::clone_on_copy)] // the type is only `Copy` on OpenGL, not WebGL
        self.uniform_location_cache.insert(name, loc.clone());
        loc
    }

    /// utility function for checking shader compilation errors.
    fn check_compile_errors(gl: &GL, shader: &<GL as GlFunctions>::GlShader, type_: &str) {
        let success = gl.get_shader_parameter(shader, glenum::ShaderParameter::CompileStatus as _);
        let log_type = if success == 1 { "WARNING" } else { "ERROR" };
        let info_log = gl.get_shader_info_log(shader);
        if info_log.is_empty() {
            return;
        }
        panic!(
            "{}::SHADER_COMPILATION_{} of type: {}\n{}",
            log_type, log_type, type_, info_log
        );
    }

    /// utility function for checking program linking errors.
    fn check_link_errors(gl: &GL, program: &<GL as GlFunctions>::GlProgram) {
        let success = gl.get_program_parameter(program, glenum::ShaderParameter::LinkStatus as _);
        let log_type = if success == 1 { "WARNING" } else { "ERROR" };
        let info_log = gl.get_program_info_log(program);
        if info_log.is_empty() {
            return;
        }
        // TODO!: warn!
        println!("{}::PROGRAM_LINKING_{} \n{}", log_type, log_type, info_log);
    }
}
