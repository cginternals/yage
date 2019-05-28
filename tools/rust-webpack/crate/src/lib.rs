#[macro_use]
extern crate cfg_if;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use yage::core::{
    glenum, Context, Cube, Drawable, GlFunctions, GpuObject, Program, Shader,
};

use yage::web::BrowserContext;

cfg_if! {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function to get better error messages if we ever panic.
    if #[cfg(feature = "console_error_panic_hook")] {
        extern crate console_error_panic_hook;
        use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        fn set_panic_hook() {}
    }
}

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

fn setup_canvas() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = BrowserContext::new(canvas).unwrap();

    context.gl().clear_color(0.1, 0.2, 0.3, 1.0);
    context.gl().clear(glenum::COLOR_BUFFER_BIT);

    // Create shader program
    let mut program = Program::new();
    program.init(&context);
    {
        // Load vertex shader
        let mut vertex_shader = Shader::new(glenum::VERTEX_SHADER);
        vertex_shader.set_code(&context, VS_SRC, &[]);

        // Load fragment shader
        let mut fragment_shader = Shader::new(glenum::FRAGMENT_SHADER);
        fragment_shader.set_code(&context, FS_SRC, &[]);

        // Attach shaders
        program.attach(vertex_shader);
        program.attach(fragment_shader);
    }

    program.use_program(&context);

    let mut cube = Cube::new();
    cube.init(&context);
    cube.draw(&context);

    Ok(())
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from yage"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;

    let _ = setup_canvas();

    Ok(())
}

// NOTE: misusing tex coord as color...
const VS_SRC: &str = "#version 300 es
precision mediump float;
layout (location = 0) in vec2 position;
layout (location = 1) in vec3 color;
out vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}";

const FS_SRC: &str = "#version 300 es
precision mediump float;
in vec3 v_color;
out vec4 FragColor;
void main() {
    FragColor = vec4(v_color, 1.0);
}";
