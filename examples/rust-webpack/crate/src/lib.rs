#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGl2RenderingContext;

use yage::gl::{GL, GlFunctions, glenum, objects::{Program, Buffer}};

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
    // TODO!!: most of this setup should go into yage-web
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context: WebGl2RenderingContext = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into()?;

    let gl = GL::from_webgl_context(context);
    gl.clear_color(0.1, 0.2, 0.3, 1.0);
    gl.clear(glenum::BufferBit::Color);

    let program = Program::from_source(&gl, VS_SRC, FS_SRC, &[]);
    program.use_program();

    let vertex_buffer = Buffer::new(&gl, glenum::BufferKind::Array as _);
    vertex_buffer.bind();
    vertex_buffer.set_data(&VERTEX_DATA, glenum::DrawMode::Static as _);

    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(Some(&vao));

    vertex_buffer.attrib_enable(
        0,
        2,
        glenum::DataType::Float as _,
        false,
        5 * std::mem::size_of::<f32>() as i32,
        0,
    );

    vertex_buffer.attrib_enable(
        1,
        3,
        glenum::DataType::Float as _,
        false,
        5 * std::mem::size_of::<f32>() as i32,
        2 * std::mem::size_of::<f32>() as i32,
    );

    // check_error!();

    gl.draw_arrays(glenum::Primitives::Triangles as _, 0, 3);

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

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];
