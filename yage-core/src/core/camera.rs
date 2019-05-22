extern crate toolshed;
use toolshed::CopyCell;

use cgmath::{
    Rad,
    Point3, Vector3, Matrix3, Matrix4,
    Ortho, Perspective, PerspectiveFov
};
use cgmath::prelude::Matrix;
use cgmath::prelude::SquareMatrix;
use cgmath::prelude::EuclideanSpace;

///
/// Represents matrices for a camera within a virtual scene.
///
/// The camera provides access to view and projection matrices that can be
/// configured as well as accessors for derived inverted matrices, their
/// products and a normal matrix.
///
#[derive(Copy, Clone)]
pub struct Camera {
    view: Matrix4<f32>, // View matrix
    projection: Matrix4<f32>, // Projection matrix
    view_inverted: CopyCell<Option<Matrix4<f32>>>, // Inverted view matrix
    projection_inverted: CopyCell<Option<Matrix4<f32>>>, // Inverted projection matrix
    view_projection: CopyCell<Option<Matrix4<f32>>>, // View-projection matrix
    view_projection_inverted: CopyCell<Option<Matrix4<f32>>>, // Inverted view-projection matrix
    normal: CopyCell<Option<Matrix3<f32>>> // Normal matrix
}

impl Camera {
    ///
    /// Create camera.
    ///
    /// # Returns
    /// A new instance of Camera.
    ///
    pub fn new() -> Self {
        Self {
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            view_inverted: CopyCell::new(None),
            projection_inverted: CopyCell::new(None),
            view_projection: CopyCell::new(None),
            view_projection_inverted: CopyCell::new(None),
            normal: CopyCell::new(None)
        }
    }

    ///
    /// Get view matrix.
    ///
    /// # Returns
    /// View matrix.
    ///
    pub fn view_matrix(&self) -> Matrix4<f32> {
        // Return view matrix
        self.view
    }

    ///
    /// Set view matrix.
    ///
    /// # Parameters
    /// - `view`: View matrix
    ///
    pub fn set_view_matrix(&mut self, view: Matrix4<f32>) {
        // Set view matrix
        self.view = view;

        // Reset cached values
        self.invalidate();
    }

