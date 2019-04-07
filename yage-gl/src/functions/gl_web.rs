use glenum::*;

use web_sys::{
    WebGl2RenderingContext, 
    WebGlShader,
    WebGlProgram,
    WebGlUniformLocation,
    WebGlTexture,
    WebGlBuffer,
    WebGlVertexArrayObject,
};

pub struct GL {
    // TODO: support WebGL1?
    gl: WebGl2RenderingContext,
}

impl GL {
    pub fn from_webgl_context(context: WebGl2RenderingContext) -> GL {
        GL {
            gl: context
        }
    }
}

impl super::GlFunctions for GL {
    type GlShader = WebGlShader;
    type GlProgram = WebGlProgram;
    type GlBuffer = WebGlBuffer;
    type GlVertexArray = WebGlVertexArrayObject;
    type GlTexture = WebGlTexture;
    type GlUniformLocation = WebGlUniformLocation;

    /// specify clear values for the color buffers
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        self.gl.clear_color(r, g, b, a);
    }

    /// clear buffers to preset values
    fn clear(&self, bit: BufferBit) {
        self.gl.clear(bit as _);
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        self.gl.viewport(x, y, width, height);
    }

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader {
        self.gl.create_shader(kind as _).unwrap()
    }

    fn shader_source(&self, shader: &Self::GlShader, source: &str) {
        self.gl.shader_source(shader, source);
    }

    fn compile_shader(&self, shader: &Self::GlShader) {
        self.gl.compile_shader(shader);
    }

    fn delete_shader(&self, shader: &Self::GlShader) {
        self.gl.delete_shader(Some(shader));
    }

    fn get_shader_parameter(&self, shader: &Self::GlShader, param: u32) -> i32 {
        // TODO!!: multi-return type problem...try in cascade? (as_f64 fails for boolean case)
        self.gl.get_shader_parameter(shader, param).as_bool().unwrap() as i32
    }

    fn get_shader_info_log(&self, shader: &Self::GlShader) -> String {
        self.gl.get_shader_info_log(shader).unwrap()
    }

    fn create_program(&self) -> Self::GlProgram {
        self.gl.create_program().unwrap()
    }

    fn attach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader) {
        self.gl.attach_shader(&program, shader);
    }

    fn link_program(&self, program: &Self::GlProgram) {
        self.gl.link_program(&program);
    }

    fn get_program_parameter(&self, program: &Self::GlProgram, param: u32) -> i32 {
        // TODO!!: see get_shader_parameter....
        self.gl.get_program_parameter(&program, param).as_bool().unwrap() as i32
    }

    fn get_program_info_log(&self, program: &Self::GlProgram) -> String {
        self.gl.get_program_info_log(&program).unwrap()
    }

    fn use_program(&self, program: Option<&Self::GlProgram>) {
        self.gl.use_program(program);
    }

    fn create_buffer(&self) -> Self::GlBuffer {
        self.gl.create_buffer().unwrap()
    }

    fn bind_buffer(&self, target: u32, buffer: Option<&Self::GlBuffer>) {
        self.gl.bind_buffer(target, buffer);
    }

    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32) {
        unsafe {
            self.gl.buffer_data_with_u8_array(
                target, 
                std::slice::from_raw_parts(
                    data.as_ptr() as *const u8, data.len() * std::mem::size_of::<T>()), 
                usage
            );
        }
    }

    fn buffer_sub_data<T>(&self, target: u32, offset: isize, data: &[T]) {
        unsafe {
            self.gl.buffer_sub_data_with_i32_and_u8_array(
                target, 
                offset as i32, 
                std::slice::from_raw_parts_mut(
                    data.as_ptr() as *mut u8, data.len() * std::mem::size_of::<T>()), 
            );
        }
    }

    fn delete_buffer(&self, buffer: &Self::GlBuffer) {
        self.gl.delete_buffer(Some(&buffer));
    }

    fn create_vertex_array(&self) -> Self::GlVertexArray {
        self.gl.create_vertex_array().unwrap()
    }

    fn bind_vertex_array(&self, vertex_array: Option<&Self::GlVertexArray>) {
        self.gl.bind_vertex_array(vertex_array)
    }

    fn delete_vertex_array(&self, vertex_array: &Self::GlVertexArray) {
        self.gl.delete_vertex_array(Some(vertex_array));
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
        self.gl.vertex_attrib_pointer_with_i32(
            index,
            size,
            data_type,
            normalized,
            stride,
            offset
        );
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        self.gl.enable_vertex_attrib_array(index);
    }

    fn disable_vertex_attrib_array(&self, index: u32) {
        self.gl.disable_vertex_attrib_array(index);
    }

    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        self.gl.draw_arrays(mode, first, count);
    }

    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        self.gl.draw_elements_with_i32(mode, count, element_type, offset);
    }

    fn enable(&self, param: u32) {
        self.gl.enable(param);
    }

    fn disable(&self, param: u32) {
        self.gl.disable(param);
    }

    /// Unimplemented - method missing in WebGL (and ES2, ES3)
    fn point_size(&self, _size: f32) {
        // TODO!: log warning instead of panic?
        unimplemented!("method not available in WebGL")
    }

    fn active_texture(&self, unit: u32) {
        self.gl.active_texture(unit);
    }

    fn bind_texture(&self, target: u32, texture: Option<&Self::GlTexture>) {
        self.gl.bind_texture(target, texture);
    }

    fn blend_func(&self, src: u32, dst: u32) {
        self.gl.blend_func(src, dst);
    }

    fn create_texture(&self) -> Self::GlTexture {
        self.gl.create_texture().unwrap()
    }

    fn delete_texture(&self, texture: &Self::GlTexture) {
        self.gl.delete_texture(Some(texture));
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
        // TODO!: unused_must_use - return Result?
        let _ = self.gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
            target,
            level,
            internal_format,
            width,
            height,
            border,
            format,
            ty,
            pixels,
        );
    }

    fn generate_mipmap(&self) {
        self.gl.generate_mipmap(glenum::TextureKind::Texture2d as _);
    }

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32) {
        self.gl.tex_parameteri(target, parameter, value);
    }

    fn get_uniform_location(
        &self,
        program: &Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation {
        self.gl.get_uniform_location(program, name).unwrap()
    }

    fn uniform_1i(&self, location: &Self::GlUniformLocation, x: i32) {
        self.gl.uniform1i(Some(location), x);
    }

    fn uniform_1f(&self, location: &Self::GlUniformLocation, x: f32) {
        self.gl.uniform1f(Some(location), x);
    }

    fn uniform_3fv(&self, location: &Self::GlUniformLocation, x: &[f32; 3]) {
        self.gl.uniform3fv_with_f32_array(Some(location), x);
    }

    fn uniform_4fv(&self, location: &Self::GlUniformLocation, x: &[f32; 4]) {
        self.gl.uniform4fv_with_f32_array(Some(location), x);
    }

    fn uniform_2f(&self, location: &Self::GlUniformLocation, x: f32, y: f32) {
        self.gl.uniform2f(Some(location), x, y);
    }

    fn uniform_3f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32) {
        self.gl.uniform3f(Some(location), x, y, z);
    }

    fn uniform_matrix_4fv(&self, _location: &Self::GlUniformLocation, _mat: &[[f32; 4]; 4]) {
        // TODO!!: how to convert properly?
        // self.gl.uniform_matrix4fv_with_f32_array(Some(location), false, std::mem::transmute(mat));
        unimplemented!();
    }
}
