use crate::Context;

///
/// Represents a drawable object, such as a geometry or a render pass.
///
pub trait Drawable {
    ///
    /// Draw/execute the rendering code.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    fn draw(&mut self, context: &Context);
}
