use crate::{
    Context,
    MeshRenderer, GpuObject, Drawable, Transform,
};

///
/// PBR mesh renderer
///
pub struct PbrMeshRenderer {
}

impl PbrMeshRenderer {
    ///
    /// Create renderer.
    ///
    /// # Returns
    /// A new instance of PbrMeshRenderer.
    ///
    pub fn new() -> Self {
        Self {
        }
    }
}

impl MeshRenderer for PbrMeshRenderer {
    fn draw(&mut self, _context: &Context, _mesh: &mut Drawable, _model_matrix: &Transform) {
    }
}

impl GpuObject for PbrMeshRenderer {
    fn init(&mut self, _context: &Context) {
    }

    fn deinit(&mut self, _context: &Context) {
    }
}
