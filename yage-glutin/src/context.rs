///
/// Interface for OpenGL contexts.
///
pub trait Context {
    fn make_current(&self);
}
