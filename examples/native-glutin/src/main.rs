use glutin::GlContext;

use yage::core::{
    GL, GlFunctions,
    glenum,
    check_error,
    Program, Buffer, VertexArray
};
use yage::glutin::{
    Application,
    Context,
    Window
};

mod example_renderer;
use example_renderer::ExampleRenderer;

fn main() {
    // create application
    let mut app = Application::new();

    // create window
    let window_id = app.add_window(Window::new(&app));

    // activate context
    app.window(window_id).unwrap().make_current();

    // resolve OpenGL functions
    // [TODO] automate this
    let gl_window = app.window(window_id).unwrap().get_gl_window();
    gl::load_with(|ptr| gl_window.context().get_proc_address(ptr) as *const _);

    // create OpenGL wrapper
    let gl = GL::new();

    // [TODO] create renderer
    let renderer = ExampleRenderer::new();

    let canvas = app.window_mut(window_id).unwrap().canvas_mut();
    canvas.set_renderer(renderer);

    gl.viewport(0, 0, 300, 200);

    while app.is_running() {
        app.poll_events();

        gl.clear(glenum::BufferBit::Color as u32);
        gl.draw_arrays(gl::TRIANGLES, 0, 3);

        // check_error!();

        app.window(window_id).unwrap().swap_buffers();
    }
}
