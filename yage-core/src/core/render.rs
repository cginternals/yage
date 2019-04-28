use crate::Context;
use crate::GpuObject;

///
/// Represents a component that executes a rendering or other GPU-based computation code.
///
pub trait Render : GpuObject {
    ///
    /// Check if a simulation update is needed
    ///
    /// # Returns
    /// true if an update is requested, else false
    ///
    fn needs_update(&self) -> bool;

    ///
    /// Update simulation
    ///
    /// # Parameters
    /// - `time_data`: Time delta (in seconds)
    ///
    fn update(&mut self, time_delta: f64);

    ///
    /// Check if renderer needs a redraw
    ///
    /// # Returns
    /// true if a redraw is requested, else false
    ///
    fn needs_redraw(&self) -> bool;

    ///
    /// Render frame
    ///
    /// # Parameters
    /// - `context`: Current OpenGL context
    ///
    fn render(&mut self, context: &Context);
}
