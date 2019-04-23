///
/// Interface for OpenGL contexts.
///
pub trait Context {
    ///
    /// Make OpenGL context current
    ///
    fn make_current(&self);

    ///
    /// Swap buffers
    ///
    fn swap(&self);
}
