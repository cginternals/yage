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
    let mut window = Window::new(&app);

    // set renderer
    let renderer = ExampleRenderer::new();
    window.canvas_mut().set_renderer(renderer);

    // add window to application
    let _ = app.add_window(window);

    // run main loop
    app.run();
}
