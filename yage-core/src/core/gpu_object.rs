use crate::Context;

///
/// A GPU object represents or owns data on the GPU.
///
pub trait GpuObject {
    ///
    /// Check if object has been initialized in OpenGL context
    ///
    /// # Returns
    /// true if object has been initialized, else false
    ///
    fn is_initialized(&self) -> bool;

    ///
    /// Initialize in OpenGL context
    ///
    /// # Parameters
    /// - `context`: OpenGL context in which the GPU object in initialized
    ///
    fn init(&mut self, context: &Context);

    ///
    /// De-Initialize in OpenGL context
    ///
    /// # Parameters
    /// - `context`: OpenGL context from which the GPU object in de-initialized
    ///
    fn deinit(&mut self, context: &Context);
}
