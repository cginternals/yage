use std::rc::Rc;

use cgmath::{Vector4};

use crate::Context;
use crate::GpuObject;
use crate::Render;
use crate::GL;
use crate::GlFunctions;

///
/// A canvas represents an area into which can be rendered.
///
pub struct Canvas {
    gl: Rc<GL>,
    viewport: Vector4<i32>,
    renderer: Option<Box<dyn Render>>,
    new_renderer: Option<Box<dyn Render>>,
    renderer_initialized: bool
}

impl Canvas {
    ///
    /// Create a canvas instance
    ///
    /// # Parameters
    /// - `gl`: GL context
    ///
    /// # Returns
    /// A new instance of Canvas.
    ///
    pub fn new(gl: &Rc<GL>) -> Canvas {
        // Return canvas
        Canvas {
            gl: gl.clone(),
            viewport: Vector4::new(0, 0, 0, 0),
            renderer: None,
            new_renderer: None,
            renderer_initialized: false
        }
    }

    ///
    /// Get viewport
    ///
    /// # Returns
    /// Size of viewport in device coordinates.
    ///
    pub fn get_viewport(&self) -> Vector4<i32> {
        self.viewport
    }

    ///
    /// Set viewport
    ///
    /// # Parameters
    /// - `viewport`: Size of viewport in device coordinates.
    ///
    pub fn set_viewport(&mut self, viewport: Vector4<i32>) {
        self.viewport = viewport;
    }

    ///
    /// Set renderer that will draw into the canvas
    ///
    /// # Parameters
    /// - `renderer`: Render object that will draw into the canvas.
    ///
    pub fn set_renderer<T: 'static + Render>(&mut self, renderer: T) {
        // Check if there already is a renderer set
        if self.renderer.is_some() {
            // Store new renderer to replace the old one on the next render call
            self.new_renderer = Some(Box::new(renderer));
        } else {
            // Store new renderer right away
            self.renderer = Some(Box::new(renderer));
            self.renderer_initialized = false;
        }
    }
}

impl GpuObject for Canvas {
    fn init(&mut self, context: &Context) {
        // [DEBUG]
        println!("initializing canvas");

        // Check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // Initialize renderer
            renderer.init(context);
            self.renderer_initialized = true;
        }
    }

    fn deinit(&mut self, context: &Context) {
        // [DEBUG]
        println!("de-initializing canvas");

        // Check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // De-initialize renderer
            renderer.deinit(context);
            self.renderer_initialized = false;
        }
    }
}

impl Render for Canvas {
    fn render(&mut self, context: &Context) {
        // Check if a new renderer waits to replace the old one
        if self.new_renderer.is_some() {
            // De-initialize the old renderer
            if let Some(ref mut renderer) = self.renderer {
                renderer.deinit(context);
                self.renderer_initialized = false;
            }

            // Replace renderer
            match self.new_renderer.take() {
                None => (),
                Some(new_renderer) => {
                    self.renderer = Some(new_renderer);
                    self.renderer_initialized = false;
                }
            }
        }

        // Check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // Check if renderer has been initialized
            if !self.renderer_initialized {
                // Initialize renderer
                renderer.init(context);
                self.renderer_initialized = true;
            }

            // Set viewport
            self.gl.viewport(self.viewport.x, self.viewport.y, self.viewport.z, self.viewport.w);

            // Execute renderer
            renderer.render(context);
        }
    }

    fn needs_redraw(&self) -> bool {
        // Check if a renderer has been set
        if let Some(ref renderer) = self.renderer {
            renderer.needs_redraw()
        } else {
            false
        }
    }
}