    ///
    /// Get projection matrix.
    ///
    /// # Returns
    /// Projection matrix.
    ///
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        // Return projection matrix
        self.projection
    }

    ///
    /// Set projection matrix.
    ///
    /// # Parameters
    /// - `Projection`: Projection matrix
    ///
    pub fn set_projection_matrix(&mut self, projection: Matrix4<f32>) {
        // Set projection matrix
        self.projection = projection;

        // Reset cached values
        self.invalidate();
    }

    ///
    /// Get invertex view matrix.
    ///
    /// # Returns
    /// Inverted view matrix.
    ///
    pub fn inverted_view_matrix(&self) -> Matrix4<f32> {
        // Lazy calculation of value
        if !self.view_inverted.get().is_some() {
            self.view_inverted.set(self.view.invert());
        }

        // Return inverted view matrix
        if let Some(view_inverted) = self.view_inverted.get() {
            view_inverted
        } else {
            Matrix4::identity()
        }
    }

    ///
    /// Get invertex projection matrix.
    ///
    /// # Returns
    /// Inverted projection matrix.
    ///
    pub fn inverted_projection_matrix(&self) -> Matrix4<f32> {
        // Lazy calculation of value
        if !self.projection_inverted.get().is_some() {
            self.projection_inverted.set(self.projection.invert());
        }

        // Return inverted projection matrix
        if let Some(projection_inverted) = self.projection_inverted.get() {
            projection_inverted
        } else {
            Matrix4::identity()
        }
    }

    ///
    /// Get view-projection matrix.
    ///
    /// # Returns
    /// View-projection matrix.
    ///
    pub fn view_projection_matrix(&self) -> Matrix4<f32> {
        // Lazy calculation of value
        if !self.view_projection.get().is_some() {
            self.view_projection.set(Some(self.projection * self.view));
        }

        // Return view-projection matrix
        if let Some(view_projection) = self.view_projection.get() {
            view_projection
        } else {
            Matrix4::identity()
        }
    }

    ///
    /// Get inverted view-projection matrix.
    ///
    /// # Returns
    /// Inverted view-projection matrix.
    ///
    pub fn inverted_view_projection_matrix(&self) -> Matrix4<f32> {
        // Lazy calculation of value
        if !self.view_projection_inverted.get().is_some() {
            self.view_projection_inverted.set(self.view_projection_matrix().invert());
        }

        // Return inverted view-projection matrix
        if let Some(view_projection_inverted) = self.view_projection_inverted.get() {
            view_projection_inverted
        } else {
            Matrix4::identity()
        }
    }

    ///
    /// Get normal matrix.
    ///
    /// # Returns
    /// Normal matrix.
    ///
    pub fn normal_matrix(&self) -> Matrix3<f32> {
        // Lazy calculation of value
        if !self.normal.get().is_some() {
            let normal = Matrix3::from_cols(
                self.view.x.truncate(),
                self.view.y.truncate(),
                self.view.z.truncate()
            );
            if let Some(invert) = normal.invert() {
                self.normal.set(Some(invert.transpose()));
            }
        }

        // Return normal matrix
        if let Some(normal) = self.normal.get() {
            normal
        } else {
            Matrix3::identity()
        }
    }

    ///
    /// Set a look-at view matrix.
    ///
    /// # Parameters
    /// - `eye`: Position of the camera
    /// - `center`: Position of the center the camera looks at
    /// - `up`: Up-vector
    ///
    pub fn look_at(&mut self, eye: Vector3<f32>, center: Vector3<f32>, up: Vector3<f32>)
    {
        self.set_view_matrix(
            Matrix4::look_at(
                Point3::from_vec(eye),
                Point3::from_vec(center),
                up
            )
        );
    }

    ///
    /// Set perspective projection matrix
    ///
    /// # Parameters
    /// - `left`: Viewport left
    /// - `right`: Viewport right
    /// - `bottom`: Viewport bottom
    /// - `top`: Viewport top
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn perspective(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        // Create perspective projection matrix
        let perspective = Perspective {
            left,
            right,
            bottom,
            top,
            near,
            far
        };

        // Set projection matrix
        self.set_projection_matrix(Matrix4::from(perspective));
    }

    ///
    /// Set perspective projection matrix
    ///
    /// # Parameters
    /// - `fovy`: Field-of-view angle
    /// - `width`: Viewport width
    /// - `height`: Viewport height
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn perspective_fov(&mut self, fovy: f32, width: f32, height: f32, near: f32, far: f32) {
        // Calculate aspect ratio automatically
        self.perspective_fov_aspect(fovy, (width / height).max(1.0), near, far);
    }

    ///
    /// Set perspective projection matrix
    ///
    /// # Parameters
    /// - `fovy`: Field-of-view angle
    /// - `aspect`: Aspect ratio
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn perspective_fov_aspect(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        // Create perspective projection matrix
        let perspective = PerspectiveFov {
            fovy: Rad(fovy),
            aspect,
            near,
            far
        };

        // Set projection matrix
        self.set_projection_matrix(Matrix4::from(perspective));
    }

    ///
    /// Set orthographic projection matrix
    ///
    /// # Parameters
    /// - `left`: Viewport left
    /// - `right`: Viewport right
    /// - `bottom`: Viewport bottom
    /// - `top`: Viewport top
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn orthographic(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        // Create orthographic projection matrix
        let ortho = Ortho {
            left,
            right,
            bottom,
            top,
            near,
            far
        };

        // Set projection matrix
        self.set_projection_matrix(Matrix4::from(ortho));
    }

    ///
    /// Set orthographic projection matrix
    ///
    /// # Parameters
    /// - `fovy`: Field-of-view angle
    /// - `width`: Viewport width
    /// - `height`: Viewport height
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn orthographic_fov(&mut self, fovy: f32, width: f32, height: f32, near: f32, far: f32) {
        // Calculate aspect ratio automatically
        self.orthographic_fov_aspect(fovy, (width / height).max(1.0), near, far);
    }

    ///
    /// Set orthographic projection matrix
    ///
    /// # Parameters
    /// - `fovy`: Field-of-view angle
    /// - `aspect`: Aspect ratio
    /// - `near`: Near plane
    /// - `far`: Far plane
    ///
    pub fn orthographic_fov_aspect(&mut self, fovy: f32, aspect: f32, near: f32, far: f32) {
        // Set orthographic projection
        self.orthographic(-fovy * aspect, fovy * aspect, -fovy, fovy, near, far);
    }

    ///
    /// Calculate orthographic projection matrix from perspective parameters
    ///
    /// # Parameters
    /// - `fovy`: Field-of-view angle
    /// - `aspect`: Aspect ratio
    /// - `near`: Near plane
    /// - `far`: Far plane
    /// - `dist`: Distance of the plane that should keep constant size in both projections ([near, far])
    ///
    pub fn orthographic_from_perspective(
        &mut self,
        fovy: f32, aspect: f32,
        near: f32, far: f32, dist: f32
    ) {
        // Compute ortho params from fovY and aspect, assuming symmetry
        let right = near * (fovy / 2.0).tan();
        let top = right / aspect;
        let left = -right;
        let bottom = -top;
        let c = dist / near;

        // Set orthographic projection
        self.orthographic(c*left, c*right, c*bottom, c*top, near, far);
    }

    ///
    /// Calculate perspective projection matrix from orthographic parameters
    ///
    /// # Parameters
    /// - `left`: Viewport left
    /// - `right`: Viewport right
    /// - `bottom`: Viewport bottom
    /// - `top`: Viewport top
    /// - `near`: Near plane
    /// - `far`: Far plane
    /// - `dist`: Distance of the plane that should keep constant size in both projections ([near, far])
    ///
    pub fn perspective_from_orthographic(
        &mut self,
        _left: f32, right: f32, top: f32, _bottom: f32,
        near: f32, far: f32, dist: f32
    ) {
        // Compute perspective projection params
        let c = near / dist;
        let r = c * right;
        let t = c * top;
        let fovy = (r / near).atan() * 2.0; //assuming right = -left and bottom = -top
        let aspect = (2.0 * r) / (2.0 * t);

        // Set perspective projection
        self.perspective_fov_aspect(fovy, aspect, near, far);
    }

    ///
    /// Invalidate all calculated matrices.
    ///
    fn invalidate(&self) {
        self.view_inverted.set(None);
        self.projection_inverted.set(None);
        self.view_projection.set(None);
        self.view_projection_inverted.set(None);
        self.normal.set(None);
    }
}
