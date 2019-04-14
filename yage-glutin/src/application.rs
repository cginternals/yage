use std::collections::HashMap;

use crate::Window;

///
/// Representation of a glutin-based application.
///
pub struct Application {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Window>,
    running: bool,
    exit_code: i32,
}

impl Application {
    ///
    /// Create an application instance
    ///
    /// # Returns
    /// A new instance of Application.
    ///
    pub fn new() -> Application {
        // create event loop
        let events_loop = glutin::EventsLoop::new();

        // return application
        Application {
            events_loop,
            windows: HashMap::new(),
            running: true,
            exit_code: 0,
        }
    }

    ///
    /// Add window to the application
    ///
    /// # Parameters
    /// - `window`: Window that is transferred to the application.
    ///
    /// # Returns
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
    /// Map of window IDs -> Window.
    ///
    pub fn windows(&self) -> &HashMap<glutin::WindowId, Window> {
        &self.windows
    }

    ///
    /// Borrow reference to a specific window
    ///
    /// # Parameters
    /// - `id`: Window ID
    ///
    /// # Returns
    /// Reference to the window.
    ///
    /// # Undefined Behavior
    /// When the application only has a single window, the return value
    /// will always be that window, regardless of the given id.
    ///
    pub fn window(&self, id: glutin::WindowId) -> Option<&Window> {
        if self.windows.len() == 1 {
            self.windows.values().next()
        } else {
            self.windows.get(&id)
        }
    }

    ///
    /// Borrow events loop
    ///
    /// # Returns
    /// Reference to the events loop.
    ///
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    ///
    /// Check if events loop is still running
    ///
    /// # Returns
    /// State of the events loop.
    ///
    pub fn is_running(&self) -> bool {
        self.running
    }

    ///
    /// Get exit code
    ///
    /// # Returns
    /// Exit code (0 for no error, > 0 for error)
    ///
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    ///
    /// Exit application
    ///
    /// This will stop the events loop and thereby exit the application.
    ///
    /// # Parameters
    /// - `exit_code`: Exit code (0 for no error, > 0 for error)
    ///
    pub fn stop(&mut self, exit_code: i32) {
        self.running = false;
        self.exit_code = exit_code;
    }

    ///
    /// Poll events for all windows once.
    ///
    pub fn poll_events(&mut self) {
        // get references to data we want to access, because closure borrows self
        let events_loop = &mut self.events_loop;
        let windows = &self.windows;
        let running = &mut self.running;
        let first_window = windows.values().next();

        // poll events
        events_loop.poll_events(|event| {
            // dispatch event
            #[allow(clippy::single_match)]
            match event {
                // window events
                glutin::Event::WindowEvent { event, window_id } => {
                    // get window
                    let window = if !first_window.is_some() {
                        first_window.unwrap()
                    } else {
                        windows.get(&window_id).unwrap()
                    };

                    // get GlWindow
                    let gl_window = window.get_gl_window();

                    // dispatch window event
                    match event {
                        // window closed
                        glutin::WindowEvent::CloseRequested => {
                            *running = false;
                        }

                        // window resized
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = gl_window.get_hidpi_factor();
                            gl_window.resize(logical_size.to_physical(dpi_factor));
                        }

                        // other event
                        _ => (),
                    }
                }

                // other event
                _ => (),
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
    /// Exit code (0 for no error, > 0 for error)
    ///
    pub fn run(&mut self) -> i32 {
        // run events loop until application is exited
        while self.running {
            self.poll_events();
        }

        // return exit code
        self.exit_code
    }
}

impl Default for Application {
    fn default() -> Application {
        Application::new()
    }
}
