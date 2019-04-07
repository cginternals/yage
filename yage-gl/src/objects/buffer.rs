use crate::{GL, GlFunctions};

/// Wrapper around an OpenGL array or element array buffer. 
pub struct Buffer<'a> {
    gl: &'a GL,
    /// Target for use in `glBindBuffer`
    target: u32,
    buffer_handle: <GL as GlFunctions>::GlBuffer,
}

impl<'a> Buffer<'a> {
    /// Creates an empty buffer. 
    /// 
    /// # Parameters
    /// - `gl`: GL context
    /// - `target`: must be a valid glenum for `glBindBuffer`
    pub fn new(gl: &'a GL, target: u32) -> Self {
        Self {
            gl,
            target,
            buffer_handle: gl.create_buffer()
        }
    }

    /// Creates the buffer object's data store.
    /// 
    /// # Parameters
    /// - `data`: buffer data
    /// - `usage`: must be a valid glenum for `glBufferData`
    pub fn set_data<T>(&self, data: &[T], usage: u32) {
        self.gl.buffer_data(self.target, data, usage);
    }

    // TODO!!
    pub fn set_sub_data() {
        unimplemented!()
    }

    /// Binds the buffer.
    pub fn bind(&self) {
        self.gl.bind_buffer(self.target, Some(&self.buffer_handle));
    }

    /// Unbinds the buffer.
    pub fn unbind(&self) {
        self.gl.bind_buffer(self.target, None);
    }

    // TODO!!: see webgl-operate
    pub fn attrib_enable() {
        unimplemented!()
    }

    // TODO!!: see webgl-operate
    pub fn attrib_disable() {
        unimplemented!()
    }
}

impl<'a> Drop for Buffer<'a> {
    fn drop(&mut self) {
        self.gl.delete_buffer(&self.buffer_handle);
    }
}

