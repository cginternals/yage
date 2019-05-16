use std::rc::Rc;

use crate::{Context, GL, GlFunctions, GpuObject};

///
/// Represents a texture on the GPU.
///
pub struct Texture {
    gl: Rc<GL>,
    target: u32,
    handle: Option<<GL as GlFunctions>::GlTexture>,
}

impl Texture {
    ///
    /// Create a texture instance.
    ///
    /// # Parameters
    /// - `gl`: GL context
    /// - `target`: Texture target (OpenGL enum, e.g., GL_TEXTURE_2D)
    ///
    /// # Returns
    /// A new instance of Texture.
    ///
    pub fn new(gl: &Rc<GL>, target: u32) -> Self {
        Self {
            gl: gl.clone(),
            target,
            handle: None,
        }
    }

    ///
    /// Get texture handle.
    ///
    /// # Returns
    /// OpenGL texture handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlTexture> {
        self.handle.as_ref()
    }

    ///
    /// Bind the texture.
    ///
    pub fn bind(&self) {
        self.gl.bind_texture(self.target, self.handle.as_ref());
    }

    ///
    /// Unbind the texture.
    ///
    pub fn unbind(&self) {
        self.gl.bind_texture(self.target, None);
    }

    ///
    /// Set texture magnification and minification filters.
    ///
    /// # Parameters:
    /// - `mag`: Value for the TEXTURE_MAG_FILTER parameter
    /// - `min`: Value for the TEXTURE_MIN_FILTER parameter
    ///
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

    /// Sets the texture object's wrapping function for s and t coordinates.
    ///
    /// # Parameters:

    ///
    /// Set texture wrapping.
    ///
    /// # Parameters:
    /// - `wrap_s`: Value for the TEXTURE_WRAP_S parameter
    /// - `wrap_t`: Value for the TEXTURE_WRAP_T parameter
    ///
    /// [TODO] Option<wrap_r> or seperate 3D texture object?
    ///
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

    ///
    /// Set 2D image data.
    ///
    /// # Parameters:
    /// - `level`: level-of-detail number
    /// - `internal_format`: Internal data format
    /// - `width`: Texture width (in pixels)
    /// - `height`: Texture height (in pixels)
    /// - `border`: Must be 0
    /// - `format`: Data format
    /// - `data_type`: Data type
    /// - `pixels`: pixel data
    ///
    #[allow(clippy::too_many_arguments)]
    pub fn set_image_2d(
        &self,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        data_type: u32,
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
            data_type,
            pixels,
        );
    }

    ///
    /// Generate mipmap data.
    ///
    pub fn generate_mipmap(&self) {
        self.gl.generate_mipmap(self.target);
    }
}

impl GpuObject for Texture {
    fn init(&mut self, _context: &Context) {
        self.handle = Some(self.gl.create_texture());
    }

    fn deinit(&mut self, _context: &Context) {
        if let Some(ref handle) = self.handle {
            self.gl.delete_texture(handle);
            self.handle = None;
        }
    }
}
