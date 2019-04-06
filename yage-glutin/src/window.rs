/// Top-level window with OpenGL context.
pub struct Window {
    window: glutin::GlWindow
}

impl Window {
    /// Create a new window.
    ///
    /// # Return
    /// a new instance of Window.
    pub fn new() -> Window {
        // create event loop
        let events_loop = glutin::EventsLoop::new();

        // create window builder
        let window = glutin::WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));

        // create context builder
        let context = glutin::ContextBuilder::new();

        // create actual OpenGL window
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        // return window
        Window {
            window: gl_window
        }
    }
}
