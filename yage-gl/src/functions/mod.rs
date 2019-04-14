mod glfunctions;
pub use glfunctions::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "gl_native.rs"]
mod gl_impl;

#[cfg(target_arch = "wasm32")]
#[path = "gl_web.rs"]
mod gl_impl;

pub use gl_impl::*;
