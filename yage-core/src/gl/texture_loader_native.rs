use crate::{
    Context,
    Texture
};

extern crate image;
use image::GenericImageView;

#[derive(Default)]
pub struct TextureLoader {
}

impl TextureLoader {
    pub fn load(context: &Context, texture: &mut Texture, path: &str) {
        // Load image
        let image_file = image::open(path);
        match image_file {
            Err(err) => panic!("Could not load image: {}", err),
            Ok(img) => {
                // Flip vertically
                let flipped = img.flipv();

                // Get image data
                let data = flipped.raw_pixels();

                // Bind texture
                texture.bind_active(context, 0);

                // Upload image data
                texture.set_image_2d(
                    context,
                    0,
                    gl::RGB as _,
                    flipped.width() as i32,
                    flipped.height() as i32,
                    0,
                    gl::RGB,
                    gl::UNSIGNED_BYTE,
                    Some(&data)
                );

                // Create mip maps
                texture.generate_mipmap(context);
            }
        }
    }
}
