/// Representation of a glutin-based application.
pub struct Application {
}

impl Application {
    /// Create an application instance.
    ///
    /// # Return
    /// a new instance of Application.
    pub fn new() -> Application {
        Application {
        }
    }

    /// Run the main loop.
    ///
    /// Execute the message loop for the application.
    /// This function will block and run as long as the message
    /// loop is running. To stop the message loop, stop() has to
    /// be called on the Application object.
    ///
    /// # Return
    /// exit code (0 for no error, > 0 for error)
    pub fn run(&self) -> i32 {
        0
    }
}
