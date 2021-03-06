use std::rc::Rc;

use glutin::GlContext;
use glutin::WindowId;
use glutin::dpi::PhysicalSize;

use yage_core::gl;
use yage_core::cgmath;
use yage_core::GpuObject;
use yage_core::GL;
use yage_core::Context;
use yage_core::Canvas;
use yage_core::Render;
use yage_core::Update;

use crate::Application;

///
/// Top-level window with OpenGL context.
///
/// A `Window` represents a top-level with an OpenGL context. After creating a
/// `Window`, it must be moved into an [`Application`], which will handle all
/// events in its main loop.
///
/// A `Window` contains a [`Canvas`], which can be accessed via [`canvas()`]
/// and [`canvas_mut()`]. To control the rendering into the window, set a
/// [`Render`] onto the [`Canvas`].
///
/// [`Application`]: struct.Application.html
/// [`Render`]: ../yage_core/trait.Render.html
/// [`Canvas`]: ../yage_core/struct.Canvas.html
/// [`canvas()`]: struct.Window.html#method.canvas
/// [`canvas_mut()`]: struct.Window.html#method.canvas_mut
///
/// # Examples
///
/// ```rust
/// let mut app = Application::new();
/// let mut window = Window::new(&app);
///
/// window.canvas_mut().set_renderer(MyRenderer::new());
///
/// let _ = app.add_window(window);
/// app.run();
///
pub struct Window {
    canvas: Canvas,
    context: WindowContext,
    exit_on_close: bool
}

impl Window {
    ///
    /// Create a new window for an application.
    ///
    /// This creates a new top-level window using the events loop of the
    /// specified application. Afterwards, you should move the window
    /// into the application using Application::add_window().
    ///
    /// # Parameters
    /// - `application`: Application for which the window is created.
    ///
    /// # Returns
    /// A new instance of Window.
    ///
    pub fn new(application: &Application) -> Window {
        // Create window builder
        let window_builder = glutin::WindowBuilder::new()
            .with_title("Yage")
            .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));

        // Create context builder
        let context_builder = glutin::ContextBuilder::new();

        // Create OpenGL window
        let gl_window =
            glutin::GlWindow::new(window_builder, context_builder, application.events_loop())
                .unwrap();

        // Activate context
        unsafe {
            let _ = gl_window.make_current();
        }

        // Resolve OpenGL functions
        gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

        // Create OpenGL function wrapper
        let gl = Rc::new(GL::new());

        // Create context
        let context = WindowContext {
            window: gl_window,
            gl: gl.clone()
        };

        // Create and initialize canvas
        let mut canvas = Canvas::new(&gl);
        canvas.init(&context);

        // Create window
        Window {
            canvas: Canvas::new(&gl),
            context,
            exit_on_close: true
        }
    }

    ///
    /// Get window ID.
    ///
    /// # Returns
    /// The ID of the window.
    ///
    pub fn id(&self) -> WindowId {
        self.context.window.id()
    }

    ///
    /// Set window title.
    ///
    /// # Parameters
    /// - `title`: The new window title
    ///
    pub fn set_title(&self, title: &str) {
        self.context.window.set_title(title);
    }

    ///
    /// Check if application shall be quit when the window is closed.
    ///
    /// # Returns
    /// true to quit application on close, else false
    ///
    pub fn get_exit_on_close(&self) -> bool {
        self.exit_on_close
    }

    ///
    /// Set if application shall be quit when the window is closed.
    ///
    /// # Parameters
    /// - `exit_on_close`: true to quit application on close, else false
    ///
    pub fn set_exit_on_close(&mut self, exit_on_close: bool) {
        self.exit_on_close = exit_on_close;
    }

    ///
    /// Get reference to the wrapped OpenGL window.
    ///
    /// # Returns
    /// Reference to the OpenGL window.
    ///
    pub fn gl_window(&self) -> &glutin::GlWindow {
        &self.context.window
    }

    ///
    /// Get mutable reference to the wrapped OpenGL window.
    ///
    /// # Returns
    /// Mutable reference to the OpenGL window.
    ///
    pub fn gl_window_mut(&mut self) -> &mut glutin::GlWindow {
        &mut self.context.window
    }

    ///
    /// Get reference to the window's canvas.
    ///
    /// # Returns
    /// Reference to the canvas.
    ///
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
    }

    ///
    /// Get mutable reference to the window's canvas.
    ///
    /// # Returns
    /// Reference to the canvas.
    ///
    pub fn canvas_mut(&mut self) -> &mut Canvas {
        &mut self.canvas
    }

    ///
    /// Check if a simulation update is needed.
    ///
    /// # Returns
    /// true if an update is requested, else false
    ///
    pub fn needs_update(&self) -> bool {
        self.canvas.needs_update()
    }

    ///
    /// Check if window needs to redraw.
    ///
    /// # Returns
    /// true if a redraw is requested, else false
    ///
    pub fn needs_redraw(&self) -> bool {
        self.canvas.needs_redraw()
    }

    ///
    /// Called when the window has been resized.
    ///
    /// # Parameters
    /// - `size`: Size in device coordinates.
    ///
    pub(crate) fn on_resize(&mut self, size: PhysicalSize) {
        // Update client area
        self.context.window.resize(size);

        // Update canvas viewport
        self.canvas.set_viewport(cgmath::Vector4::new(0, 0, size.width as i32, size.height as i32));
    }

    ///
    /// Called when the window is being destroyed.
    ///
    pub(crate) fn on_destroy(&mut self) {
        // Activate context
        self.context.make_current();

        // De-initialize canvas
        self.canvas.deinit(&self.context);
    }

    ///
    /// Called once every mainloop iteration.
    ///
    pub(crate) fn on_update(&mut self) {
        // Update time delta
        self.canvas.update_time();

        // Update simulation (regardless of whether it wants to or not - ensures to wakeup the update loop when one event is lost)
        self.canvas.update(0.0);
    }

    ///
    /// Called when the window needs to be drawn.
    ///
    pub(crate) fn on_draw(&mut self) {
        // Draw canvas
        self.canvas.render(&self.context);

        // Swap buffers
        self.context.swap();
    }
}

///
/// OpenGL context of a glutin window.
///
struct WindowContext {
    window: glutin::GlWindow,
    gl: Rc<GL>
}

impl Context for WindowContext {
    fn make_current(&self) {
        let _ = unsafe { self.window.make_current() };
    }

    fn swap(&self) {
        let _ = self.window.swap_buffers();
    }

    fn gl(&self) -> &Rc<GL> {
        &self.gl
    }
}
