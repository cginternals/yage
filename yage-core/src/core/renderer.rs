use crate::GpuObject;

///
/// A renderer in an object that executes the actual draw or computation code.
///
pub trait Renderer : GpuObject {
    ///
    /// Render frame
    ///
    fn render(&mut self);
}
