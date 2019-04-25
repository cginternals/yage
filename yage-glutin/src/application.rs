use std::collections::HashMap;

use glutin::ControlFlow;

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
    /// Borrow mutable reference to a specific window
    ///
    /// # Parameters
    /// - `id`: Window ID
    ///
    /// # Returns
    /// Mutable reference to the window.
    ///
    /// # Undefined Behavior
    /// When the application only has a single window, the return value
    /// will always be that window, regardless of the given id.
    ///
    pub fn window_mut(&mut self, id: glutin::WindowId) -> Option<&mut Window> {
        if self.windows.len() == 1 {
            self.windows.values_mut().next()
        } else {
            self.windows.get_mut(&id)
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
        // get references to data we want to access, because closure borrows self
        let windows = &mut self.windows;
        let running = &mut self.running;

        // run main loop
        self.events_loop.run_forever(|event| {
            // dispatch event
            #[allow(clippy::single_match)]
            match event {
                // window events
                glutin::Event::WindowEvent { event, window_id } => {
                    // get window
                    if let Some(window) = windows.get_mut(&window_id) {
                        // dispatch window event
                        match event {
                            // window resized
                            glutin::WindowEvent::Resized(logical_size) => {
                                // calculate size in device coordinates
                                let dpi_factor = window.gl_window().get_hidpi_factor();
                                let size = logical_size.to_physical(dpi_factor);

                                // set new size
                                window.on_resize(size);
                            }

                            // DPI factor changed
                            glutin::WindowEvent::HiDpiFactorChanged(dpi_factor) => {
                                if let Some(logical_size) = window.gl_window().get_inner_size() {
                                    // calculate size in device coordinates
                                    let size = logical_size.to_physical(dpi_factor);

                                    // set new size
                                    window.on_resize(size);
                                }
                            }

                            // window closed
                            glutin::WindowEvent::CloseRequested => {
                                // check whether to exit the application
                                let exit_on_close = window.get_exit_on_close();

                                // de-initialize and destroy the window
                                window.on_destroy();
                                windows.remove(&window_id);

                                // stop application if instructed
                                *running = !exit_on_close;
                            }

                            // window needs to be refreshed (painted)
                            glutin::WindowEvent::Refresh => {
                                // draw window
                                window.on_draw();
                            }

                            // other event
                            _ => (),
                        }
                    }
                }

                // other event
                _ => (),
            }

            // abort main loop?
            if !*running {
                ControlFlow::Break
            } else {
                ControlFlow::Continue
            }
        });

        // return exit code
        self.exit_code
    }
}

impl Default for Application {
    fn default() -> Application {
        Application::new()
    }
}
