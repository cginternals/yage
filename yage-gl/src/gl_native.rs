use std::ops::Deref;

use glenum::*;

#[derive(Default)]
pub struct GL {
}

impl GL {
    pub fn new() -> GL {
        GL {
        }
    }

    /// specify clear values for the color buffers
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    /// clear buffers to preset values
    pub fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
    }

    pub fn create_shader(_kind: glenum::ShaderKind) -> Shader {
        unimplemented!()
    }

    pub fn shader_source(&self, _shader: &Shader, _source: &str) {
        unimplemented!()
    }

    // TODO!!:
    // pub fn compile_shader()
    // pub fn create_program()
    // pub fn attach_shader()
    // pub fn link_shader()
    // pub fn use_program()

    // pub fn gen_buffers()
    // pub fn bind_buffer()
    // pub fn buffer_data()
    // pub fn gen_vertex_arrays()
    // pub fn bind_vertex_array()

    // pub fn get_attrib_location()
    // pub fn vertex_attrib_pointer()
    // pub fn enable_vertex_attrib_array()

    // pub fn draw_arrays()
}

type Reference = u32;

// TODO!!: OpenGL returns an int, WebGL an opaque type... trait + generics?
// -> https://docs.rs/web-sys/0.3.15/web_sys/struct.WebGl2RenderingContext.html#method.create_shader
#[derive(Debug)]
/// an OpenGL shader created with [`GLContext::create_shader`]
pub struct Shader(pub Reference);
impl Deref for Shader {
    type Target = Reference;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
