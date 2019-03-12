use glenum::*;
// use glutin::Context

pub type Reference = u32;

#[derive(Default)]
pub struct GL {
    ///// openGL internal reference
    // pub reference: Reference,
    ///// whether this context is a WebGL 2.0 context
    // pub is_webgl2: bool,
}

impl GL {
    pub fn new() -> GL {
        // println!("opengl {}", get_string(gl::VERSION));
        // println!(
        //     "shading language {}",
        //     get_string(gl::SHADING_LANGUAGE_VERSION)
        // );
        // println!("vendor {}", get_string(gl::VENDOR));
        GL {
            // reference: 0,
            // is_webgl2: true,
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
