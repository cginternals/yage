use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::str;

use cgmath::{Matrix4, Vector3, Vector4};

// use log::{warn, trace};

use crate::{GL, GlFunctions};

// TODO!!: copied from gltf-viewer (struct Shader) for debugging, partially adapted
pub struct Program<'a> {
    pub id: <GL as GlFunctions>::GlProgram,
    gl: &'a GL,
    uniform_location_cache: HashMap<&'static str, i32>
}

impl<'a> Program<'a> {
    #[allow(dead_code)]
    pub fn new(gl: &'a GL, vertex_path: &str, fragment_path: &str, defines: &[String]) -> Program<'a> {
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut v_shader_file = File::open(vertex_path).unwrap_or_else(|_| panic!("Failed to open {}", vertex_path));
        let mut f_shader_file = File::open(fragment_path).unwrap_or_else(|_| panic!("Failed to open {}", fragment_path));
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

    // TODO!!: generic GL/ impl Trait?
    pub fn from_source(gl: &'a GL, vertex_code: &str, fragment_code: &str, defines: &[String]) -> Program<'a> {
        let vertex_code = Self::add_defines(vertex_code, defines);
        let fragment_code = Self::add_defines(fragment_code, defines);

        // 2. compile shaders
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
        let id = gl.create_program();
        gl.attach_shader(&id, &vertex);
        gl.attach_shader(&id, &fragment);
        gl.link_program(&id);
        Self::check_link_errors(gl, &id);
        // delete the shaders as they're linked into our program now and no longer necessary
        gl.delete_shader(&vertex);
        gl.delete_shader(&fragment);

        Self {
            id,
            gl,
            uniform_location_cache: HashMap::new()
        }
    }

    fn add_defines(source: &str, defines: &[String]) -> String {
        // insert preprocessor defines after #version if exists
        // (#version must occur before any other statement in the program)
        let defines = defines.iter()
            .map(|define| format!("#define {}", define))
            .collect::<Vec<_>>()
            .join("\n");
        let mut lines: Vec<_> = source.lines().collect();
        if let Some(version_line) = lines.iter().position(|l| l.starts_with("#version")) {
            lines.insert(version_line+1, &defines);
        }
        else {
            lines.insert(0, &defines);
        }
        lines.join("\n")
    }

    /// activate the shader
    pub fn use_program(&self) {
        self.gl.use_program(Some(&self.id))
    }

    // uniform setting functions

    pub fn set_bool(&self, location: i32, value: bool) {
        self.gl.uniform_1i(&location, value as i32);
    }
    pub fn set_int(&self, location: i32, value: i32) {
        self.gl.uniform_1i(&location, value);
    }
    pub fn set_float(&self, location: i32, value: f32) {
        self.gl.uniform_1f(&location, value);
    }
    pub fn set_vector3(&self, location: i32, value: &Vector3<f32>) {
        self.gl.uniform_3fv(&location, value.as_ref());
    }
    pub fn set_vector4(&self, location: i32, value: &Vector4<f32>) {
        self.gl.uniform_4fv(&location, value.as_ref());
    }
    pub fn set_vec2(&self, location: i32, x: f32, y: f32) {
        self.gl.uniform_2f(&location, x, y);
    }
    pub fn set_vec3(&self, location: i32, x: f32, y: f32, z: f32) {
        self.gl.uniform_3f(&location, x, y, z);
    }
    pub fn set_mat4(&self, location: i32, mat: &Matrix4<f32>) {
        self.gl.uniform_matrix_4fv(&location, mat.as_ref());
    }

    /// get uniform location with caching
    pub fn uniform_location(&mut self, name: &'static str) -> i32 {
        if let Some(loc) = self.uniform_location_cache.get(name) {
            return *loc;
        }

        let loc = self.gl.get_uniform_location(&self.id, name);
        if loc == -1 {
            // TODO!: trace!
            println!("uniform '{}' unknown for shader {:?}", name, self.id);
        }
        self.uniform_location_cache.insert(name, loc);
        loc
    } 

    /// utility function for checking shader compilation errors.
    fn check_compile_errors(gl: &GL, shader: &<GL as GlFunctions>::GlShader, type_: &str) {
        let success = gl.get_shader_parameter(shader, glenum::ShaderParameter::CompileStatus as _);
        let log_type = if success == 1 { "WARNING" } else { "ERROR" };
        let info_log = gl.get_shader_info_log(shader);
        if info_log.is_empty() { return }
        panic!("{}::SHADER_COMPILATION_{} of type: {}\n{}", 
            log_type, log_type, type_, info_log);
    }

    /// utility function for checking program linking errors.
    fn check_link_errors(gl: &GL, program: &<GL as GlFunctions>::GlProgram) {
        let success = gl.get_program_parameter(program, glenum::ShaderParameter::LinkStatus as _);
        let log_type = if success == 1 { "WARNING" } else { "ERROR" };
        let info_log = gl.get_program_info_log(program);
        if info_log.is_empty() { return }
        // TODO!: warn!
        println!("{}::PROGRAM_LINKING_{} \n{}",
                    log_type, log_type, info_log);
    }
}
