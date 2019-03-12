pub extern crate yage_core as core;
pub extern crate yage_gl as gl;
pub extern crate yage_app as app;
#[cfg(feature = "gltf")]
pub extern crate yage_gltf as gltf;

mod utils;

use cfg_if::cfg_if;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "wasm")]
use wasm_bindgen::JsCast;

#[cfg(feature = "wasm")]
use js_sys::WebAssembly;
#[cfg(feature = "wasm")]
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn start_new() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    // TODO!: make it WebGL2
    let context: WebGlRenderingContext = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into()?;

    let gl = gl::GL::from_webgl_context(context);
    gl.clear_color(0.0, 1.0, 0.0, 1.0);
    gl.clear(glenum::BufferBit::Color);

    Ok(())
}
