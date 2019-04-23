use glutin::GlContext;

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

    // create renderer
    let renderer = ExampleRenderer::new();
    app.window_mut(window_id).unwrap().canvas_mut().set_renderer(renderer);

    // initialize renderer
    // [TODO] will be removed
    app.window_mut(window_id).unwrap().canvas_mut().init();

    // run main loop
    while app.is_running() {
        app.poll_events();

        // execute renderer
        // [TODO] will be removed
        app.window_mut(window_id).unwrap().canvas_mut().render();

        // swap buffers
        // [TODO] will be removed
        app.window(window_id).unwrap().swap_buffers();
    }

    // de-initialize renderer
    // [TODO] will be removed
    app.window_mut(window_id).unwrap().canvas_mut().deinit();
}
