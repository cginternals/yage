#[macro_use]
extern crate cfg_if;
extern crate web_sys;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::WebGlRenderingContext;

use yage::gl::GL;
use yage::gl::glenum;

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

    // TODO!: make it WebGL2
    let context: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into()?;

    let gl = GL::from_webgl_context(context);
    gl.clear_color(0.0, 1.0, 0.0, 1.0);
    gl.clear(glenum::BufferBit::Color);

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

    setup_canvas();

    Ok(())
}
