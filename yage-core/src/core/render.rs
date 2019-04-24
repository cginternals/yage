use crate::Context;
use crate::GpuObject;

///
/// Represents a component that executes a rendering or other GPU-based computation code.
///
pub trait Render : GpuObject {
    ///
    /// Render frame
    ///
    /// # Parameters
    /// - `context`: Current OpenGL context
    ///
    fn render(&mut self, context: &Context);
}
