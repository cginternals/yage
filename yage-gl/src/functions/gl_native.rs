use glenum::*;

#[derive(Default)]
pub struct GL {}

impl GL {
    pub fn new() -> GL {
        GL {}
    }
}

impl super::GlFunctions for GL {
    type GlShader = gl::types::GLuint;
    type GlProgram = gl::types::GLuint;
    type GlBuffer = gl::types::GLuint;
    type GlVertexArray = gl::types::GLuint;
    type GlTexture = gl::types::GLuint;
    type GlUniformLocation = gl::types::GLint;
    type GlFramebuffer = gl::types::GLuint;
    type GlRenderbuffer = gl::types::GLuint;

    /// specify clear values for the color buffers
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    /// clear buffers to preset values
    fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
    }

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader {
        unsafe { gl::CreateShader(kind as _) }
    }

    fn shader_source(&self, shader: &Self::GlShader, source: &str) {
        unsafe {
            gl::ShaderSource(
                *shader,
                1,
                &(source.as_ptr() as *const i8),
                &(source.len() as i32),
            );
        }
    }

    fn compile_shader(&self, shader: &Self::GlShader) {
        unsafe {
            gl::CompileShader(*shader);
        }
    }

    fn delete_shader(&self, shader: &Self::GlShader) {
        unsafe {
            gl::DeleteShader(*shader);
        }
    }

    fn get_shader_parameter(&self, shader: &Self::GlShader, param: u32) -> i32 {
        let mut result = 0;
        unsafe {
            gl::GetShaderiv(*shader, param, &mut result);
        }
        result
    }

    fn get_shader_info_log(&self, shader: &Self::GlShader) -> String {
        let mut length = self.get_shader_parameter(shader, gl::INFO_LOG_LENGTH);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            unsafe {
                gl::GetShaderInfoLog(
                    *shader,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut gl::types::GLchar,
                );
            }
            log.truncate(length as usize);
            log
        } else {
            "".into()
        }
    }

    fn create_program(&self) -> Self::GlProgram {
        unsafe { gl::CreateProgram() }
    }

    fn attach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader) {
        unsafe { gl::AttachShader(*program, *shader) }
    }

    fn link_program(&self, program: &Self::GlProgram) {
        unsafe { gl::LinkProgram(*program) }
    }

    fn get_program_parameter(&self, program: &Self::GlProgram, param: u32) -> i32 {
        let mut result = 0;
        unsafe {
            gl::GetProgramiv(*program, param, &mut result);
        }
        result
    }

    fn get_program_info_log(&self, program: &Self::GlProgram) -> String {
        let mut length = self.get_program_parameter(program, gl::INFO_LOG_LENGTH);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(std::iter::repeat('\0').take(length as usize));
            unsafe {
                gl::GetProgramInfoLog(
                    *program,
                    length,
                    &mut length,
                    (&log[..]).as_ptr() as *mut gl::types::GLchar,
                );
            }
            log.truncate(length as usize);
            log
        } else {
            "".into()
        }
    }

    fn use_program(&self, program: Option<&Self::GlProgram>) {
        unsafe {
            gl::UseProgram(*program.unwrap_or(&0));
        }
    }

    fn create_buffer(&self) -> Self::GlBuffer {
        let mut buf = 0;
        unsafe {
            gl::GenBuffers(1, &mut buf);
        }
        buf
    }

    fn bind_buffer(&self, target: u32, buffer: Option<&Self::GlBuffer>) {
        unsafe {
            gl::BindBuffer(target, *buffer.unwrap_or(&0));
        }
    }

    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32) {
        unsafe {
            gl::BufferData(
                target,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
                usage,
            );
        }
    }

    fn buffer_sub_data<T>(&self, target: u32, offset: isize, data: &[T]) {
        unsafe {
            gl::BufferSubData(
                target,
                offset,
                (data.len() * std::mem::size_of::<T>()) as isize,
                data.as_ptr() as *const std::ffi::c_void,
            )
        }
    }

    fn delete_buffer(&self, buffer: &Self::GlBuffer) {
        unsafe {
            gl::DeleteBuffers(1, buffer);
        }
    }

    fn create_vertex_array(&self) -> Self::GlVertexArray {
        let mut vao = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        vao
    }

    fn bind_vertex_array(&self, vertex_array: Option<&Self::GlVertexArray>) {
        unsafe {
            gl::BindVertexArray(*vertex_array.unwrap_or(&0));
        }
    }

    fn delete_vertex_array(&self, vertex_array: &Self::GlVertexArray) {
        unsafe {
            gl::DeleteVertexArrays(1, vertex_array);
        }
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
        unsafe {
            gl::VertexAttribPointer(
                index,
                size,
                data_type,
                normalized as u8,
                stride,
                offset as *const std::ffi::c_void,
            );
        }
    }

    fn enable_vertex_attrib_array(&self, index: u32) {
        unsafe {
            gl::EnableVertexAttribArray(index);
        }
    }

    fn disable_vertex_attrib_array(&self, index: u32) {
        unsafe {
            gl::DisableVertexAttribArray(index);
        }
    }

    fn draw_arrays(&self, mode: u32, first: i32, count: i32) {
        unsafe {
            gl::DrawArrays(mode as u32, first, count);
        }
    }

    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32) {
        unsafe {
            gl::DrawElements(
                mode as u32,
                count,
                element_type as u32,
                offset as *const std::ffi::c_void,
            );
        }
    }

    fn enable(&self, param: u32) {
        unsafe {
            gl::Enable(param);
        }
    }

    fn disable(&self, param: u32) {
        unsafe {
            gl::Disable(param);
        }
    }

    fn point_size(&self, size: f32) {
        unsafe {
            gl::PointSize(size);
        }
    }

    fn active_texture(&self, unit: u32) {
        unsafe {
            gl::ActiveTexture(unit);
        }
    }

    fn bind_texture(&self, target: u32, texture: Option<&Self::GlTexture>) {
        unsafe {
            gl::BindTexture(target, *texture.unwrap_or(&0));
        }
    }

    fn blend_func(&self, src: u32, dst: u32) {
        unsafe {
            gl::BlendFunc(src, dst);
        }
    }

    fn create_texture(&self) -> Self::GlTexture {
        let mut tex = 0;
        unsafe {
            gl::GenTextures(1, &mut tex);
        }
        tex
    }

    fn delete_texture(&self, texture: &Self::GlTexture) {
        unsafe {
            gl::DeleteTextures(1, texture);
        }
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
        unsafe {
            gl::TexImage2D(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                ty,
                pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const std::ffi::c_void,
            );
        }
    }

    fn generate_mipmap(&self) {
        unsafe {
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
    }

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32) {
        unsafe {
            gl::TexParameteri(target, parameter, value);
        }
    }

    fn get_uniform_location(
        &self,
        program: &Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation {
        unsafe { gl::GetUniformLocation(*program, name.as_ptr() as *const i8) as i32 }
    }

    fn uniform_1i(&self, location: &Self::GlUniformLocation, x: i32) {
        unsafe {
            gl::Uniform1i(*location, x);
        }
    }

    fn uniform_1f(&self, location: &Self::GlUniformLocation, x: f32) {
        unsafe {
            gl::Uniform1f(*location, x);
        }
    }

    fn uniform_3fv(&self, location: &Self::GlUniformLocation, x: &[f32; 3]) {
        unsafe {
            gl::Uniform3fv(*location, 1, x.as_ptr());
        }
    }

    fn uniform_4fv(&self, location: &Self::GlUniformLocation, x: &[f32; 4]) {
        unsafe {
            gl::Uniform4fv(*location, 1, x.as_ptr());
        }
    }

    fn uniform_2f(&self, location: &Self::GlUniformLocation, x: f32, y: f32) {
        unsafe {
            gl::Uniform2f(*location, x, y);
        }
    }

    fn uniform_3f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32) {
        unsafe {
            gl::Uniform3f(*location, x, y, z);
        }
    }

    fn uniform_matrix_4fv(&self, location: &Self::GlUniformLocation, mat: &[[f32; 4]; 4]) {
        unsafe {
            gl::UniformMatrix4fv(*location, 1, gl::FALSE, mat.as_ptr() as _);
        }
    }

    fn create_framebuffer(&self) -> Self::GlFramebuffer {
        let mut fb = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut fb);
        }
        fb
    }

    fn delete_framebuffer(&self, framebuffer: &Self::GlFramebuffer) {
        unsafe {
            gl::DeleteFramebuffers(1, framebuffer);
        }
    }

    fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Self::GlFramebuffer>) {
        unsafe {
            gl::BindFramebuffer(target, *framebuffer.unwrap_or(&0));
        }
    }

    fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<&Self::GlTexture>,
        level: i32,
    ) {
        unsafe {
            gl::FramebufferTexture2D(
                target,
                attachment,
                texture_target,
                *texture.unwrap_or(&0),
                level,
            );
        }
    }

    fn create_renderbuffer(&self) -> Self::GlRenderbuffer {
        let mut rb = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut rb);
        }
        rb
    }

    fn delete_renderbuffer(&self, renderbuffer: &Self::GlRenderbuffer) {
        unsafe {
            gl::DeleteRenderbuffers(1, renderbuffer);
        }
    }

    fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&Self::GlRenderbuffer>) {
        unsafe {
            gl::BindRenderbuffer(target, *renderbuffer.unwrap_or(&0));
        }
    }

    fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32) {
        unsafe {
            gl::RenderbufferStorage(target, internal_format, width, height);
        }
    }

    fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<&Self::GlRenderbuffer>,
    ) {
        unsafe {
            gl::FramebufferRenderbuffer(
                target,
                attachment,
                renderbuffer_target,
                *renderbuffer.unwrap_or(&0),
            );
        }
    }

    fn check_framebuffer_status(&self, target: u32) -> u32 {
        unsafe { gl::CheckFramebufferStatus(target) }
    }

    fn polygon_mode(&self, face: u32, mode: u32) {
        unsafe {
            gl::PolygonMode(face, mode);
        }
    }

    fn pixel_storei(&self, storage: u32, value: i32) {
        unsafe {
            gl::PixelStorei(storage, value);
        }
    }

    fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        kind: u32,
        data: &mut [u8],
    ) {
        unsafe {
            gl::ReadPixels(
                x as _,
                y as _,
                width as _,
                height as _,
                format as _,
                kind as _,
                data.as_mut_ptr() as _,
            );
        }
    }
}
