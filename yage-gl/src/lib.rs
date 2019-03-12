#[cfg(not(target_arch = "wasm32"))]
#[path = "gl.rs"]
mod gl;

#[cfg(target_arch = "wasm32")]
#[path = "webgl.rs"]
pub mod gl;

pub use crate::gl::*;
pub use glenum;
