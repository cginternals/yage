///
/// Description of a vertex attribute.
///
#[derive(Copy, Clone)]
pub struct VertexAttribute {
    buffer: usize, // Index of buffer that is used
    base_offset: usize, // Offset into the buffer (in bytes)
    relative_offset: usize, // Relative offset of this attribute data (in bytes)
    stride: usize, // Number of bytes between two adjacent elements in the buffer (in bytes)
    data_type: u32, // Data type (e.g., gl::GL_FLOAT)
    components: usize, // Number of components
    normalize: bool // Shall the data be normalized?
}

impl VertexAttribute {
    ///
    /// Create vertex attribute.
    ///
    /// # Parameters
    /// - `buffer`: Index of buffer that is used
    /// - `base_offset`: Offset into the buffer (in bytes)
    /// - `relative_offset`: Relative offset of this attribute data (in bytes)
    /// - `stride`: Number of bytes between two adjacent elements in the buffer (in bytes)
    /// - `data_type`: Data type (e.g., gl::GL_FLOAT)
    /// - `components`: Number of components
    /// - `normalize`: Shall the data be normalized?
    ///
    /// # Returns
    /// A new instance of VertexAttribute.
    ///
    pub fn new(
        buffer: usize,
        base_offset: usize,
        relative_offset: usize,
        stride: usize,
        data_type: u32,
        components: usize,
        normalize: bool
    ) -> Self {
        Self {
            buffer,
            base_offset,
            relative_offset,
            stride,
            data_type,
            components,
            normalize
        }
    }

    ///
    /// Get buffer.
    ///
    /// # Returns
    /// Index of buffer that is used
    ///
    pub fn buffer(&self) -> usize {
        self.buffer
    }

    ///
    /// Get base offset.
    ///
    /// # Returns
    /// Offset into the buffer (in bytes)
    ///
    pub fn base_offset(&self) -> usize {
        self.base_offset
    }

    ///
    /// Get relative offset.
    ///
    /// # Returns
    /// Relative offset of this attribute data (in bytes)
    ///
    pub fn relative_offset(&self) -> usize {
        self.relative_offset
    }

    ///
    /// Get stride.
    ///
    /// # Returns
    /// Number of bytes between two adjacent elements in the buffer (in bytes)
    ///
    pub fn stride(&self) -> usize {
        self.stride
    }

    ///
    /// Get data type.
    ///
    /// # Returns
    /// Data type (e.g., gl::GL_FLOAT)
    ///
    pub fn data_type(&self) -> u32 {
        self.data_type
    }

    ///
    /// Get number of components.
    ///
    /// # Returns
    /// Number of components
    ///
    pub fn components(&self) -> usize {
        self.components
    }

    ///
    /// Check if data is normalized.
    ///
    /// # Returns
    /// Shall the data be normalized?
    ///
    pub fn normalize(&self) -> bool {
        self.normalize
    }
}
