use glutin::GlContext;

use yage::gl::{GL, GlFunctions, check_error, objects::Program};
use yage::gl::glenum;

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_dimensions(glutin::dpi::LogicalSize::new(300.0, 200.0));
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    // println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

    let gl = GL::new();
    gl.clear_color(0.0, 1.0, 0.0, 1.0);

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

    let mut running = true;
    while running {
        events_loop.poll_events(|event| {
            // println!("{:?}", event);
            #[allow(clippy::single_match)]
            match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => running = false,
                    glutin::WindowEvent::Resized(logical_size) => {
                        let dpi_factor = gl_window.get_hidpi_factor();
                        gl_window.resize(logical_size.to_physical(dpi_factor));
                    },
                    _ => (),
                },
                _ => ()
            }
        });

        gl.clear(glenum::BufferBit::Color);
        let _ = gl_window.swap_buffers();
    }
}


const VS_SRC: &str = "
#version 100
precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}";

const FS_SRC: &str = "
#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}";
