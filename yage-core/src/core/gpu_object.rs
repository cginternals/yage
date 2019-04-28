use crate::Context;

///
/// A `GpuObject` represents or owns data on the GPU.
///
/// In a complex application architecture, it cannot be guaranteed that
/// an OpenGL [`Context`] is active at the time an object is created or
/// deleted. Therefore, OpenGL objects and functions must only be accessed
/// when a [`Context`] is certainly active. Therefore, no OpenGL functions
/// should be called for example when creating an object or during `drop`.
/// An OpenGL [`Context`] can also be replaced, for example when a window
/// switches from windowed to fullscreen mode. In this case, the GPU object
/// must be restored on the new context, although the actual struct instance
/// has not been modified.
///
/// This trait makes sure that GPU data is initialized on the OpenGL
/// [`Context`] when it has been created, and released when the [`Context`]
/// is destroyed or replaced. It is the responsibility of the windowing
/// backend to make sure that this logic is implemented correctly.
///
/// This trait guarantees that a [`Context`] has been created and made
/// active on [`init()`]. This function should be used to initialize the
/// data on the GPU. It should also be used to call [`init()`] on all
/// owned `GpuObject` objects. Before a [`Context`] is destroyed,
/// it is made active once more and [`deinit()`] is called. In this
/// function, all OpenGL objects must be released and [`deinit()`]
/// must be called on all owned `GpuObject` objects. This process also
/// enables GPU objects to read data back onto the CPU in [`deinit()`],
/// and restore from CPU data in [`init()`].
///
/// All methods on a GPU object which need an active OpenGL context
/// should require a [`Context`] reference as an argument, and only
/// be called from a method which guarantees an active context,
/// such as [`init()`], [`deinit()`], or [`Render::update()`].
///
/// [`Context`]: trait.Context.html
/// [`init()`]: trait.Context.html#tymethod.init
/// [`deinit()`]: trait.Context.html#tymethod.deinit
/// [`Render::update()`]: trait.Render.html#tymethod.update
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
