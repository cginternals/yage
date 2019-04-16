use crate::{GL, GlFunctions};

/// Wrapper around an OpenGL vertex array.
pub struct VertexArray<'a> {
    gl: &'a GL,
    array_handle: <GL as GlFunctions>::GlVertexArray,
}

impl<'a> VertexArray<'a> {
    /// Creates a vertex array.
    ///
    /// # Parameters
    /// - `gl`: GL context
    pub fn new(gl: &'a GL) -> Self {
        Self {
            gl,
            array_handle: gl.create_vertex_array()
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlVertexArray {
        &self.array_handle
    }

    /// Binds the vertex array.
    pub fn bind(&self) {
        self.gl.bind_vertex_array(Some(&self.array_handle));
    }

    /// Unbinds the vertex array.
    pub fn unbind(&self) {
        self.gl.bind_vertex_array(None);
    }
}

impl<'a> Drop for VertexArray<'a> {
    fn drop(&mut self) {
        self.gl.delete_vertex_array(&self.array_handle);
    }
}
