use std::rc::Rc;

use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

use yage_core::{Context, GL};

pub struct CanvasContext {
    gl: Rc<GL>
}

impl CanvasContext {
    pub fn new(canvas_element: HtmlCanvasElement) -> Result<Self, String> {
        let error_msg = "Failed to get WebGL2 context.";
        let context: WebGl2RenderingContext = canvas_element
            .get_context("webgl2")
            .map_err(|_| error_msg)?
            .ok_or_else(|| error_msg)?
            .dyn_into()
            .map_err(|_| error_msg)?;

        let gl = GL::from_webgl_context(context);

        Ok(Self { gl: Rc::new(gl) })
    }
}

impl Context for CanvasContext {
    fn make_current(&self) {
        // doesn't apply in the browser
    }

    fn swap(&self) {
        // doesn't apply in the browser
    }

    fn gl(&self) -> &Rc<GL> {
        &self.gl
    }
}
