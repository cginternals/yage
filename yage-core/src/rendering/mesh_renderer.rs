use crate::{
    Context,
    GpuObject, Drawable, Transform, Camera,
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
    /// - `camera`: Active camera
    /// - `mesh`: Drawable that renders the mesh
    /// - `transform`: Transformation for the mesh
    ///
    fn draw(&mut self,
        context: &Context,
        camera: &Camera,
        mesh: &mut Drawable,
        transform: &Transform
    );
}
