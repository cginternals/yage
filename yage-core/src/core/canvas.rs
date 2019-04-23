use std::rc::Rc;

use cgmath::{Vector4};

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
    fn init(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.init();
        }
    }

    fn deinit(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            renderer.deinit();
        }
    }
}

impl Renderer for Canvas {
    fn render(&mut self) {
        if let Some(ref mut renderer) = self.renderer {
            self.gl.viewport(self.viewport.x, self.viewport.y, self.viewport.z, self.viewport.w);
            renderer.render();
        }
    }
}
