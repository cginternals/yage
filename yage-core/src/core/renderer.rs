
///
/// A renderer in an object that executes the actual draw or computation code.
///
pub trait Renderer {
    ///
    /// Initialize in OpenGL context
    ///
    fn init(&mut self);

    ///
    /// De-Initialize in OpenGL context
    ///
    fn deinit(&mut self);

    ///
    /// Render frame
    ///
    fn render(&self);
}
