use std::rc::Rc;

use glenum;

use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL renderbuffer.
// TODO!!: incomplete
pub struct Renderbuffer {
    gl: Rc<GL>,
    handle: <GL as GlFunctions>::GlRenderbuffer,
}

impl Renderbuffer {
    /// Creates a renderbuffer.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &Rc<GL>) -> Self {
        Self {
            gl: gl.clone(),
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

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        self.gl.delete_renderbuffer(&self.handle);
    }
}
