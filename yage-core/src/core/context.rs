use std::rc::Rc;

use crate::GL;

///
/// Representation of an OpenGL context.
///
/// This represents an actual OpenGL context. It it usually provided by the
/// windowing backend and belongs either to a [`Window`] or represents an
/// offscreen context.
///
/// [`Window`]: ../yage_glutin/struct.Window.html
///
pub trait Context {
    ///
    /// Make OpenGL context current.
    ///
    /// This activates the OpenGL context in the current thread.
    ///
    fn make_current(&self);

    ///
    /// Swap buffers.
    ///
    /// Swaps the back and front buffers.
    ///
    fn swap(&self);

    ///
    /// Get OpenGL function wrapper.
    ///
    /// # Returns
    /// Rc with the OpenGL function wrapper for this context.
    ///
    fn gl(&self) -> &Rc<GL>;
}
