use cgmath::{Vector2};

use crate::Renderer;

///
/// A canvas represents an area into which can be rendered.
///
pub struct Canvas {
    size: Vector2<f32>,
    renderer: Option<Box<dyn Renderer>>
}

impl Canvas {
    ///
    /// Create a canvas instance
    ///
    /// # Returns
    /// A new instance of Canvas.
    ///
    pub fn new() -> Canvas {
        // return canvas
        Canvas {
            size: Vector2::new(0.0, 0.0),
            renderer: None
        }
    }

    ///
    /// Get size
    ///
    /// # Returns
    /// Size of canvas in device coordinates.
    ///
    pub fn get_size(&self) -> Vector2<f32> {
        self.size
    }

    ///
    /// Set size
    ///
    /// # Parameters
    /// - `size`: Size of canvas in device coordinates.
    ///
    pub fn set_size(&mut self, size: Vector2<f32>) {
        self.size = size;
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

impl Default for Canvas {
    fn default() -> Canvas {
        Canvas::new()
    }
}
