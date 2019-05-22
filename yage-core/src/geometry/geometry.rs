use crate::{
    Context, GpuObject, Drawable, ResourceManager,
    Primitive, Buffer, VertexAttribute, Material
};

///
/// Represents a renderable geometry
///
/// A geometry represents polygonal geometry that can be rendered
/// onto the screen. It consists of the primitives that make up
/// the parts of the geometry, their buffers, vertex attributes,
/// and materials.
///
pub struct Geometry {
    buffers: ResourceManager<Buffer>, // Vertex buffers
    attributes: ResourceManager<VertexAttribute>, // Vertex attributes
    materials: ResourceManager<Material>, // Materials
    primitives: Vec<Primitive>, // Geometric primitives
}

impl Geometry {
    ///
    /// Create geometry.
    ///
    /// # Returns
    /// A new instance of Geometry.
    ///
    pub fn new() -> Self {
        Self {
            buffers: ResourceManager::new(),
            attributes: ResourceManager::new(),
            materials: ResourceManager::new(),
            primitives: Vec::new(),
        }
    }

    ///
    /// Get buffers.
    ///
    /// # Returns
    /// Reference to resource manager for buffers.
    ///
    pub fn buffers(&self) -> &ResourceManager<Buffer> {
        &self.buffers
    }

    ///
    /// Get buffers.
    ///
    /// # Returns
    /// Mutable reference to resource manager for buffers.
    ///
    pub fn buffers_mut(&mut self) -> &mut ResourceManager<Buffer> {
        &mut self.buffers
    }

    ///
    /// Add buffer.
    ///
    /// # Parameters
    /// - `buffer`: Vertex buffer
    ///
    /// # Returns
    /// Index of vertex buffer.
    ///
    pub fn add_buffer(&mut self, buffer: Buffer) -> usize {
        self.buffers.add(buffer)
    }

    ///
    /// Get vertex attributes.
    ///
    /// # Returns
    /// Reference to resource manager for vertex attributes.
    ///
    pub fn vertex_attributes(&self) -> &ResourceManager<VertexAttribute> {
        &self.attributes
    }

    ///
    /// Get vertex attributes.
    ///
    /// # Returns
    /// Mutable reference to resource manager for vertex attributes.
    ///
    pub fn vertex_attributes_mut(&mut self) -> &mut ResourceManager<VertexAttribute> {
        &mut self.attributes
    }

    ///
    /// Add vertex attribute.
    ///
    /// # Parameters
    /// - `attribute`: Vertex attribute
    ///
    /// # Returns
    /// Index of vertex attribute.
    ///
    pub fn add_vertex_attribute(&mut self, attribute: VertexAttribute) -> usize {
        self.attributes.add(attribute)
    }

    ///
    /// Get materials.
    ///
    /// # Returns
    /// Reference to resource manager for materials.
    ///
    pub fn materials(&self) -> &ResourceManager<Material> {
        &self.materials
    }

    ///
    /// Get materials.
    ///
    /// # Returns
    /// Mutable reference to resource manager for materials.
    ///
    pub fn materials_mut(&mut self) -> &mut ResourceManager<Material> {
        &mut self.materials
    }

    ///
    /// Add material.
    ///
    /// # Parameters
    /// - `material`: Material
    ///
    /// # Returns
    /// Index of material.
    ///
    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.add(material)
    }

    ///
    /// Add primitive to geometry.
    ///
    /// # Parameters
    /// - `primitive`: Geometry primitive
    ///
    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

impl GpuObject for Geometry {
    fn init(&mut self, context: &Context) {
        // Initialize buffers
        for buffer in self.buffers.objects_mut() {
            buffer.init(context);
        }

        // Initialize primitives
        for primitive in &mut self.primitives {
            primitive.init_vao(
                context,
                &self.attributes,
                &self.buffers
            );
        }
    }

    fn deinit(&mut self, context: &Context) {
        // De-initialize buffers
        for buffer in self.buffers.objects_mut() {
            buffer.deinit(context);
        }

        // De-initialize primitives
        for primitive in &mut self.primitives {
            primitive.deinit_vao(context);
        }
    }
}

impl Drawable for Geometry {
    fn draw(&mut self, context: &Context) {
        // Lazy initialization
        self.init(context);

        // Draw primitives
        for primitive in &mut self.primitives {
            primitive.draw(context);
        }
    }
}
