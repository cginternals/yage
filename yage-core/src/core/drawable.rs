use crate::Context;

///
/// Represents a drawable object, such as a geometry or a render pass.
///
pub trait Drawable {
    ///
    /// Draw/execute the rendering code.
    ///
    /// # Parameters
    /// - `context`: Current OpenGL context
    ///
    fn draw(&self, context: &Context);
}
