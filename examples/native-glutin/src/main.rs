use yage::glutin::{
    Application,
    Window
};

mod renderer;
use renderer::Renderer;

fn main() {
    // create application
    let mut app = Application::new();

    // create window
    let mut window = Window::new(&app);

    // set renderer
    let renderer = Renderer::new();
    window.canvas_mut().set_renderer(renderer);

    // add window to application
    let _ = app.add_window(window);

    // run main loop
    app.run();
}
