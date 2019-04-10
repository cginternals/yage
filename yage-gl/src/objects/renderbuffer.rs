use glenum;

use crate::{GL, GlFunctions};

/// Wrapper around an OpenGL renderbuffer.
// TODO!!: incomplete
pub struct Renderbuffer<'a> {
    gl: &'a GL,
    handle: <GL as GlFunctions>::GlRenderbuffer,
}

impl<'a> Renderbuffer<'a> {
    /// Creates a renderbuffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &'a GL) -> Self {
        Self {
            gl,
            handle: gl.create_renderbuffer()
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlRenderbuffer {
        &self.handle
    }

    /// Binds the renderbuffer.
    pub fn bind(&self) {
        self.gl.bind_renderbuffer(glenum::Buffers::Renderbuffer as _, Some(&self.handle));
    }

    /// Unbinds the renderbuffer.
    pub fn unbind(&self) {
        self.gl.bind_renderbuffer(glenum::Buffers::Renderbuffer as _, None);
    }
}

impl<'a> Drop for Renderbuffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_renderbuffer(&self.handle);
    }
}
