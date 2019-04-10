use std::ffi::c_void;

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
    type GlTransformFeedback = gl::types::GLuint;

    // View and Clip

    fn viewport(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Viewport(x, y, width, height);
        }
    }

    fn scissor(&self, x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Scissor(x, y, width, height);
        }
    }

    // Programs and Shaders

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

    fn detach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader) {
        unsafe {
            gl::DetachShader(*program, *shader);
        }
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

    fn get_attrib_location(&self, program: &Self::GlProgram, name: &str) -> i32 {
        unsafe { gl::GetAttribLocation(*program, name.as_ptr() as *const i8) }
    }

    fn bind_attrib_location(&self, program: &Self::GlProgram, index: u32, name: &str) {
        unsafe {
            gl::BindAttribLocation(*program, index, name.as_ptr() as *const i8);
        }
    }

    // Buffer Objects

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
                data.as_ptr() as *const c_void,
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
                data.as_ptr() as *const c_void,
            )
        }
    }

    fn delete_buffer(&self, buffer: &Self::GlBuffer) {
        unsafe {
            gl::DeleteBuffers(1, buffer);
        }
    }

    fn is_buffer(&self, buffer: &Self::GlBuffer) -> bool {
        unsafe { gl::IsBuffer(*buffer) != 0 }
    }

    // Vertex Array Objects

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

    // Uniforms and Attributes

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
                offset as *const c_void,
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

    // Writing to the Draw Buffer

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
                offset as *const c_void,
            );
        }
    }

    fn vertex_attrib_divisor(&self, index: u32, divisor: u32) {
        unsafe {
            gl::VertexAttribDivisor(index, divisor);
        }
    }

    fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32) {
        unsafe {
            gl::DrawArraysInstanced(mode, first, count, instance_count);
        }
    }

    fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    ) {
        unsafe {
            gl::DrawElementsInstanced(
                mode,
                count,
                element_type,
                offset as *const c_void,
                instance_count,
            );
        }
    }

    // Special Functions

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

    fn finish(&self) {
        unsafe { gl::Finish() }
    }

    fn flush(&self) {
        unsafe { gl::Flush() }
    }

    fn get_error(&self) -> u32 {
        unsafe { gl::GetError() }
    }

    fn get_parameter_i32(&self, parameter: u32) -> i32 {
        let mut value = 0;
        unsafe {
            gl::GetIntegerv(parameter, &mut value);
        }
        value
    }

    fn pixel_storei(&self, storage: u32, value: i32) {
        unsafe {
            gl::PixelStorei(storage, value);
        }
    }

    // Texture Objects

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
        type_: u32,
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
                type_,
                pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const c_void,
            );
        }
    }

    fn tex_image_3d(
        &self,
        target: u32,
        level: i32,
        internal_format: i32,
        width: i32,
        height: i32,
        depth: i32,
        border: i32,
        format: u32,
        type_: u32,
        pixels: Option<&[u8]>,
    ) {
        unsafe {
            gl::TexImage3D(
                target,
                level,
                internal_format,
                width,
                height,
                depth,
                border,
                format,
                type_,
                pixels.map(|p| p.as_ptr()).unwrap_or(std::ptr::null()) as *const c_void,
            );
        }
    }

    fn generate_mipmap(&self, target: u32) {
        unsafe {
            gl::GenerateMipmap(target);
        }
    }

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32) {
        unsafe {
            gl::TexParameteri(target, parameter, value);
        }
    }

    fn is_texture(&self, texture: &Self::GlTexture) -> bool {
        unsafe { gl::IsTexture(*texture) != 0 }
    }

    // Framebuffer Objects

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

    fn is_frambuffer(&self, framebuffer: &Self::GlFramebuffer) -> bool {
        unsafe { gl::IsFramebuffer(*framebuffer) != 0 }
    }

    fn check_framebuffer_status(&self, target: u32) -> u32 {
        unsafe { gl::CheckFramebufferStatus(target) }
    }

    fn blit_framebuffer(
        &self,
        src_x0: i32,
        src_y0: i32,
        src_x1: i32,
        src_y1: i32,
        dst_x0: i32,
        dst_y0: i32,
        dst_x1: i32,
        dst_y1: i32,
        mask: u32,
        filter: u32,
    ) {
        unsafe {
            gl::BlitFramebuffer(
                src_x0, src_y0, src_x1, src_y1, dst_x0, dst_y0, dst_x1, dst_y1, mask, filter,
            );
        }
    }

    fn read_buffer(&self, mode: u32) {
        unsafe {
            gl::ReadBuffer(mode);
        }
    }

    // Renderbuffer Objects

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

    // Per-Fragment Operations

    fn depth_func(&self, func: u32) {
        unsafe {
            gl::DepthFunc(func);
        }
    }

    fn blend_func(&self, src: u32, dst: u32) {
        unsafe {
            gl::BlendFunc(src, dst);
        }
    }

    fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) {
        unsafe {
            gl::BlendFuncSeparate(src_rgb, dst_rgb, src_alpha, dst_alpha);
        }
    }

    fn stencil_func(&self, func: u32, reference: i32, mask: u32) {
        unsafe {
            gl::StencilFunc(func, reference, mask);
        }
    }

    fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32) {
        unsafe {
            gl::StencilOp(stencil_fail, depth_fail, pass);
        }
    }

    // Whole Framebuffer Operations

    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32) {
        unsafe {
            gl::ClearColor(r, g, b, a);
        }
    }

    fn clear(&self, bit: BufferBit) {
        unsafe {
            gl::Clear(bit as _);
        }
    }

    fn clear_depth(&self, depth: f32) {
        unsafe {
            gl::ClearDepthf(depth);
        }
    }

    fn clear_stencil(&self, stencil: i32) {
        unsafe {
            gl::ClearStencil(stencil);
        }
    }

    fn depth_mask(&self, value: bool) {
        unsafe {
            gl::DepthMask(value as _);
        }
    }

    fn stencil_mask(&self, mask: u32) {
        unsafe {
            gl::StencilMask(mask);
        }
    }

    // Read Back Pixels

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

    // Rasterization

    fn cull_face(&self, value: u32) {
        unsafe {
            gl::CullFace(value);
        }
    }

    fn point_size(&self, size: f32) {
        unsafe {
            gl::PointSize(size);
        }
    }

    fn polygon_mode(&self, face: u32, mode: u32) {
        unsafe {
            gl::PolygonMode(face, mode);
        }
    }

    // Multiple Render Targets

    fn draw_buffers(&self, buffers: &[u32]) {
        unsafe { gl::DrawBuffers(buffers.len() as i32, buffers.as_ptr()) }
    }

    // Transform Feedback

    // fn create_transform_feedback(&self) -> Self::GlTransformFeedback {
    //     let mut feedback = 0;
    //     unsafe {
    //         gl::GenTransformFeedbacks(1, &mut feedback);
    //     }
    //     feedback
    // }
}
