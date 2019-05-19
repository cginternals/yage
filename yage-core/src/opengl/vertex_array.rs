use crate::{
    Context,
    GL, GlFunctions,
    GpuObject, Buffer
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

    ///
    /// Enable vertex attribute.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `index`: Index of the vertex attribute
    ///
    pub fn enable_attribute(&self, context: &Context, index: u32) {
        context.gl().enable_vertex_attrib_array(index);
    }

    ///
    /// Disable vertex attribute.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `index`: Index of the vertex attribute
    ///
    pub fn disable_attribute(&self, context: &Context, index: u32) {
        context.gl().disable_vertex_attrib_array(index);
    }

    ///
    /// Define vertex attribute.
    ///
    /// Expects the vertex array to be bound.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `index` - Index of the vertex attribute that is to be setup and enabled.
    /// - `buffer` - Buffer that contains the vertex data.
    /// - `size` - Number of components per vertex attribute.
    /// - `type` - Data type of each component in the array.
    /// - `normalized` - Whether integer data values should be normalized when being casted to a float.
    /// - `stride` - Offset in bytes between the beginning of consecutive vertex attributes.
    /// - `offset` - Offset in bytes of the first component in the vertex attribute array.
    ///
    pub fn set_attribute(
        &self,
        context: &Context,
        index: u32,
        buffer: &Buffer,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        buffer.bind(context);
        context.gl().vertex_attrib_pointer(index, size, data_type, normalized, stride, offset);
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
