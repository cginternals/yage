mod glfunctions;
pub use glfunctions::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "gl_native.rs"]
mod gl_impl;

#[cfg(target_arch = "wasm32")]
#[path = "gl_web.rs"]
mod gl_impl;

pub use gl_impl::*;

#[cfg(not(target_arch = "wasm32"))]
#[path = "texture_loader_native.rs"]
mod texture_loader;

#[cfg(target_arch = "wasm32")]
#[path = "texture_loader_web.rs"]
mod texture_loader;

pub use texture_loader::*;

mod buffer;
pub use buffer::*;

mod vertex_array;
pub use vertex_array::*;

mod program;
pub use program::*;

mod shader;
pub use shader::*;

mod uniform;
pub use uniform::*;

mod texture;
pub use texture::*;

mod framebuffer;
pub use framebuffer::*;

mod renderbuffer;
pub use renderbuffer::*;

#[macro_use]
pub mod utils;
pub use utils::*;
