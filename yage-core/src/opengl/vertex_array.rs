use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a vertex array on the GPU.
///
pub struct VertexArray {
    handle: Option<<GL as GlFunctions>::GlVertexArray>
}

impl VertexArray {
    ///
    /// Create a vertex array.
    ///
    /// # Returns
    /// A new instance of VertexArray.
    ///
    pub fn new() -> Self {
        Self {
            handle: None
        }
    }

    ///
    /// Get vertex array handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlVertexArray> {
        self.handle.as_ref()
    }

    ///
    /// Bind vertex array.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn bind(&self, context: &Context) {
        context.gl().bind_vertex_array(self.handle.as_ref());
    }

    ///
    /// Unbind vertex array.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn unbind(&self, context: &Context) {
        context.gl().bind_vertex_array(None);
    }
}

impl GpuObject for VertexArray {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_vertex_array());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_vertex_array(handle);
            self.handle = None;
        }
    }
}
