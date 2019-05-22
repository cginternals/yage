extern crate toolshed;
use toolshed::CopyCell;

use cgmath::{Vector3, Matrix4, Quaternion};
use cgmath::prelude::One;

///
/// Representation of a 3D transformation (translation, rotation, and scale)
///
#[derive(Copy, Clone)]
pub struct Transform {
    rotation: Quaternion<f32>,
    translation: Vector3<f32>,
    scale: Vector3<f32>,
    transform: CopyCell<Option<Matrix4<f32>>>
}

impl Transform {
    ///
    /// Create identity transform.
    ///
    /// # Returns
    /// A new instance of Transform.
    ///
    pub fn new() -> Self {
        Self {
            rotation: Quaternion::one(),
            translation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
            transform: CopyCell::new(None)
        }
    }

    ///
    /// Get rotation.
    ///
    /// # Returns
    /// Rotation quaternion.
    ///
    pub fn rotation(&self) -> Quaternion<f32> {
        self.rotation
    }

    ///
    /// Set rotation.
    ///
    /// # Parameters
    /// - `rotation`: Rotation quaternion
    ///
    pub fn set_rotation(&mut self, rotation: Quaternion<f32>) {
        self.rotation = rotation;
        self.transform.set(None);
    }

    ///
    /// Get translation.
    ///
    /// # Returns
    /// Translation vector.
    ///
    pub fn translation(&self) -> Vector3<f32> {
        self.translation
    }

    ///
    /// Set translation.
    ///
    /// # Parameters
    /// - `translation`: Translation vector
    ///
    pub fn set_translation(&mut self, translation: Vector3<f32>) {
        self.translation = translation;
        self.transform.set(None);
    }

    ///
    /// Get scale.
    ///
    /// # Returns
    /// Scale vector.
    ///
    pub fn scale(&self) -> Vector3<f32> {
        self.scale
    }

    ///
    /// Set scale.
    ///
    /// # Parameters
    /// - `scale`: Scale vector
    ///
    pub fn set_scale(&mut self, scale: Vector3<f32>) {
        self.scale = scale;
        self.transform.set(None);
    }

    ///
    /// Get transformation matrix.
    ///
    /// # Returns
    /// Transformation matrix.
    ///
    pub fn transform(&self) -> Matrix4<f32> {
        // Check if transformation needs to be updated
        if !self.transform.get().is_some() {
            self.update_transform();
        }

        // Return transformation
        self.transform.get().unwrap()
    }

    ///
    /// Update transformation matrix
    ///
    pub fn update_transform(&self) {
        // Get transformation components
        let rotation: Matrix4<f32> = Matrix4::from(self.rotation);
        let translation: Matrix4<f32> = Matrix4::from_translation(self.translation);
        let scale: Matrix4<f32> = Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z);

        // Calculate transformation matrix
        self.transform.set(Some(rotation * translation * scale));
    }
}
