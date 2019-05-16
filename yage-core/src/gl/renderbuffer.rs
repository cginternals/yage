use glenum;

use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a renderbuffer object on the GPU.
///
pub struct Renderbuffer {
    handle: Option<<GL as GlFunctions>::GlRenderbuffer>,
}

impl Renderbuffer {
    ///
    /// Create a renderbuffer instance.
    ///
    /// # Returns
    /// A new instance of Renderbuffer.
    ///
    pub fn new() -> Self {
        Self {
            handle: None
        }
    }

    ///
    /// Get renderbuffer handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlRenderbuffer> {
        self.handle.as_ref()
    }

    ///
    /// Bind framebuffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn bind(&self, context: &Context) {
        context.gl().bind_renderbuffer(glenum::Buffers::Renderbuffer as _, self.handle.as_ref());
    }

    ///
    /// Unbind framebuffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn unbind(&self, context: &Context) {
        context.gl().bind_renderbuffer(glenum::Buffers::Renderbuffer as _, None);
    }
}

impl GpuObject for Renderbuffer {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_renderbuffer());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_renderbuffer(handle);
            self.handle = None;
        }
    }
}
