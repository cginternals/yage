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
        // return canvas
        Canvas {
            gl: gl.clone(),
            viewport: Vector4::new(0, 0, 0, 0),
            renderer: None,
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
        // store new renderer
        self.renderer = Some(Box::new(renderer));
        self.renderer_initialized = false;
    }
}

impl GpuObject for Canvas {
    fn init(&mut self, context: &Context) {
        // [DEBUG]
        println!("initializing canvas");

        // check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // initialize renderer
            renderer.init(context);
            self.renderer_initialized = true;
        }
    }

    fn deinit(&mut self, context: &Context) {
        // [DEBUG]
        println!("de-initializing canvas");

        // check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // de-initialize renderer
            renderer.deinit(context);
            self.renderer_initialized = false;
        }
    }
}

impl Render for Canvas {
    fn render(&mut self, context: &Context) {
        // check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // check if renderer has been initialized
            if !self.renderer_initialized {
                // initialize renderer
                renderer.init(context);
                self.renderer_initialized = true;
            }

            // set viewport
            self.gl.viewport(self.viewport.x, self.viewport.y, self.viewport.z, self.viewport.w);

            // execute renderer
            renderer.render(context);
        }
    }
}
