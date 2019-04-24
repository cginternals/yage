use crate::Context;
use crate::GpuObject;

///
/// A renderer in an object that executes the actual draw or computation code.
///
pub trait Renderer : GpuObject {
    ///
    /// Render frame
    ///
    /// # Parameters
    /// - `context`: Current OpenGL context
    ///
    fn render(&mut self, context: &Context);
}
