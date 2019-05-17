/// Trait for many GL functions
///
/// Associated types are used to support different handles types in native GL and WebGL
/// (integers vs opaque JS types like `WebGLShader`)
/// Please refer to the OpenGL/WebGL documentation for details about each
/// function (hint: http://docs.gl/).
/// Some functions are named differently in WebGL than in OpenGL - we use the WebGL nomenclature
/// since they are clearer (e.g. create_buffer for gl::GenBuffers,
/// get_shader_parameter for gl::GetShaderiv)
pub trait GlFunctions {
    type GlShader;
    type GlProgram;
    type GlBuffer;
    type GlVertexArray;
    type GlTexture;
    type GlUniformLocation;
    type GlFramebuffer;
    type GlRenderbuffer;
    type GlTransformFeedback;

    // View and Clip

    fn scissor(&self, x: i32, y: i32, width: i32, height: i32);
    fn viewport(&self, x: i32, y: i32, width: i32, height: i32);

    // Programs and Shaders

    fn create_shader(&self, kind: glenum::ShaderKind) -> Self::GlShader;
    fn shader_source(&self, shader: &Self::GlShader, source: &str);
    fn compile_shader(&self, shader: &Self::GlShader);
    fn delete_shader(&self, shader: &Self::GlShader);
    /// Named after the WebGL function. See `gl::GetShaderiv` for OpenGL.
    fn get_shader_parameter(&self, shader: &Self::GlShader, param: u32) -> i32;
    fn get_shader_info_log(&self, shader: &Self::GlShader) -> String;

