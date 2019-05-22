///
/// Represents an interpolation between two values.
///
/// This trait enables to interpolate between two values of a type.
/// If a type supports Interpolate, it can also be used as a value
/// in an [`Animation`].
///
/// [`Animation`]: struct.Animation.html
///
pub trait Interpolate<T> {
    ///
    /// Get interpolated value
    ///
    /// # Parameters
    /// - `a`: First value
    /// - `b`: Second value
    /// - `t`: Position between a and b ([0.0 .. 1.0])
    ///
    /// # Returns
    /// Interpolated value.
    ///
    fn interpolate(a: T, b: T, t: f32) -> T {
        Self::interpolate_safe(a, b, t.min(1.0).max(0.0))
    }

    ///
    /// Get interpolated value
    ///
    /// This does not perform a check to see if t is between 0.0 and 1.0,
    /// but assumes that it is.
    ///
    /// # Parameters
    /// - `a`: First value
    /// - `b`: Second value
    /// - `t`: Position between a and b ([0.0 .. 1.0])
    ///
    /// # Returns
    /// Interpolated value.
    ///
    fn interpolate_safe(a: T, b: T, t: f32) -> T;
}

impl Interpolate<f32> for f32 {
    fn interpolate_safe(a: f32, b: f32, t: f32) -> f32 {
        a * (1.0 - t) + b * t
    }
}

impl Interpolate<f64> for f64 {
    fn interpolate_safe(a: f64, b: f64, t: f32) -> f64 {
        a * (1.0 - t) as f64 + b * t as f64
    }
}

impl Interpolate<i8> for i8 {
    fn interpolate_safe(a: i8, b: i8, t: f32) -> i8 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as i8
    }
}

impl Interpolate<i16> for i16 {
    fn interpolate_safe(a: i16, b: i16, t: f32) -> i16 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as i16
    }
}

impl Interpolate<i32> for i32 {
    fn interpolate_safe(a: i32, b: i32, t: f32) -> i32 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as i32
    }
}

impl Interpolate<i64> for i64 {
    fn interpolate_safe(a: i64, b: i64, t: f32) -> i64 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as i64
    }
}

impl Interpolate<u8> for u8 {
    fn interpolate_safe(a: u8, b: u8, t: f32) -> u8 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as u8
    }
}

impl Interpolate<u16> for u16 {
    fn interpolate_safe(a: u16, b: u16, t: f32) -> u16 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as u16
    }
}

impl Interpolate<u32> for u32 {
    fn interpolate_safe(a: u32, b: u32, t: f32) -> u32 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as u32
    }
}

impl Interpolate<u64> for u64 {
    fn interpolate_safe(a: u64, b: u64, t: f32) -> u64 {
        ((a as f32) * (1.0 - t) + (b as f32) * t) as u64
    }
}
