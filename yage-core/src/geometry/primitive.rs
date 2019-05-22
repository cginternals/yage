use std::collections::HashMap;

use crate::{
    Context, GlFunctions,
    GpuObject,
    VertexArray, Buffer,
    ResourceManager, VertexAttribute,
};

///
/// Represents a renderable geometric primitive.
///
pub struct Primitive {
    material: usize, // Material ID
    render_mode: u32, // Render mode (e.g., GL_TRIANGLES)
    count: usize, // Number of elements
    index_buffer: Option<usize>, // Index buffer ID, or None
    index_buffer_type: u32, // Data type (e.g., GL_UNSIGNED_INT)
    attributes: HashMap<usize, usize>, // Attribute bindings
    vao: Option<VertexArray> // Vertex array object
}

impl Primitive {
    ///
    /// Create primitive.
    ///
    /// # Parameters
    /// - `material`: Material ID
    /// - `render_mode`: Render mode (e.g., GL_TRIANGLES)
    /// - `count`: Number of elements
    /// - `index_buffer`: Index buffer ID, or None
    /// - `index_buffer_type`: Data type (e.g., GL_UNSIGNED_INT)
    /// - `attributes`: Attribute bindings
    ///
    /// # Returns
    /// A new instance of Primitive.
    ///
    pub fn new(
        material: usize,
        render_mode: u32,
        count: usize,
        index_buffer: Option<usize>,
        index_buffer_type: u32,
        attributes: &[(usize, usize)],
    ) -> Self {
        Self {
            material,
            render_mode,
            count,
            index_buffer,
            index_buffer_type,
            attributes: attributes.iter().cloned().collect(),
            vao: None,
        }
    }

    ///
    /// Get material.
    ///
    /// # Returns
    /// Material ID.
    ///
    pub fn material(&self) -> usize {
        self.material
    }

    ///
    /// Get render mode.
    ///
    /// # Returns
    /// Render mode (e.g., GL_TRIANGLES).
    ///
    pub fn render_mode(&self) -> u32 {
        self.render_mode
    }

    ///
    /// Get number of elements to draw.
    ///
    /// # Returns
    /// Number of elements.
    ///
    pub fn count(&self) -> usize {
        self.count
    }

    ///
    /// Get index buffer.
    ///
    /// # Returns
    /// Index buffer ID, or None.
    ///
    pub fn index_buffer(&self) -> Option<usize> {
        self.index_buffer
    }

    ///
    /// Get index buffer type.
    ///
    /// # Returns
    /// Data type (e.g., GL_UNSIGNED_INT).
    ///
    pub fn index_buffer_type(&self) -> u32 {
        self.index_buffer_type
    }

    ///
    /// Get attribute bindings.
    ///
    /// # Returns
    /// Attribute bindings.
    ///
    pub fn attribute_bindings(&self) -> &HashMap<usize, usize> {
        &self.attributes
    }

    ///
    /// Get vertex attribute for binding index.
    ///
    /// # Parameters
    /// - `index`: Binding index
    ///
    /// # Returns
    /// Index of vertex attribute, or None.
    ///
    pub fn get_attribute_binding(&self, index: usize) -> Option<usize> {
        if let Some(attribute_index) = self.attributes.get(&index) {
            Some(*attribute_index)
        } else {
            None
        }
    }

    ///
    /// Set vertex attribute for binding index.
    ///
    /// # Parameters
    /// - `index`: Binding index
    /// - `attribute_index`: Index of vertex attribute
    ///
    pub fn set_attribute_binding(&mut self, index: usize, attribute_index: usize) {
        self.attributes.insert(index, attribute_index);
    }

    ///
    /// Initialize VAO for primitive.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `vertex_attributes`: Resource manager for vertex attributes
    /// - `buffers`: Resource manager for buffers
    ///
    pub fn init_vao(&mut self,
        context: &Context,
        vertex_attributes: &ResourceManager<VertexAttribute>,
        buffers: &ResourceManager<Buffer>,
    ) {
        // Check if VAO needs to be initialized
        if self.vao.is_some() {
            return;
        }

        // Create new VAO
        self.vao = Some(VertexArray::new());

        // Get VAO
        if let Some(ref mut vao) = self.vao {
            // Initialize VAO
            vao.init(context);

            // Bind VAO
            vao.bind(context);

            // Bind vertex attributes
            for (bind_index, index) in self.attributes.iter() {
                // Get vertex attribute
                if let Some(ref vertex_attribute) = vertex_attributes.get(*index) {
                    // Get buffer
                    if let Some(ref buffer) = buffers.get(vertex_attribute.buffer()) {
                        // Create vertex attribute binding
                        vao.set_attribute(
                            context,
                            *bind_index as u32,
                            buffer,
                            vertex_attribute.components() as i32,
                            vertex_attribute.data_type(),
                            vertex_attribute.normalize(),
                            vertex_attribute.stride() as i32,
                            vertex_attribute.relative_offset() as i32
                        );

                        // Enable vertex attribute
                        vao.enable_attribute(context, *bind_index as u32);
                    }
                }
            }
        }
    }

    ///
    /// De-initialize VAO for primitive.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn deinit_vao(&mut self, context: &Context) {
        // Get VAO
        if let Some(ref mut vao) = self.vao {
            // De-initialize VAO
            vao.deinit(context);

            // Destroy VAO
            self.vao = None;
        }
    }

    ///
    /// Draw geometry.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn draw(&mut self, context: &Context) {
        // Get VAO
        if let Some(ref vao) = self.vao {
            // Draw VAO
            vao.bind(context);

            // [TODO] Dispatch draw_arrays/draw_elements based on index-buffer
            context.gl().draw_arrays(self.render_mode, 0, self.count as i32);
        }
    }
}
