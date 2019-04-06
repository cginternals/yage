use glenum::*;

use web_sys::WebGlRenderingContext;

pub struct GL {
    // TODO: WebGl2RenderingContext is a different class - make type generic?
    context: WebGlRenderingContext,
}

impl GL {
    pub fn from_webgl_context(context: WebGlRenderingContext) -> GL {
        GL {
            context
        }
    }

    /// specify clear values for the color buffers
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
    }

        /// clear buffers to preset values
    pub fn clear(&self, bit: BufferBit) {
        self.context.clear(bit as _);
    }
}
