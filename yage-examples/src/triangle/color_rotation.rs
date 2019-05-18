use cgmath::Matrix3;
use cgmath::prelude::SquareMatrix;

///
/// Utility that implements an RGB color rotation.
///
pub struct ColorRotation {
    angle: f32,
    matrix: Matrix3<f32>
}

impl ColorRotation {
    ///
    /// Create color rotation.
    ///
    /// # Returns
    /// A new instance of ColorRotation.
    ///
    pub fn new() -> Self {
        ColorRotation {
            angle: 0.0,
            matrix: Matrix3::identity()
        }
    }

    ///
    /// Get current angle.
    ///
    /// # Returns
    /// Angle (in radians)
    ///
    pub fn get_angle(&self) -> f32 {
        self.angle
    }

    ///
    /// Set current angle.
    ///
    /// # Parameters
    /// - `angle`: Angle (in radians)
    ///
    pub fn set_angle(&mut self, angle: f32) {
        // Set angle
        self.angle = angle;

        // Calculate rotation matrix
        let cos_a = self.angle.cos();
        let sin_a = self.angle.sin();

        self.matrix = Matrix3::new(
            cos_a + (1.0 - cos_a) / 3.0,
            1.0/3.0 * (1.0 - cos_a) - (1.0/3.0 as f32).sqrt() * sin_a,
            1.0/3.0 * (1.0 - cos_a) + (1.0/3.0 as f32).sqrt() * sin_a,
            1.0/3.0 * (1.0 - cos_a) + (1.0/3.0 as f32).sqrt() * sin_a,
            cos_a + 1.0/3.0 * (1.0 - cos_a),
            1.0/3.0 * (1.0 - cos_a) - (1.0/3.0 as f32).sqrt() * sin_a,
            1.0/3.0 * (1.0 - cos_a) - (1.0/3.0 as f32).sqrt() * sin_a,
            1.0/3.0 * (1.0 - cos_a) + (1.0/3.0 as f32).sqrt() * sin_a,
            cos_a + 1.0/3.0 * (1.0 - cos_a)
        );
    }

    ///
    /// Get rotation matrix for the current angle.
    ///
    /// # Returns
    /// Rotation matrix
    ///
    pub fn get_matrix(&self) -> Matrix3<f32> {
        self.matrix
    }
}
