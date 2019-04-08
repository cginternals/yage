use std::collections::HashMap;

use crate::Window;

///
/// Representation of a glutin-based application.
///
pub struct Application {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Window>,
    running: bool
}

impl Application {
    ///
    /// Create an application instance
    ///
    /// # Returns
    ///
    /// A new instance of Application.
    ///
    pub fn new() -> Application {
        // create event loop
        let events_loop = glutin::EventsLoop::new();

        // return application
        Application {
            events_loop: events_loop,
            windows: HashMap::new(),
            running: true
        }
    }

    ///
    /// Add window to the application
    ///
    /// # Parameters
    ///
    /// - `window`: Window that is transferred to the application.
    ///
    /// # Returns
    ///
    /// Window ID.
    ///
    pub fn add_window(&mut self, window: Window) -> glutin::WindowId {
        // move window
        let id = window.id();
        self.windows.insert(id, window);

        // return window ID
        id
    }

    ///
    /// Get windows that belong to the application
    ///
    /// # Returns
    ///
    /// Map of window IDs -> Window.
    ///
    pub fn windows(&self) -> &HashMap<glutin::WindowId, Window> {
        &self.windows
    }

    ///
    /// Borrow mutable reference to a specific window
    ///
    /// # Parameters
    ///
    /// - `id`: Window ID
    ///
    /// # Returns
    ///
    /// Mutable reference to the window.
    ///
    pub fn window(&mut self, id: glutin::WindowId) -> Option<&mut Window> {
        self.windows.get_mut(&id)
    }

    ///
    /// Borrow events loop
    ///
    /// # Returns
    ///
    /// Reference to the events loop.
    ///
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    ///
    /// Check if events loop is still running
    ///
    /// # Returns
    ///
    /// State of the events loop.
    ///
    pub fn is_running(&self) -> bool {
        self.running
    }

    ///
    /// Stop events loop
    ///
    /// This will stop the events loop and thereby exit the application.
    ///
    pub fn stop(&mut self) {
        self.running = false;
    }

    ///
    /// Poll events for all windows once.
    ///
    pub fn poll_events(&mut self) {
        // get references to data we want to access, because closure borrows self
        let events_loop = &mut self.events_loop;
        let windows     = &self.windows;
        let running     = &mut self.running;

        // poll events
        events_loop.poll_events(|event| {
            // dispatch event
            #[allow(clippy::single_match)]
            match event {
                // window events
                glutin::Event::WindowEvent { event, window_id } => {
                    // get window
                    let window = windows.get(&window_id).unwrap();
                    let gl_window = window.get_gl_window();

                    // dispatch window event
                    match event {
                        // window closed
                        glutin::WindowEvent::CloseRequested => {
                            *running = false;
                        },

                        // window resized
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = gl_window.get_hidpi_factor();
                            gl_window.resize(logical_size.to_physical(dpi_factor));
                        },

                        // other event
                        _ => (),
                    }
                },

                // other event
                _ => ()
            }
        });
    }

    ///
    /// Run events loop
    ///
    /// Executes the events loop for the application.
    /// This function will block and run as long as the events loop
    /// is running. To stop the message loop, stop() has to be called
    /// on the Application object.
    ///
    /// # Returns
    ///
    /// Exit code (0 for no error, > 0 for error)
    ///
    pub fn run(&mut self) -> i32 {
        // run events loop until application is exited
        while self.running {
            self.poll_events();
        }

        // return value
        0
    }
}
