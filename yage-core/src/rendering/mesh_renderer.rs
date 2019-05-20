use crate::{
    Context,
    Geometry,
    GpuObject, Transform, Camera,
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
    /// - `geometry`: Geometry that is drawn
    /// - `transform`: Transformation for the mesh
    ///
    fn draw(&mut self,
        context: &Context,
        camera: &Camera,
        geometry: &mut Geometry,
        transform: &Transform
    );
}
