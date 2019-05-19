use crate::{
    Context,
    GpuObject, Drawable, Transform,
};

///
/// Represents a renderer that draws a mesh
///
pub trait MeshRenderer : GpuObject {
    ///
    /// Render a mesh.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `mesh`: Drawable that renders the mesh
    /// - `model_matrix`: Model matrix for the mesh
    ///
    fn draw(&mut self, context: &Context, mesh: &mut Drawable, model_matrix: &Transform);
}
