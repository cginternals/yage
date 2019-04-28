use yage::glutin::{
    Application,
    Window
};

mod renderer;
use renderer::Renderer;

fn main() {
    // Create application
    let mut app = Application::new();

    // Create window
    let mut window = Window::new(&app);

    // Set renderer
    let renderer = Renderer::new();
    window.canvas_mut().set_renderer(renderer);

    // Add window to application
    let _ = app.add_window(window);

    // Run main loop
    app.run();
}
