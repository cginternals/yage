use crate::{
    Geometry,
    Context, GpuObject, Drawable,
    Primitive, Buffer, VertexAttribute,
    opengl::glenum,
};

///
/// 3D cube shape
///
pub struct Cube {
    geometry: Geometry, // Geometry containing the cube
    initialized: bool, // Has the shape been initialized?
}

impl Cube {
    ///
    /// Create cube.
    ///
    /// # Returns
    /// A new instance of Cube.
    ///
    pub fn new() -> Self {
        Self {
            geometry: Geometry::new(),
            initialized: false
        }
    }

    ///
    /// Get geometry.
    ///
    /// # Returns
    /// Reference to geometry.
    ///
    pub fn geometry(&self) -> &Geometry {
        &self.geometry
    }

    ///
    /// Get geometry.
    ///
    /// # Returns
    /// Mutable reference to geometry.
    ///
    pub fn geometry_mut(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}

impl GpuObject for Cube {
    fn init(&mut self, context: &Context) {
        // Abort if already initialized
        if self.initialized {
            return;
        }

        // Create geometry
        self.geometry = Geometry::new();
        {
            // Create vertex buffer
            let mut buffer = Buffer::new(glenum::ARRAY_BUFFER);
            buffer.init(context);
            buffer.bind(context);
            buffer.set_data(context, &VERTEX_DATA, glenum::STATIC_DRAW);

            // Add vertex buffer
            let buffer_index = self.geometry.add_buffer(buffer);

            // Create vertex attribute for 'position'
            let va_position = VertexAttribute::new(
                buffer_index,
                0,
                0,
                5 * std::mem::size_of::<f32>(),
                glenum::FLOAT,
                3,
                false
            );

            // Create vertex attribute for 'texcoord'
            let va_texcoord = VertexAttribute::new(
                buffer_index,
                0,
                3 * std::mem::size_of::<f32>(),
                5 * std::mem::size_of::<f32>(),
                glenum::FLOAT,
                2,
                false
            );

            // Add vertex attributes
            let position_index = self.geometry.add_vertex_attribute(va_position);
            let texcoord_index = self.geometry.add_vertex_attribute(va_texcoord);

            // Create primitive
            let primitive = Primitive::new(
                0,
                glenum::TRIANGLES,
                36,
                None,
                0,
                &[ (0, position_index), (1, texcoord_index) ]
            );

            // Add primitive
            self.geometry.add_primitive(primitive);
        }

        // Done
        self.initialized = true;
    }

    fn deinit(&mut self, context: &Context) {
        // De-initialize geometry
        self.geometry.deinit(context);
    }
}

impl Drawable for Cube {
    fn draw(&mut self, context: &Context) {
        // Lazy initialization
        self.init(context);

        // Draw geometry
        self.geometry.draw(context);
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 180] = [
    -0.5, -0.5,  0.5, 1.0, 0.0,
     0.5, -0.5,  0.5, 0.0, 0.0,
    -0.5,  0.5,  0.5, 1.0, 1.0,

     0.5, -0.5,  0.5, 0.0, 0.0,
     0.5,  0.5,  0.5, 0.0, 1.0,
    -0.5,  0.5,  0.5, 1.0, 1.0,

    -0.5, -0.5, -0.5, 0.0, 0.0,
    -0.5,  0.5, -0.5, 0.0, 1.0,
     0.5, -0.5, -0.5, 1.0, 0.0,

    -0.5,  0.5, -0.5, 0.0, 1.0,
     0.5,  0.5, -0.5, 1.0, 1.0,
     0.5, -0.5, -0.5, 1.0, 0.0,

    -0.5, -0.5,  0.5, 0.0, 0.0,
    -0.5,  0.5,  0.5, 0.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 0.0,

    -0.5,  0.5,  0.5, 0.0, 1.0,
    -0.5,  0.5, -0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 0.0,

     0.5, -0.5, -0.5, 0.0, 0.0,
     0.5,  0.5, -0.5, 0.0, 1.0,
     0.5, -0.5,  0.5, 1.0, 0.0,

     0.5,  0.5, -0.5, 0.0, 1.0,
     0.5,  0.5,  0.5, 1.0, 1.0,
     0.5, -0.5,  0.5, 1.0, 0.0,

    -0.5,  0.5,  0.5, 1.0, 0.0,
     0.5,  0.5,  0.5, 0.0, 0.0,
    -0.5,  0.5, -0.5, 1.0, 1.0,

     0.5,  0.5,  0.5, 0.0, 0.0,
     0.5,  0.5, -0.5, 0.0, 1.0,
    -0.5,  0.5, -0.5, 1.0, 1.0,

    -0.5, -0.5,  0.5, 1.0, 1.0,
    -0.5, -0.5, -0.5, 1.0, 0.0,
     0.5, -0.5,  0.5, 0.0, 1.0,

    -0.5, -0.5, -0.5, 1.0, 0.0,
     0.5, -0.5, -0.5, 0.0, 0.0,
     0.5, -0.5,  0.5, 0.0, 1.0,
];
