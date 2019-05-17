use cgmath::{
    Vector2, Vector3, Vector4,
    Matrix2, Matrix3, Matrix4
};

use crate::{
    Context,
    GL, GlFunctions,
};

///
/// Represents a type that can be set as a uniform on a shader program.
///
pub trait Uniform<T> {
    ///
    /// Set uniform value
    ///
    /// # Parameters
    /// - `context`: Active OpenGL context
    /// - `location`: Uniform location
    /// - `value`: Uniform value
    ///
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: T
    );
}

// Implementation for bool
impl Uniform<bool> for bool {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: bool,
    ) {
        context.gl().uniform_1i(location, value as i32);
    }
}

// Implementation for i32
impl Uniform<i32> for i32 {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: i32,
    ) {
        context.gl().uniform_1i(location, value);
    }
}

// Implementation for u32
impl Uniform<u32> for u32 {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: u32,
    ) {
        context.gl().uniform_1ui(location, value);
    }
}

// Implementation for f32
impl Uniform<f32> for f32 {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: f32,
    ) {
        context.gl().uniform_1f(location, value);
    }
}

// Implementation for (i32, i32)
impl Uniform<(i32, i32)> for (i32, i32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (i32, i32),
    ) {
        context.gl().uniform_2i(location, value.0, value.1);
    }
}

// Implementation for (u32, u32)
impl Uniform<(u32, u32)> for (u32, u32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (u32, u32),
    ) {
        context.gl().uniform_2ui(location, value.0, value.1);
    }
}

// Implementation for (f32, f32)
impl Uniform<(f32, f32)> for (f32, f32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (f32, f32),
    ) {
        context.gl().uniform_2f(location, value.0, value.1);
    }
}

// Implementation for (i32, i32, i32)
impl Uniform<(i32, i32, i32)> for (i32, i32, i32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (i32, i32, i32),
    ) {
        context.gl().uniform_3i(location, value.0, value.1, value.2);
    }
}

// Implementation for (u32, u32, u32)
impl Uniform<(u32, u32, u32)> for (u32, u32, u32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (u32, u32, u32),
    ) {
        context.gl().uniform_3ui(location, value.0, value.1, value.2);
    }
}

// Implementation for (f32, f32, f32)
impl Uniform<(f32, f32, f32)> for (f32, f32, f32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (f32, f32, f32),
    ) {
        context.gl().uniform_3f(location, value.0, value.1, value.2);
    }
}

// Implementation for (i32, i32, i32, i32)
impl Uniform<(i32, i32, i32, i32)> for (i32, i32, i32, i32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (i32, i32, i32, i32),
    ) {
        context.gl().uniform_4i(location, value.0, value.1, value.2, value.3);
    }
}

// Implementation for (u32, u32, u32, u32)
impl Uniform<(u32, u32, u32, u32)> for (u32, u32, u32, u32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (u32, u32, u32, u32),
    ) {
        context.gl().uniform_4ui(location, value.0, value.1, value.2, value.3);
    }
}

// Implementation for (f32, f32, f32, f32)
impl Uniform<(f32, f32, f32, f32)> for (f32, f32, f32, f32) {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: (f32, f32, f32, f32),
    ) {
        context.gl().uniform_4f(location, value.0, value.1, value.2, value.3);
    }
}

// Implementation for [i32; 2]
impl Uniform<&[i32; 2]> for &[i32; 2] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[i32; 2],
    ) {
        <(i32, i32)>::set_uniform(context, location, (value[0], value[1]));
    }
}

// Implementation for [u32; 2]
impl Uniform<&[u32; 2]> for &[u32; 2] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[u32; 2],
    ) {
        <(u32, u32)>::set_uniform(context, location, (value[0], value[1]));
    }
}

// Implementation for [f32; 2]
impl Uniform<&[f32; 2]> for &[f32; 2] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[f32; 2],
    ) {
        <(f32, f32)>::set_uniform(context, location, (value[0], value[1]));
    }
}

// Implementation for [i32; 3]
impl Uniform<&[i32; 3]> for &[i32; 3] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[i32; 3],
    ) {
        <(i32, i32, i32)>::set_uniform(context, location, (value[0], value[1], value[2]));
    }
}

// Implementation for [u32; 3]
impl Uniform<&[u32; 3]> for &[u32; 3] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[u32; 3],
    ) {
        <(u32, u32, u32)>::set_uniform(context, location, (value[0], value[1], value[2]));
    }
}

// Implementation for [f32; 3]
impl Uniform<&[f32; 3]> for &[f32; 3] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[f32; 3],
    ) {
        <(f32, f32, f32)>::set_uniform(context, location, (value[0], value[1], value[2]));
    }
}

// Implementation for [i32; 4]
impl Uniform<&[i32; 4]> for &[i32; 4] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[i32; 4],
    ) {
        <(i32, i32, i32, i32)>::set_uniform(context, location, (value[0], value[1], value[2], value[3]));
    }
}

// Implementation for [u32; 4]
impl Uniform<&[u32; 4]> for &[u32; 4] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[u32; 4],
    ) {
        <(u32, u32, u32, u32)>::set_uniform(context, location, (value[0], value[1], value[2], value[3]));
    }
}

// Implementation for [f32; 4]
impl Uniform<&[f32; 4]> for &[f32; 4] {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &[f32; 4],
    ) {
        <(f32, f32, f32, f32)>::set_uniform(context, location, (value[0], value[1], value[2], value[3]));
    }
}

// Implementation for Vector2<i32>
impl Uniform<&Vector2<i32>> for &Vector2<i32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector2<i32>,
    ) {
        <&[i32; 2]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector2<u32>
impl Uniform<&Vector2<u32>> for &Vector2<u32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector2<u32>,
    ) {
        <&[u32; 2]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector2<f32>
impl Uniform<&Vector2<f32>> for &Vector2<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector2<f32>,
    ) {
        <&[f32; 2]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector3<i32>
impl Uniform<&Vector3<i32>> for &Vector3<i32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector3<i32>,
    ) {
        <&[i32; 3]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector3<u32>
impl Uniform<&Vector3<u32>> for &Vector3<u32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector3<u32>,
    ) {
        <&[u32; 3]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector3<f32>
impl Uniform<&Vector3<f32>> for &Vector3<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector3<f32>,
    ) {
        <&[f32; 3]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector4<i32>
impl Uniform<&Vector4<i32>> for &Vector4<i32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector4<i32>,
    ) {
        <&[i32; 4]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector4<u32>
impl Uniform<&Vector4<u32>> for &Vector4<u32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector4<u32>,
    ) {
        <&[u32; 4]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Vector4<f32>
impl Uniform<&Vector4<f32>> for &Vector4<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Vector4<f32>,
    ) {
        <&[f32; 4]>::set_uniform(context, location, value.as_ref());
    }
}

// Implementation for Matrix2<f32>
impl Uniform<&Matrix2<f32>> for &Matrix2<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Matrix2<f32>,
    ) {
        context.gl().uniform_matrix_2fv(location, value.as_ref());
    }
}

// Implementation for Matrix3<f32>
impl Uniform<&Matrix3<f32>> for &Matrix3<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Matrix3<f32>,
    ) {
        context.gl().uniform_matrix_3fv(location, value.as_ref());
    }
}

// Implementation for Matrix4<f32>
impl Uniform<&Matrix4<f32>> for &Matrix4<f32> {
    fn set_uniform(
        context: &Context,
        location: &<GL as GlFunctions>::GlUniformLocation,
        value: &Matrix4<f32>,
    ) {
        context.gl().uniform_matrix_4fv(location, value.as_ref());
    }
}
