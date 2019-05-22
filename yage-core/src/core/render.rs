use cgmath::Vector4;

use crate::Context;
use crate::GpuObject;
use crate::Update;

///
/// Represents a component that executes rendering or other GPU-based computation code.
///
/// A `Render` object can be moved into a [`Canvas`] to control what is rendered onto
/// the screen. But `Render` objects can also be used on their own or as part of other
/// `Render` objects.
///
/// The rendering process is organized into two separate steps: simulation and rendering.
///
/// Rendering involves executing the actual rendering code to produce an image onto
/// the screen. This is implemented via [`render()`]. It receives an active [`Context`]
/// and therefore it is safe to call OpenGL functions within this method.
///
/// Simulation means updating or advancing the simulation. For this, [`update()`]
/// is called with the time delta that has elapsed since the last simulation step.
/// At this time, no [`Context`] is guaranteed to be active, so never try to access
/// OpenGL objects or call OpenGL functions within this method. Within this method,
/// only the state of the simulation (e.g., animations) shall be changed. To render
/// a new frame as a result, you can signal this via [`needs_redraw()`]. To schedule
/// an immediate next simulation step, signal this via [`needs_update()`].
///
/// By default, rendering is non-continous, i.e., [`render()`] is only triggered
/// when the window needs to be redrawn, for example because it has been moved or
/// resized. To implement continous rendering, [`needs_redraw()`] and [`needs_update()`]
/// should be used. The following rules apply, which must be respected by the
/// windowing backend:
///
/// - When [`needs_update()`] is set to `true`, an update is scheduled immediatly after
///   all previous events have been handled. As a result, [`update()`] is called.
/// - When [`needs_redraw()`] is set to `true`, a redraw is scheduled immediatly after
///   all previous events have been handled. As a result, [`render()`] is called.
/// - Both [`update()`] and [`render()`] can also be called at any time, or in response
///   to the other signal. Therefore, it should not be assumed that they are called
///   separately or in a particular order.
/// - [`update()`] can be called several times in row.
///
/// A `Render` object is also a [`GpuObject`].
///
/// [`Context`]: trait.Context.html
/// [`GpuObject`]: trait.GpuObject.html
/// [`Canvas`]: struct.Canvas.html
/// [`needs_update()`]: trait.Update.html#tymethod.needs_update
/// [`update()`]: trait.Update.html#tymethod.update
/// [`needs_redraw()`]: trait.Render.html#tymethod.needs_redraw
/// [`render()`]: trait.Render.html#tymethod.render
///
pub trait Render : GpuObject + Update {
    ///
    /// Set viewport.
    ///
    /// # Parameters
    /// - `viewport`: Size of viewport in device coordinates.
    ///
    fn set_viewport(&mut self, viewport: Vector4<i32>);

    ///
    /// Check if renderer needs a redraw
    ///
    /// # Returns
    /// true if a redraw is requested, else false
    ///
    fn needs_redraw(&self) -> bool;

    ///
    /// Render frame
    ///
    /// # Parameters
    /// - `context`: Current OpenGL context
    ///
    fn render(&mut self, context: &Context);
}
