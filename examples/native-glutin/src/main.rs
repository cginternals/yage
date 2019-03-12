mod support;

use glutin::GlContext;

// use yage::some_non_wasm_function;
// use yage::gltf::load_gltf;
use yage::gl::GL;
use yage::gl::glenum;

// TODO!!: This is just the (slighly adapted) glutin window example and doesn't even use yage
fn main() {

    // dbg!(some_non_wasm_function());
    // load_gltf("Box.glb");

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("A fantastic window!");
    let context = glutin::ContextBuilder::new();
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    let _ = unsafe { gl_window.make_current() };

    println!("Pixel format of the window's GL context: {:?}", gl_window.get_pixel_format());

    support::load(&gl_window.context());

    let gl = GL::new();
    gl.clear_color(0.0, 1.0, 0.0, 1.0);

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

        // support::draw_frame([1.0, 0.5, 0.7, 1.0]);
        gl.clear(glenum::BufferBit::Color);
        let _ = gl_window.swap_buffers();
    }
}
