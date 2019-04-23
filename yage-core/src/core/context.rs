use std::rc::Rc;

use crate::GL;

///
/// Interface for OpenGL contexts.
///
pub trait Context {
    ///
    /// Get OpenGL function wrapper
    ///
    fn gl(&self) -> &Rc<GL>;

    ///
    /// Make OpenGL context current
    ///
    fn make_current(&self);

    ///
    /// Swap buffers
    ///
    fn swap(&self);
}
