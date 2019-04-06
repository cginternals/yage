use glenum::*;

use web_sys::WebGlRenderingContext;

pub struct GL {
    // TODO: WebGl2RenderingContext is a different class - make type generic?
    context: WebGlRenderingContext,
}

impl GL {
    pub fn from_webgl_context(context: WebGlRenderingContext) -> GL {
        GL {
            context
        }
    }
}

#[allow(dead_code)]
impl super::GlFunctions for GL {
    type GlShader = u32;
    type GlProgram = u32;
    type GlBuffer = u32;
    type GlVertexArray = u32;
    type GlTexture = u32;
    type GlUniformLocation = i32;

    /// specify clear values for the color buffers
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.context.clear_color(r, g, b, a);
    }

    /// clear buffers to preset values
    fn clear(&self, bit: BufferBit) {
        self.context.clear(bit as _);
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unimplemented!()
    }

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader {
        unimplemented!()
    }

    fn shader_source(&self, shader: Self::GlShader, source: &str) {
        unimplemented!()
    }

    fn compile_shader(&self, shader: Self::GlShader) {
        unimplemented!()
    }

    fn delete_shader(&self, shader: Self::GlShader) {
        unimplemented!()
    }

    fn get_shader_parameter(&self, shader: Self::GlShader, param: u32) -> i32 {
        unimplemented!()
    }

    fn get_shader_info_log(&self, shader: Self::GlShader) -> String {
        unimplemented!()
    }

    fn create_program(&self) -> Self::GlProgram {
        unimplemented!()
    }

    fn attach_shader(&self, program: Self::GlProgram, shader: Self::GlShader) {
        unimplemented!()
    }

    fn link_program(&self, program: Self::GlProgram) {
        unimplemented!()
    }

    fn get_program_parameter(&self, program: Self::GlProgram, param: u32) -> i32 {
        unimplemented!()
    }

    fn get_program_info_log(&self, program: Self::GlProgram) -> String {
        unimplemented!()
    }

    fn use_program(&self, program: Option<Self::GlProgram>) {
        unimplemented!()
    }

    fn create_buffer(&self) -> Self::GlBuffer {
        unimplemented!()
    }

    fn bind_buffer(&self, target: u32, buffer: Option<Self::GlBuffer>) {
        unimplemented!()
    }

    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32) {
        unimplemented!()
    }

    fn create_vertex_array(&self) -> Self::GlVertexArray {
        unimplemented!()
    }

    fn bind_vertex_array(&self, vertex_array: Option<Self::GlVertexArray>) {
        unimplemented!()
    }

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    ) {
        unimplemented!()
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        unimplemented!()
    }

    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        unimplemented!()
    }

    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        unimplemented!()
    }

    fn enable(&self, param: u32) {
        unimplemented!()
    }

    fn disable(&self, param: u32) {
        unimplemented!()
    }

    fn point_size(&self, size: f32) {
        unimplemented!()
    }

    fn active_texture(&self, unit: u32) {
        unimplemented!()
    }

    fn bind_texture(&self, target: u32, texture: Option<Self::GlTexture>) {
        unimplemented!()
    }

    fn blend_func(&self, src: u32, dst: u32) {
        unimplemented!()
    }

    fn create_texture(&self) -> Self::GlTexture {
        unimplemented!()
    }

    fn tex_image_2d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        ty: u32,
        pixels: Option<&[u8]>,
    ) {
        unimplemented!()
    }

    fn generate_mipmap(&self) {
        unimplemented!()
    }

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32) {
        unimplemented!()
    }

    fn get_uniform_location(
        &self,
        program: Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation {
        unimplemented!()
    }

    fn uniform_1i(&self, location: Self::GlUniformLocation, x: i32) {
        unimplemented!()
    }

    fn uniform_1f(&self, location: Self::GlUniformLocation, x: f32) {
        unimplemented!()
    }

    fn uniform_3fv(&self, location: Self::GlUniformLocation, x: &[f32; 3]) {
        unimplemented!()
    }

    fn uniform_4fv(&self, location: Self::GlUniformLocation, x: &[f32; 4]) {
        unimplemented!()
    }

    fn uniform_2f(&self, location: Self::GlUniformLocation, x: f32, y: f32) {
        unimplemented!()
    }

    fn uniform_3f(&self, location: Self::GlUniformLocation, x: f32, y: f32, z: f32) {
        unimplemented!()
    }

    fn uniform_matrix_4fv(&self, location: Self::GlUniformLocation, mat: &[[f32; 4]; 4]) {
        unimplemented!()
    }
}
