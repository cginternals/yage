use cgmath::Vector4;

use crate::{Context, GpuObject, Render, Update};

///
/// Wrapper that enables continuous rendering on a Render object.
///
/// This struct takes a Render object, which might update and render
/// itself in irregular intervals, and renders it continuously.
///
pub struct ContinuousRendering<T: Render> {
    render: T,
    need_update: bool,
    need_redraw: bool
}

impl<T: Render> ContinuousRendering<T> {
    ///
    /// Create renderer.
    ///
    /// # Parameters
    /// - `render`: Render object
    ///
    /// # Returns
    /// A new instance of ContinuousRendering.
    ///
    pub fn new(render: T) -> ContinuousRendering<T> {
        // Return continuous renderer
        ContinuousRendering {
            render,
            need_update: false,
            need_redraw: false
        }
    }
}

impl<T: Render> GpuObject for ContinuousRendering<T> {
    fn init(&mut self, context: &Context) {
        // Initialize render object
        self.render.init(context);
    }

    fn deinit(&mut self, context: &Context) {
        // De-initialize render object
        self.render.deinit(context);
    }
}

impl<T: Render> Update for ContinuousRendering<T> {
    fn needs_update(&self) -> bool {
        self.need_update
    }

    fn update(&mut self, time_delta: f64) {
        // Update render object
        self.render.update(time_delta);

        // Schedule redraw
        self.need_update = false;
        self.need_redraw = true;
    }
}

impl<T: Render> Render for ContinuousRendering<T> {
    fn set_viewport(&mut self, viewport: Vector4<i32>) {
        self.render.set_viewport(viewport);
    }

    fn needs_redraw(&self) -> bool {
        self.need_redraw
    }

    fn render(&mut self, context: &Context) {
        // Draw render object
        self.render.render(context);

        // Schedule update
        self.need_update = true;
        self.need_redraw = false;
    }
}
