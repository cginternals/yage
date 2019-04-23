use std::rc::Rc;

use crate::{GlFunctions, GL};

/// Wrapper around an OpenGL texture.
pub struct Texture {
    gl: Rc<GL>,
    /// Target for use in `glBindTexture`
    target: u32,
    handle: <GL as GlFunctions>::GlTexture,
}

impl Texture {
    /// Creates an texture.
    ///
    /// # Parameters
    /// - `gl`: GL context
    /// - `target`: must be a valid glenum for `glBindTexture`
    pub fn new(gl: &Rc<GL>, target: u32) -> Self {
        Self {
            gl: gl.clone(),
            target,
            handle: gl.create_texture(),
        }
    }

    /// Getter for the OpenGL/WebGL handle
    pub fn handle(&self) -> &<GL as GlFunctions>::GlTexture {
        &self.handle
    }

    /// Binds the texture.
    pub fn bind(&self) {
        self.gl.bind_texture(self.target, Some(&self.handle));
    }

    /// Unbinds the texture.
    pub fn unbind(&self) {
        self.gl.bind_texture(self.target, None);
    }

    /// Sets the texture object's magnification and minification filter.
    ///
    /// # Parameters:
    /// - `mag`: Value for the TEXTURE_MAG_FILTER parameter
    /// - `min`: Value for the TEXTURE_MIN_FILTER parameter
    pub fn filter(&self, mag: i32, min: i32) {
        self.gl.tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureMagFilter as _,
            mag,
        );
        self.gl.tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureMinFilter as _,
            min,
        );
    }

    // TODO!!: Option<wrap_r> or seperate 3D texture object?
    /// Sets the texture object's wrapping function for s and t coordinates.
    ///
    /// # Parameters:
    /// - `wrap_s`: Value for the TEXTURE_WRAP_S parameter
    /// - `wrap_t`: Value for the TEXTURE_WRAP_T parameter
    pub fn wrap(&self, wrap_s: i32, wrap_t: i32) {
        self.gl.tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureWrapS as _,
            wrap_s,
        );
        self.gl.tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureWrapT as _,
            wrap_t,
        );
    }

    /// Pass image data to the texture object.
    ///
    /// # Parameters:
    /// - `level`: level-of-detail number
    /// - `internal_format`:
    /// - `width`:
    /// - `height`:
    /// - `border`:
    /// - `format`:
    /// - `ty`: type
    /// - `pixels`: pixel data
    #[allow(clippy::too_many_arguments)]
    pub fn image_2d(
        &self,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        self.gl.tex_image_2d(
            self.target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            pixels,
        );
    }

    pub fn generate_mipmap(&self) {
        self.gl.generate_mipmap(self.target);
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        self.gl.delete_texture(&self.handle);
    }
}
