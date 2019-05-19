use std::collections::HashMap;

///
/// Represents a renderable geometric primitive.
///
pub struct Primitive {
    attributes: HashMap<usize, usize>, // Attribute bindings
    index_buffer: Option<usize>, // Index buffer ID, or None
    index_buffer_type: u32, // Data type (e.g., GL_UNSIGNED_INT)
    material: usize, // Material ID
    render_mode: u32, // Render mode (e.g., GL_TRIANGLES)
    count: usize, // Number of elements
    // [TODO] VAO
}

impl Primitive {
    ///
    /// Create primitive.
    ///
    /// # Returns
    /// A new instance of Primitive.
    ///
    pub fn new() -> Self {
        Self {
            attributes: HashMap::new(),
            index_buffer: None,
            index_buffer_type: 0,
            material: 0,
            render_mode: 0,
            count: 0,
        }
    }

    pub fn attribute_bindings(&self) -> &HashMap<usize, usize> {
        &self.attributes
    }

    pub fn get_attribute_binding(&self, index: usize) -> Option<usize> {
        if let Some(attribute_index) = self.attributes.get(&index) {
            Some(*attribute_index)
        } else {
            None
        }
    }

    pub fn set_attribute_binding(&mut self, index: usize, attribute_index: usize) {
        self.attributes.insert(index, attribute_index);
    }

    pub fn index_buffer(&self) -> Option<usize> {
        self.index_buffer
    }

    pub fn set_index_buffer(&mut self, index: Option<usize>) {
        self.index_buffer = index;
    }

    pub fn index_buffer_type(&self) -> u32 {
        self.index_buffer_type
    }

    pub fn set_index_buffer_type(&mut self, data_type: u32) {
        self.index_buffer_type = data_type;
    }

    pub fn material(&self) -> usize {
        self.material
    }

    pub fn set_material(&mut self, material: usize) {
        self.material = material;
    }

    pub fn render_mode(&self) -> u32 {
        self.render_mode
    }

    pub fn set_render_mode(&mut self, render_mode: u32) {
        self.render_mode = render_mode;
    }

    pub fn count(&self) -> usize {
        self.count
    }

    pub fn set_count(&mut self, count: usize) {
        self.count = count;
    }
}
