use std::f32::consts::PI;

use yage_core::{
    glenum, cgmath, check_error,
    Context, GlFunctions,
    Cube, Transform, Camera,
    Texture, TextureLoader,
    BasicMeshRenderer,
    GpuObject, Render, Update, Animation, MeshRenderer,
};

///
/// Example renderer that renders a single triangle.
///
pub struct Renderer {
    initialized: bool,
    camera: Camera,
    mesh_renderer: BasicMeshRenderer,
    cube: Cube,
    texture: Texture,
    transform: Transform,
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
        // Initialize camera
        let mut camera = Camera::new();
        camera.look_at(
            cgmath::Vector3::new(0.0, 3.0, 3.0),
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 1.0, 0.0)
        );
        camera.perspective_fov_aspect(
            PI / 4.0,
            4.0 / 3.0,
            0.1,
            64.0
        );

        // Create renderer
        Renderer {
            initialized: false,
            camera,
            mesh_renderer: BasicMeshRenderer::new(),
            cube: Cube::new(),
            texture: Texture::new(glenum::TEXTURE_2D),
            transform: Transform::new(),
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
        self.texture = Texture::new(glenum::TEXTURE_2D);
        self.texture.init(context);
        {
            // Load texture
            TextureLoader::load(context, &mut self.texture, "data/rust.jpg");
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
            x: cgmath::Rad(0.0),
            y: cgmath::Rad(value),
            z: cgmath::Rad(0.0),
        });
        self.transform.set_rotation(rotation);

        // Redraw
        self.redraw = true;
    }
}

impl Render for Renderer {
    fn set_viewport(&mut self, viewport: cgmath::Vector4<i32>) {
        // Update camera projection
        self.camera.perspective_fov(
            PI / 4.0,
            viewport.z as f32,
            viewport.w as f32,
            0.1,
            64.0
        );
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
        context.gl().clear(glenum::COLOR_BUFFER_BIT | glenum::DEPTH_BUFFER_BIT);
        check_error!();

        // Bind texture
        self.texture.bind_active(context, 0);
        check_error!();

        // Draw geometry
        self.mesh_renderer.draw(context, &self.camera, self.cube.geometry_mut(), &self.transform);
    }
}
