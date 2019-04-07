use std::collections::HashMap;

use crate::Window;

/// Representation of a glutin-based application.
pub struct Application {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Window>
}

impl Application {
    /// Create an application instance
    ///
    /// # Returns
    ///
    /// A new instance of Application.
    pub fn new() -> Application {
        // create event loop
        let events_loop = glutin::EventsLoop::new();

        // return application
        Application {
            events_loop: events_loop,
            windows: HashMap::new()
        }
    }

    /// Add a window to the application
    ///
    /// # Parameters
    ///
    /// - `window`: Window that is transferred to the application.
    ///
    /// # Returns
    ///
    /// Mutable reference to the window.
    pub fn add_window(&mut self, window: Window) {
        let id = window.id();

        // move window
        //let w : &mut Window = self.window1.get_or_insert(Box::new(window));
        self.windows.insert(id, window);

        // return reference to window
        // self.windows.get_mut(&id).unwrap()
    }

    pub fn first_window(&self) -> &Window {
        self.windows.values().next().unwrap()
    }

    /// Borrow events loop
    ///
    /// # Returns
    ///
    /// Reference to the events loop.
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    /// [TODO]
    pub fn poll_events(&mut self) -> bool {
        let mut running = true;

        let events_loop = &mut self.events_loop;
        let windows     = &self.windows;

        events_loop.poll_events(|event| {
            // Dispatch event
            #[allow(clippy::single_match)]
            match event {
                // Window events
                glutin::Event::WindowEvent { event, window_id } => {
                    // get window
                    let window = windows.get(&window_id).unwrap();
                    let gl_window = window.get_gl_window();

                    // Dispatch window event
                    match event {
                        // Window closed
                        glutin::WindowEvent::CloseRequested => {
                            running = false
                        },

                        // Window resized
                        glutin::WindowEvent::Resized(logical_size) => {
                            let dpi_factor = gl_window.get_hidpi_factor();
                            gl_window.resize(logical_size.to_physical(dpi_factor));
                        },

                        // Other event
                        _ => (),
                    }
                },

                // Other event
                _ => ()
            }
        });

        running
    }

    /// Run the main loop
    ///
    /// Executes the message loop for the application.
    /// This function will block and run as long as the message
    /// loop is running. To stop the message loop, stop() has to
    /// be called on the Application object.
    ///
    /// # Returns
    ///
    /// Exit code (0 for no error, > 0 for error)
    pub fn run(&self) -> i32 {
        0
    }
}