    fn create_program(&self) -> Self::GlProgram;
    fn attach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader);
    fn detach_shader(&self, program: &Self::GlProgram, shader: &Self::GlShader);
    fn link_program(&self, program: &Self::GlProgram);
    fn get_program_parameter(&self, program: &Self::GlProgram, param: u32) -> i32;
    fn get_program_info_log(&self, program: &Self::GlProgram) -> String;
    fn use_program(&self, program: Option<&Self::GlProgram>);
    fn get_attrib_location(&self, program: &Self::GlProgram, name: &str) -> i32;
    fn bind_attrib_location(&self, program: &Self::GlProgram, index: u32, name: &str);
    fn delete_program(&self, program: &Self::GlProgram);

    // Buffer Objects

    /// Named after the WebGL function. See `gl::GenBuffers` for OpenGL.
    fn create_buffer(&self) -> Self::GlBuffer;
    fn bind_buffer(&self, target: u32, buffer: Option<&Self::GlBuffer>);
    fn buffer_data<T>(&self, target: u32, data: &[T], usage: u32);
    fn buffer_sub_data<T>(&self, target: u32, offset: isize, data: &[T]);
    fn delete_buffer(&self, buffer: &Self::GlBuffer);
    fn is_buffer(&self, buffer: &Self::GlBuffer) -> bool;

    // Vertex Array Objects

    fn create_vertex_array(&self) -> Self::GlVertexArray;
    fn bind_vertex_array(&self, vertex_array: Option<&Self::GlVertexArray>);
    fn delete_vertex_array(&self, vertex_array: &Self::GlVertexArray);

    // Uniforms and Attributes

    fn vertex_attrib_pointer(
        &self,
        index: u32,
        size: i32,
        data_type: u32,
        normalized: bool,
        stride: i32,
        offset: i32,
    );
    fn enable_vertex_attrib_array(&self, index: u32);
    fn disable_vertex_attrib_array(&self, index: u32);

    fn get_uniform_location(
        &self,
        program: &Self::GlProgram,
        name: &str,
    ) -> Self::GlUniformLocation;
    fn uniform_1i(&self, location: &Self::GlUniformLocation, x: i32);
    fn uniform_1ui(&self, location: &Self::GlUniformLocation, x: u32);
    fn uniform_1f(&self, location: &Self::GlUniformLocation, x: f32);

    fn uniform_2i(&self, location: &Self::GlUniformLocation, x: i32, y: i32);
    fn uniform_2ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32);
    fn uniform_2f(&self, location: &Self::GlUniformLocation, x: f32, y: f32);

    fn uniform_3i(&self, location: &Self::GlUniformLocation, x: i32, y: i32, z: i32);
    fn uniform_3ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32, z: u32);
    fn uniform_3f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32);

    fn uniform_4i(&self, location: &Self::GlUniformLocation, x: i32, y: i32, z: i32, w: i32);
    fn uniform_4ui(&self, location: &Self::GlUniformLocation, x: u32, y: u32, z: u32, w: u32);
    fn uniform_4f(&self, location: &Self::GlUniformLocation, x: f32, y: f32, z: f32, w: f32);

    fn uniform_matrix_2fv(&self, location: &Self::GlUniformLocation, value: &[[f32; 2]; 2]);
    fn uniform_matrix_3fv(&self, location: &Self::GlUniformLocation, value: &[[f32; 3]; 3]);
    fn uniform_matrix_4fv(&self, location: &Self::GlUniformLocation, value: &[[f32; 4]; 4]);

    // Writing to the Draw Buffer

    fn draw_arrays(&self, mode: u32, first: i32, count: i32);
    fn draw_elements(&self, mode: u32, count: i32, element_type: u32, offset: i32);

    fn vertex_attrib_divisor(&self, index: u32, divisor: u32);
    fn draw_arrays_instanced(&self, mode: u32, first: i32, count: i32, instance_count: i32);
    fn draw_elements_instanced(
        &self,
        mode: u32,
        count: i32,
        element_type: u32,
        offset: i32,
        instance_count: i32,
    );

    // Special Functions

    fn enable(&self, param: u32);
    fn disable(&self, param: u32);
    fn finish(&self);
    fn flush(&self);
    fn get_error(&self) -> u32;
    fn get_parameter_i32(&self, parameter: u32) -> i32;
    fn pixel_storei(&self, storage: u32, value: i32);

    // Texture Objects

    fn active_texture(&self, unit: u32);
    fn bind_texture(&self, target: u32, texture: Option<&Self::GlTexture>);

    fn create_texture(&self) -> Self::GlTexture;
    fn delete_texture(&self, texture: &Self::GlTexture);

    #[allow(clippy::too_many_arguments)]
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
    );

    #[allow(clippy::too_many_arguments)]
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
    );

    fn generate_mipmap(&self, target: u32);

    fn tex_parameteri(&self, target: u32, parameter: u32, value: i32);

    fn is_texture(&self, texture: &Self::GlTexture) -> bool;

    // Framebuffer Objects

    fn create_framebuffer(&self) -> Self::GlFramebuffer;
    fn delete_framebuffer(&self, framebuffer: &Self::GlFramebuffer);
    fn bind_framebuffer(&self, target: u32, framebuffer: Option<&Self::GlFramebuffer>);
    fn framebuffer_texture_2d(
        &self,
        target: u32,
        attachment: u32,
        texture_target: u32,
        texture: Option<&Self::GlTexture>,
        level: i32,
    );
    fn framebuffer_renderbuffer(
        &self,
        target: u32,
        attachment: u32,
        renderbuffer_target: u32,
        renderbuffer: Option<&Self::GlRenderbuffer>,
    );
    fn is_frambuffer(&self, framebuffer: &Self::GlFramebuffer) -> bool;
    fn check_framebuffer_status(&self, target: u32) -> u32;
    #[allow(clippy::too_many_arguments)]
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
    );
    fn read_buffer(&self, mode: u32);

    // Renderbuffer Objects

    fn create_renderbuffer(&self) -> Self::GlRenderbuffer;
    fn delete_renderbuffer(&self, renderbuffer: &Self::GlRenderbuffer);
    fn bind_renderbuffer(&self, target: u32, renderbuffer: Option<&Self::GlRenderbuffer>);
    fn renderbuffer_storage(&self, target: u32, internal_format: u32, width: i32, height: i32);

    // Per-Fragment Operations

    fn blend_func(&self, src: u32, dst: u32);
    fn blend_func_separate(&self, src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32);
    fn depth_func(&self, func: u32);
    fn stencil_func(&self, func: u32, reference: i32, mask: u32);
    fn stencil_op(&self, stencil_fail: u32, depth_fail: u32, pass: u32);

    // Whole Framebuffer Operations

    fn clear(&self, mask: u32);
    fn clear_color(&self, r: f32, g: f32, b: f32, a: f32);
    fn clear_depth(&self, depth: f32);
    fn clear_stencil(&self, stencil: i32);

    fn depth_mask(&self, value: bool);
    fn stencil_mask(&self, mask: u32);

    // Read Back Pixels

    #[allow(clippy::too_many_arguments)]
    fn read_pixels(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        format: u32,
        kind: u32,
        data: &mut [u8],
    );

    // Rasterization

    fn cull_face(&self, value: u32);
    fn point_size(&self, size: f32);
    fn polygon_mode(&self, face: u32, mode: u32);

    // Multiple Render Targets

    fn draw_buffers(&self, buffers: &[u32]);
    // TODO: glClearBuffer - 4 variants...

    // Transform Feedback

    // TODO!: transform feeback (NOTE: create is implemented, but commented out)
    // fn create_transform_feedback(&self) -> Self::GlTransformFeedback;
    // fn delete_transform_feedback(&self, tf: Option<&WebGlTransformFeedback>);
    // fn bind_transform_feedback(&self, target: u32, tf: Option<&Self::GlTransformFeedback>);
    // fn bind_buffer_base(&self, target: u32, index: u32, buffer: Option<&Self::GlBuffer>);
    // fn transform_feedback_varyings(
    //     &self,
    //     program: &WebGlProgram,
    //     varyings: &[&str],
    //     buffer_mode: u32,
    // );
    // fn begin_transform_feedback(&self, primitive_mode: u32);
    // fn pause_transform_feedback(&self);
    // fn resume_transform_feedback(&self);
    // fn end_transform_feedback(&self);
    // fn is_transform_feedback(&self, tf: Option<&WebGlTransformFeedback>) -> bool;

    // TODO!:
    // get_parameter / GetIntegerv
    // reorder based on quick reference sections...
}
