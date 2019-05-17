use std::collections::HashMap;

use glutin::ControlFlow;

use crate::Window;

///
/// Representation of a glutin-based application.
///
/// This is the main entry point for native applications. The `Application`
/// manages the top-level [`Window`] instances and the main message loop.
///
/// The message loop supports both, non-continuous, and continuous rendering.
/// This is controlled by the signals of the [`Render`] object, which is set
/// onto a window's [`Canvas`]. By constantly signalling [`needs_update`] and
/// [`needs_redraw`], continous simulation and rendering can be achieved.
///
/// [`Window`]: struct.Window.html
/// [`Canvas`]: ../yage_core/struct.Canvas.html
/// [`Render`]: ../yage_core/trait.Render.html
/// [`needs_update`]: ../yage_core/trait.Render.html#tymethod.needs_update
/// [`needs_redraw`]: ../yage_core/trait.Render.html#tymethod.needs_redraw
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
pub struct Application {
    events_loop: glutin::EventsLoop,
    windows: HashMap<glutin::WindowId, Window>,
    running: bool,
    exit_code: i32,
}

impl Application {
    ///
    /// Create an application.
    ///
    /// # Returns
    /// A new instance of Application.
    ///
    pub fn new() -> Application {
        // Create event loop
        let events_loop = glutin::EventsLoop::new();

        // Return application
        Application {
            events_loop,
            windows: HashMap::new(),
            running: true,
            exit_code: 0,
        }
    }

    ///
    /// Add window to the application.
    ///
    /// # Parameters
    /// - `window`: Window that is transferred to the application.
    ///
    /// # Returns
    /// Window ID.
    ///
    pub fn add_window(&mut self, window: Window) -> glutin::WindowId {
        // Move window
        let id = window.id();
        self.windows.insert(id, window);

        // Return window ID
        id
    }

    ///
    /// Get windows that belong to the application.
    ///
    /// # Returns
    /// Map of window IDs -> Window.
    ///
    pub fn windows(&self) -> &HashMap<glutin::WindowId, Window> {
        &self.windows
    }

    ///
    /// Borrow reference to a specific window.
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
    /// Borrow mutable reference to a specific window.
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
    /// Borrow events loop.
    ///
    /// # Returns
    /// Reference to the events loop.
    ///
    pub fn events_loop(&self) -> &glutin::EventsLoop {
        &self.events_loop
    }

    ///
    /// Check if events loop is still running.
    ///
    /// # Returns
    /// State of the events loop.
    ///
    pub fn is_running(&self) -> bool {
        self.running
    }

    ///
    /// Get exit code.
    ///
    /// # Returns
    /// Exit code (0 for no error, > 0 for error)
    ///
    pub fn exit_code(&self) -> i32 {
        self.exit_code
    }

    ///
    /// Exit application.
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
    /// Run events loop.
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
        // Get references to data we want to access, because closure borrows self
        let windows = &mut self.windows;
        let running = &mut self.running;
        let proxy = self.events_loop.create_proxy();
        let mut wakeup_scheduled = false;

        // Run main loop
        self.events_loop.run_forever(|event| {
            // [DEBUG]
            // println!("{:?}", event);

            // Dispatch event
            #[allow(clippy::single_match)]
            match event {
                // Window events
                glutin::Event::WindowEvent { event, window_id } => {
                    // Get window
                    if let Some(window) = windows.get_mut(&window_id) {
                        // Dispatch window event
                        match event {
                            // Window resized
                            glutin::WindowEvent::Resized(logical_size) => {
                                // Calculate size in device coordinates
                                let dpi_factor = window.gl_window().get_hidpi_factor();
                                let size = logical_size.to_physical(dpi_factor);

                                // Set new size
                                window.on_resize(size);
                            }

                            // DPI factor changed
                            glutin::WindowEvent::HiDpiFactorChanged(dpi_factor) => {
                                if let Some(logical_size) = window.gl_window().get_inner_size() {
                                    // Calculate size in device coordinates
                                    let size = logical_size.to_physical(dpi_factor);

                                    // Set new size
                                    window.on_resize(size);
                                }
                            }

                            // Window closed
                            glutin::WindowEvent::CloseRequested => {
                                // Check whether to exit the application
                                let exit_on_close = window.get_exit_on_close();

                                // De-initialize and destroy the window
                                window.on_destroy();
                                windows.remove(&window_id);

                                // Stop application if instructed
                                *running = !exit_on_close;
                            }

                            // Window needs to be refreshed (painted)
                            glutin::WindowEvent::Refresh => {
                                // Draw window
                                window.on_draw();
                            }

                            // Other event
                            _ => (),
                        }
                    }
                }

                // Wakeup event
                glutin::Event::Awakened => {
                    // This is the update event, in which all windows will be updated
                    // and drawn if necessary. It is scheduled whenever a window has
                    // set its update or redraw flags to true.

                    // Reset wakeup
                    wakeup_scheduled = false;

                    // Update windows
                    for (_, window) in windows.iter_mut() {
                        // Update window if necessary
                        if window.needs_update() {
                            window.on_update();
                        }

                        // Redraw window if necessary
                        if window.needs_redraw() {
                            window.on_draw();
                        }
                    }
                }

                // Other event
                _ => ()
            }

            // After each event, we check if a window needs to be
            // updated (animation) or redrawn. If that is the case,
            // we schedule a wakeup event, which will be processed
            // after all events still waiting in the event queue.
            // We also make sure that this event is only added once.
            //
            // [TODO] The better way to do this would be to send
            // a redraw event to the window, but I don't see a way
            // in glutin to do that.
            for (_, window) in windows.iter_mut() {
                if (window.needs_update() || window.needs_redraw()) && !wakeup_scheduled {
                    assert!(proxy.wakeup().is_ok());
                    wakeup_scheduled = true;
                }
            }

            // Abort main loop?
            if !*running {
                ControlFlow::Break
            } else {
                ControlFlow::Continue
            }
        });

        // Return exit code
        self.exit_code
    }
}

impl Default for Application {
    fn default() -> Application {
        Application::new()
    }
}
