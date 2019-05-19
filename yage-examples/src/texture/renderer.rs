use std::f32::consts::PI;

use yage_core::{
    glenum, cgmath, check_error,
    Context, GlFunctions,
    Cube, Transform,
    Texture, TextureLoader,
    BasicMeshRenderer,
    GpuObject, Render, Update, Animation, MeshRenderer,
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    mesh_renderer: BasicMeshRenderer,
    cube: Cube,
    texture: Texture,
    model_matrix: Transform,
    animation: Animation<f32>,
    frame_count: i32,
    redraw: bool
}

impl Renderer {
    ///
    /// Create a renderer.
    ///
    /// # Returns
    /// A new instance of Renderer.
    ///
    pub fn new() -> Renderer {
        Renderer {
            initialized: false,
            mesh_renderer: BasicMeshRenderer::new(),
            cube: Cube::new(),
            texture: Texture::new(gl::TEXTURE_2D),
            model_matrix: Transform::new(),
            animation: Animation::new(0.0, 2.0 * PI, 4.0, true, false, true),
            frame_count: 0,
            redraw: false
        }
    }
}

impl GpuObject for Renderer {
    fn init(&mut self, context: &Context) {
        // Abort if already initialized
        if self.initialized {
            return;
        }

        // [DEBUG]
        println!("initializing renderer");

        // Initialize mesh renderer
        self.mesh_renderer.init(context);

        // Initialize geometry
        self.cube.init(context);

        // Create texture
        self.texture = Texture::new(gl::TEXTURE_2D);
        self.texture.init(context);
        {
            // Load texture
            TextureLoader::load(context, &mut self.texture, "data/duck.jpg");
            check_error!();
        }

        // Done
        self.initialized = true;
    }

    fn deinit(&mut self, context: &Context) {
        // Abort if not initialized
        if !self.initialized {
            return;
        }

        // [DEBUG]
        println!("de-initializing renderer");

        // De-Initialize OpenGL objects
        self.mesh_renderer.init(context);
        self.cube.deinit(context);
        self.texture.deinit(context);
        self.initialized = false;
    }
}

impl Update for Renderer {
    fn needs_update(&self) -> bool {
        self.animation.needs_update()
    }

    fn update(&mut self, time_delta: f64) {
        // Perform animation
        self.animation.update(time_delta);
        let value = self.animation.get_value();

        // Update rotation
        let rotation = cgmath::Quaternion::from(cgmath::Euler {
            x: cgmath::Rad(-0.4),
            y: cgmath::Rad(value),
            z: cgmath::Rad(0.0),
        });
        self.model_matrix.set_rotation(rotation);

        // Redraw
        self.redraw = true;
    }
}

impl Render for Renderer {
    fn set_viewport(&mut self, _viewport: cgmath::Vector4<i32>) {
        // We don't care as the viewport is correctly set by the canvas
    }

    fn needs_redraw(&self) -> bool {
        self.redraw
    }

    fn render(&mut self, context: &Context) {
        // [DEBUG]
        //println!("frame #{}", self.frame_count);
        self.frame_count = self.frame_count + 1;

        // Clear background
        context.gl().clear_color(0.1, 0.2, 0.3, 1.0);
        context.gl().clear(glenum::BufferBit::Color as u32 | glenum::BufferBit::Depth as u32);
        check_error!();

        // Bind texture
        self.texture.bind_active(context, 0);
        check_error!();

        // Draw geometry
        self.mesh_renderer.draw(context, &mut self.cube, &self.model_matrix);
    }
}
