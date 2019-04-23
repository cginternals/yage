use yage::core::{
    Context,
    GpuObject
};
use yage::glutin::{
    Application,
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
        app.window_mut(window_id).unwrap().render();

        // swap buffers
        // [TODO] will be removed
        app.window(window_id).unwrap().swap_buffers();
    }

    // de-initialize renderer
    // [TODO] will be removed
    app.window_mut(window_id).unwrap().canvas_mut().deinit();
}
