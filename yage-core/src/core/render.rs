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

    ///
    /// Check if renderer needs a redraw
    ///
    /// # Returns
    /// true if a redraw is requested, else false
    ///
    fn needs_redraw(&self) -> bool;
}
