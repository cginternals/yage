use std::rc::Rc;

use cgmath::{Vector4};

use crate::Context;
use crate::GpuObject;
use crate::Renderer;
use crate::GL;
use crate::GlFunctions;

///
/// A canvas represents an area into which can be rendered.
///
pub struct Canvas {
    gl: Rc<GL>,
    viewport: Vector4<i32>,
    renderer: Option<Box<dyn Renderer>>
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
            renderer: None
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
    /// - `renderer`: Renderer that will draw into the canvas.
    ///
    pub fn set_renderer<T: 'static + Renderer>(&mut self, renderer: T) {
        self.renderer = Some(Box::new(renderer));
    }
}

impl GpuObject for Canvas {
    fn is_initialized(&self) -> bool {
        false
    }

    fn init(&mut self, context: &Context) {
        println!("initializing canvas");

        if let Some(ref mut renderer) = self.renderer {
            println!("initializing renderer");
            renderer.init(context);
        }
    }

    fn deinit(&mut self, context: &Context) {
        println!("de-initializing canvas");

        if let Some(ref mut renderer) = self.renderer {
            println!("de-initializing renderer");
            renderer.deinit(context);
        }
    }
}

impl Renderer for Canvas {
    fn render(&mut self, context: &Context) {
        println!("render canvas");

        if let Some(ref mut renderer) = self.renderer {
            if !renderer.is_initialized() {
                println!("initializing renderer late");
                renderer.init(context);
            }

            self.gl.viewport(self.viewport.x, self.viewport.y, self.viewport.z, self.viewport.w);
            renderer.render(context);
        }
    }
}
