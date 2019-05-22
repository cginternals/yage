use std::rc::Rc;
use std::time::Instant;

use cgmath::Vector4;

use crate::Context;
use crate::GpuObject;
use crate::Render;
use crate::Update;
use crate::GL;
use crate::GlFunctions;

///
/// A canvas represents an area into which can be rendered.
///
/// The `Canvas` is created by the windowing backend, such as [`yage-glutin`]
/// or [`yage-web`]. It usually belongs to a visible item, e.g., a window
/// or a DOM element, on which the rendering result will be shown. It can
/// also belong to an offscreen context. The windowing backend is responsible
/// for initializing and de-initializing the OpenGL context on the `Canvas`,
/// setting its size, and calling the update, render, and input functions
/// at the appropriate time.
///
/// A `Canvas` owns exactly one [`Render`] object, which is responsible for
/// rendering into the `Canvas`. It can be set with [`set_renderer()`].
/// The `Canvas` acts as a proxy to the renderer: all methods, which are
/// defined in the [`Render`] and [`GpuObject`] traits, are forwarded
/// from the `Canvas` to the renderer.
///
/// [`yage-glutin`]: ../yage_glutin/index.html
/// [`yage-web`]: ../yage_web/index.html
/// [`Render`]: trait.Render.html
/// [`GpuObject`]: trait.GpuObject.html
/// [`set_renderer()`]: struct.Canvas.html#method.set_renderer
///
pub struct Canvas {
    gl: Rc<GL>,
    viewport: Vector4<i32>,
    renderer: Option<Box<dyn Render>>,
    new_renderer: Option<Box<dyn Render>>,
    renderer_initialized: bool,
    time: Instant,
    time_delta: f64
}

impl Canvas {
    ///
    /// Create a canvas.
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
            renderer_initialized: false,
            time: Instant::now(),
            time_delta: 0.0
        }
    }

    ///
    /// Get viewport.
    ///
    /// # Returns
    /// Size of viewport in device coordinates.
    ///
    pub fn get_viewport(&self) -> Vector4<i32> {
        self.viewport
    }

    ///
    /// Set renderer that will draw into the canvas.
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

    ///
    /// Update timing information.
    ///
    /// When this function is called, the elapsed time since the last call to
    /// [`update`] is calculated. Since `update_time` might be called several
    /// times before `update`, the time delta is accumulated until the canvas
    /// is actually updated, and then reset in `update`.
    ///
    /// [`update`]: trait.Render.html#tymethod.update
    ///
    pub fn update_time(&mut self) {
        // Get number of milliseconds since last call
        let nanos = self.time.elapsed().as_nanos();
        self.time = Instant::now();

        // Determine and update time delta
        let time_delta = nanos as f64 / 1000000000.0;
        self.time_delta = self.time_delta + time_delta;
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

impl Update for Canvas {
    fn needs_update(&self) -> bool {
        // Check if a renderer has been set
        if let Some(ref renderer) = self.renderer {
            renderer.needs_update()
        } else {
            false
        }
    }

    fn update(&mut self, _time_delta: f64) {
        // Check if a renderer has been set
        if let Some(ref mut renderer) = self.renderer {
            // Update renderer
            renderer.update(self.time_delta);
        }

        // Reset time delta
        self.time_delta = 0.0;
    }
}

impl Render for Canvas {
    fn set_viewport(&mut self, viewport: Vector4<i32>) {
        // Save viewport
        self.viewport = viewport;

        // Inform renderer
        if let Some(ref mut renderer) = self.renderer {
            renderer.set_viewport(viewport)
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
}
