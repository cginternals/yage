use glenum::*;

use super::GlFunctions;

#[derive(Default)]
pub struct GL {}

#[allow(dead_code)]
impl GL {
    pub fn new() -> GL {
        GL {}
    }
}

impl GlFunctions for GL {
    type GlShader = gl::types::GLuint;
    type GlProgram = gl::types::GLuint;

    /// specify clear values for the color buffers
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    /// clear buffers to preset values
    fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
    }

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader {
        unsafe { gl::CreateShader(kind as _) }
    }

    fn shader_source(&self, shader: Self::GlShader, source: &str) {
        unsafe {
            gl::ShaderSource(
                shader, 
                1,
                &(source.as_ptr() as *const i8), 
                &(source.len() as i32)
            );
        }
    }

    fn compile_shader(&self, shader: Self::GlShader) {
        unsafe {
            gl::CompileShader(shader);
        }
    }

    fn delete_shader(&self, shader: Self::GlShader) {
        unsafe {
            gl::DeleteShader(shader);
        }
    }

    fn create_program(&self) -> Self::GlProgram {
        unsafe {
            gl::CreateProgram()
        }
    }

    fn attach_shader(&self, program: Self::GlProgram, shader: Self::GlShader) {
        unsafe {
            gl::AttachShader(program, shader)
        }
    }

    fn link_program(&self, program: Self::GlProgram) {
        unsafe {
            gl::LinkProgram(program)
        }
    }

    fn use_program(&self, program: Option<Self::GlProgram>) {
        unsafe {
            gl::UseProgram(program.unwrap_or(0));
        }
    }

    // TODO!!:

    // fn gen_buffers()
    // fn bind_buffer()
    // fn buffer_data()
    // fn gen_vertex_arrays()
    // fn bind_vertex_array()

    // fn get_attrib_location()
    // fn vertex_attrib_pointer()
    // fn enable_vertex_attrib_array()

    // fn draw_arrays()
}


