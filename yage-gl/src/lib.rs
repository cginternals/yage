#[cfg(not(target_arch = "wasm32"))]
#[path = "gl_native.rs"]
mod gl;

#[cfg(target_arch = "wasm32")]
#[path = "gl_web.rs"]
pub mod gl;

pub use crate::gl::*;

pub mod objects;

#[macro_use]
pub mod utils;
pub use utils::*;

pub use glenum;

use glenum::BufferBit;

/// Trait for all GL functions
/// Associated types are used to support different handles types in native GL and WebGL
/// (integers vs opaque JS types like `WebGLShader`)
pub trait GlFunctions {
    type GlShader;
    type GlProgram;

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn clear(&self, bit: BufferBit);

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader;
    fn shader_source(&self, shader: Self::GlShader, source: &str);
    fn compile_shader(&self, shader: Self::GlShader);
    fn delete_shader(&self, shader: Self::GlShader);

    fn create_program(&self) -> Self::GlProgram;
    fn attach_shader(&self, program: Self::GlProgram, shader: Self::GlShader);
    fn link_program(&self, program: Self::GlProgram);
    fn use_program(&self, program: Option<Self::GlProgram>);
}
