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
    type GlBuffer = gl::types::GLuint;
    type GlVertexArray = gl::types::GLuint;

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

    fn create_buffer(&self) -> Self::GlBuffer {
        let mut buf = 0;
        unsafe { gl::GenBuffers(1, &mut buf); }
        buf
    }

    fn bind_buffer(&self, target: u32, buffer: Option<Self::GlBuffer>) {
        unsafe {
            gl::BindBuffer(target, buffer.unwrap_or(0));
        }
    }

    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32) {
        unsafe {
            gl::BufferData(
                target,
                (data.len() * 4) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage
            );
        }
    }

    fn create_vertex_array(&self) -> Self::GlVertexArray {
        let mut vao = 0;
        unsafe { gl::GenVertexArrays(1, &mut vao); }
        vao
    }

    fn bind_vertex_array(&self, vertex_array: Option<Self::GlVertexArray>) {
        unsafe {
            gl::BindVertexArray(vertex_array.unwrap_or(0));
        }
    }

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                data_type,
                normalized as u8,
                stride,
                offset as *const std::ffi::c_void,
            );
        }
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode as u32, first, count);
        }
    }
}
