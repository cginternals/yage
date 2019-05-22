use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a framebuffer object on the GPU.
///
pub struct Framebuffer {
    pub target: u32,
    handle: Option<<GL as GlFunctions>::GlFramebuffer>
}

impl Framebuffer {
    ///
    /// Create a framebuffer.
    ///
    /// # Returns
    /// A new instance of Framebuffer.
    ///
    pub fn new() -> Self {
        Self {
            target: glenum::Buffers::Framebuffer as _,
            handle: None
        }
    }

    ///
    /// Get framebuffer handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlFramebuffer> {
        self.handle.as_ref()
    }

    ///
    /// Bind framebuffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn bind(&self, context: &Context) {
        context.gl().bind_framebuffer(self.target, self.handle.as_ref());
    }

    ///
    /// Unbind framebuffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn unbind(&self, context: &Context) {
        context.gl().bind_framebuffer(self.target, None);
    }
}

impl GpuObject for Framebuffer {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_framebuffer());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_framebuffer(handle);
            self.handle = None;
        }
    }
}
