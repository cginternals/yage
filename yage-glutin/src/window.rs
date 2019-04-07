use glutin::GlContext;
use glutin::WindowId;

use crate::Context;

/// Top-level window with OpenGL context.
pub struct Window {
    window: glutin::GlWindow
}

impl Window {
    /// Create a new window for an application
    ///
    /// This creates a new top-level window and transfers ownership
    /// to the specified application.
    ///
    /// # Parameters
    ///
    /// - `application`: Application for which the window is created.
    ///
    /// # Returns
    ///
    /// Reference to the new window.
    pub fn new(events_loop: &glutin::EventsLoop) -> Window {
        // create window builder
        let window_builder = glutin::WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));

        // create context builder
        let context_builder = glutin::ContextBuilder::new();

        // create actual OpenGL window
        let gl_window = glutin::GlWindow::new(window_builder, context_builder, events_loop).unwrap();

        // [TODO] initialize OpenGL in context
        gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

        // create window
        Window {
            window: gl_window
        }

        // move window into application and return reference
        // application.add_window(window)
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn get_gl_window(&self) -> &glutin::GlWindow {
        &self.window
    }

    pub fn swap_buffers(&self) {
        let _ = self.window.swap_buffers();
    }

/*
    pub fn poll_events(&mut self) -> bool {
        let mut running = true;

        let events_loop = &mut self.events_loop;
        let window      = &mut self.window;

        events_loop.poll_events(|event| {
            #[allow(clippy::single_match)]
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => {
                        running = false
                    },
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = window.get_hidpi_factor();
                        window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        running
    }
*/
}

impl Context for Window {
    fn make_current(&self) {
        let _ = unsafe { self.window.make_current() };
    }

    fn release(&self) {
        // [TODO]
    }
}
