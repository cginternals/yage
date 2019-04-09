use glenum;

use crate::{GL, GlFunctions};

/// Wrapper around an OpenGL frame buffer.
pub struct Framebuffer<'a> {
    gl: &'a GL,
    /// Target for use in `glBindFrameBuffer`
    pub target: u32,
    handle: <GL as GlFunctions>::GlFramebuffer,
}

impl<'a> Framebuffer<'a> {
    /// Creates a framebuffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &'a GL) -> Self {
        Self {
            gl,
            target: glenum::Buffers::Framebuffer as _,
            handle: gl.create_framebuffer()
        }
    }

    /// Binds the framebuffer.
    pub fn bind(&self) {
        self.gl.bind_framebuffer(self.target, Some(&self.handle));
    }

    /// Unbinds the framebuffer.
    pub fn unbind(&self) {
        self.gl.bind_framebuffer(self.target, None);
    }
}

impl<'a> Drop for Framebuffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(&self.handle);
    }
}
