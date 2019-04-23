use std::rc::Rc;

use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL frame buffer.
// TODO!!: incomplete
pub struct Framebuffer {
    gl: Rc<GL>,
    /// Target for use in `glBindFrameBuffer`
    pub target: u32,
    handle: <GL as GlFunctions>::GlFramebuffer,
}

impl Framebuffer {
    /// Creates a framebuffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &Rc<GL>) -> Self {
        Self {
            gl: gl.clone(),
            target: glenum::Buffers::Framebuffer as _,
            handle: gl.create_framebuffer()
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlFramebuffer {
        &self.handle
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

impl Drop for Framebuffer {
    fn drop(&mut self) {
        self.gl.delete_framebuffer(&self.handle);
    }
}
