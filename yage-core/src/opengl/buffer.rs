use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a generic buffer on the GPU.
///
pub struct Buffer {
    target: u32,
    handle: Option<<GL as GlFunctions>::GlBuffer>,
}

impl Buffer {
    ///
    /// Create a buffer.
    ///
    /// # Parameters
    /// - `target`: Must be a valid glenum for `glBindBuffer`
    ///
    /// # Returns
    /// A new instance of Buffer.
    ///
    pub fn new(target: u32) -> Self {
        Self {
            target,
            handle: None
        }
    }

    ///
    /// Get buffer handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlBuffer> {
        self.handle.as_ref()
    }

    ///
    /// Bind buffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn bind(&self, context: &Context) {
        context.gl().bind_buffer(self.target, self.handle.as_ref());
    }

    ///
    /// Unbind buffer.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn unbind(&self, context: &Context) {
        context.gl().bind_buffer(self.target, None);
    }

    ///
    /// Set buffer data.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `data`: buffer data
    /// - `usage`: must be a valid glenum for `glBufferData`
    ///
    pub fn set_data<T>(&self, context: &Context, data: &[T], usage: u32) {
        context.gl().buffer_data(self.target, data, usage);
    }

    ///
    /// Update a subset of a buffer's data store.
    ///
    /// Expects the buffer to be bound.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `offset`: offset into the buffer object's data store in bytes
    /// - `data`: buffer data
    ///
    pub fn set_sub_data<T>(&self, context: &Context, offset: isize, data: &[T]) {
        context.gl().buffer_sub_data(self.target, offset, data);
    }
}

impl GpuObject for Buffer {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_buffer());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_buffer(handle);
            self.handle = None;
        }
    }
}
