
///
/// A GPU object represents or owns data on the GPU.
///
pub trait GpuObject {
    ///
    /// Initialize in OpenGL context
    ///
    fn init(&mut self);

    ///
    /// De-Initialize in OpenGL context
    ///
    fn deinit(&mut self);
}
