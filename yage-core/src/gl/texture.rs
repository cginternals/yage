use crate::{
    Context,
    GL, GlFunctions,
    GpuObject
};

///
/// Represents a texture on the GPU.
///
pub struct Texture {
    target: u32,
    handle: Option<<GL as GlFunctions>::GlTexture>
}

impl Texture {
    ///
    /// Create a texture.
    ///
    /// # Parameters
    /// - `target`: Texture target (OpenGL enum, e.g., GL_TEXTURE_2D)
    ///
    /// # Returns
    /// A new instance of Texture.
    ///
    pub fn new(target: u32) -> Self {
        Self {
            target,
            handle: None,
        }
    }

    ///
    /// Get texture handle.
    ///
    /// # Returns
    /// OpenGL handle.
    ///
    pub fn handle(&self) -> Option<& <GL as GlFunctions>::GlTexture> {
        self.handle.as_ref()
    }

    ///
    /// Bind texture.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn bind(&self, context: &Context) {
        context.gl().bind_texture(self.target, self.handle.as_ref());
    }

    ///
    /// Bind texture to a specific texture unit
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `unit`: Texture unit
    ///
    pub fn bind_active(&self, context: &Context, unit: u32) {
        context.gl().active_texture(unit);
        context.gl().bind_texture(self.target, self.handle.as_ref());
    }

    ///
    /// Unbind the texture.
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn unbind(&self, context: &Context) {
        context.gl().bind_texture(self.target, None);
    }

    ///
    /// Set texture magnification and minification filters.
    ///
    /// # Parameters:
    /// - `context`: Active OpenGL context
    /// - `mag`: Value for the TEXTURE_MAG_FILTER parameter
    /// - `min`: Value for the TEXTURE_MIN_FILTER parameter
    ///
    pub fn filter(&self, context: &Context, mag: i32, min: i32) {
        context.gl().tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureMagFilter as _,
            mag,
        );
        context.gl().tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureMinFilter as _,
            min,
        );
    }

    ///
    /// Set texture wrapping.
    ///
    /// # Parameters:
    /// - `context`: Active OpenGL context
    /// - `wrap_s`: Value for the TEXTURE_WRAP_S parameter
    /// - `wrap_t`: Value for the TEXTURE_WRAP_T parameter
    ///
    /// [TODO] Option<wrap_r> or seperate 3D texture object?
    ///
    pub fn wrap(&self, context: &Context, wrap_s: i32, wrap_t: i32) {
        context.gl().tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureWrapS as _,
            wrap_s,
        );
        context.gl().tex_parameteri(
            self.target,
            glenum::TextureParameter::TextureWrapT as _,
            wrap_t,
        );
    }

    ///
    /// Set 2D image data.
    ///
    /// # Parameters:
    /// - `context`: Active OpenGL context
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
        context: &Context,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        data_type: u32,
        pixels: Option<&[u8]>,
    ) {
        context.gl().tex_image_2d(
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
    /// # Parameters
    /// - `context`: Active OpenGL context
    ///
    pub fn generate_mipmap(&self, context: &Context) {
        context.gl().generate_mipmap(self.target);
    }
}

impl GpuObject for Texture {
    fn init(&mut self, context: &Context) {
        self.handle = Some(context.gl().create_texture());
    }

    fn deinit(&mut self, context: &Context) {
        if let Some(ref handle) = self.handle {
            context.gl().delete_texture(handle);
            self.handle = None;
        }
    }
}
