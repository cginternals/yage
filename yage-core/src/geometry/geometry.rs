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
    buffers: ResourceManager<Buffer>,
    attributes: ResourceManager<VertexAttribute>,
    materials: ResourceManager<Material>,
    primitives: Vec<Primitive>,
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
    pub fn buffers(&self) -> &ResourceManager<Buffer> {
        &self.buffers
    }

    pub fn buffers_mut(&mut self) -> &mut ResourceManager<Buffer> {
        &mut self.buffers
    }

    pub fn add_buffer(&mut self, buffer: Buffer) -> usize {
        self.buffers.add(buffer)
    }

    pub fn vertex_attributes(&self) -> &ResourceManager<VertexAttribute> {
        &self.attributes
    }

    pub fn vertex_attributes_mut(&mut self) -> &mut ResourceManager<VertexAttribute> {
        &mut self.attributes
    }

    pub fn add_vertex_attribute(&mut self, attribute: VertexAttribute) -> usize {
        self.attributes.add(attribute)
    }

    pub fn materials(&self) -> &ResourceManager<Material> {
        &self.materials
    }

    pub fn materials_mut(&mut self) -> &mut ResourceManager<Material> {
        &mut self.materials
    }

    pub fn add_material(&mut self, material: Material) -> usize {
        self.materials.add(material)
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }
}

impl GpuObject for Geometry {
    fn init(&mut self, _context: &Context) {
    }

    fn deinit(&mut self, _context: &Context) {
    }
}

impl Drawable for Geometry {
    fn draw(&self, _context: &Context) {
    }
}
