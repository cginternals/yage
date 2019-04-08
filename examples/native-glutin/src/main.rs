use yage::gl::{GL, GlFunctions, check_error, objects::Program};
use yage::gl::glenum;
use yage::glutin::Application;
use yage::glutin::Window;
use yage::glutin::Context;

fn main() {
    // create application
    let mut app = Application::new();

    // create window
    let window_id = app.add_window(Window::new(&app));

    // activate context
    app.window(window_id).unwrap().make_current();

    // create OpenGL wrapper
    let gl = GL::new();

    gl.clear_color(0.1, 0.2, 0.3, 1.0);

    // let vs = gl.create_shader(glenum::ShaderKind::Vertex);
    // gl.shader_source(vs, VS_SRC);
    // gl.compile_shader(vs);

    // let fs = gl.create_shader(glenum::ShaderKind::Fragment);
    // gl.shader_source(vs, FS_SRC);
    // gl.compile_shader(fs);

    // let program = gl.create_program();

    // gl.attach_shader(program, vs);
    // gl.attach_shader(program, fs);
    // check_error!();

    // gl.link_program(program);
    // check_error!();
    // gl.use_program(Some(program));

    let program = Program::from_source(&gl, VS_SRC, FS_SRC, &[]);
    check_error!();
    program.use_program();
    check_error!();

    let vb = gl.create_buffer();
    gl.bind_buffer(glenum::BufferKind::Array as _, Some(vb));
    gl.buffer_data(
        glenum::BufferKind::Array as _,
        &VERTEX_DATA,
        glenum::DrawMode::Static as _
    );

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(Some(vao));

    gl.vertex_attrib_pointer(
        0,
        2,
        gl::FLOAT,
        false,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        0,
    );

    gl.vertex_attrib_pointer(
        1,
        3,
        gl::FLOAT,
        false,
        5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
        2 * std::mem::size_of::<f32>() as i32,
    );

    gl.enable_vertex_attrib_array(0);
    gl.enable_vertex_attrib_array(1);

    check_error!();

    unsafe {
        gl::Viewport(0, 0, 300, 200);
    }

    while app.is_running() {
        app.poll_events();

        gl.clear(glenum::BufferBit::Color);
        gl.draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();

        app.window(window_id).unwrap().swap_buffers();
    }
}


const VS_SRC: &str = "
#version 330 core
precision mediump float;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}";

const FS_SRC: &str = "
#version 330 core
precision mediump float;
in vec3 v_color;
out vec4 FragColor;
void main() {
    FragColor = vec4(v_color, 1.0);
    //FragColor = vec4(1.0, 0.0, 0.0, 1.0);
}";

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];
