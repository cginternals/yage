use glenum::*;

#[derive(Default)]
pub struct GL {
}

impl GL {
    pub fn new() -> GL {
        GL {
        }
    }

    /// specify clear values for the color buffers
    pub fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    /// clear buffers to preset values
    pub fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
    }
}
