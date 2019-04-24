use crate::Context;

///
/// A GPU object represents or owns data on the GPU.
///
pub trait GpuObject {
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
