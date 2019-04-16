pub extern crate yage_core as core;
#[cfg(not(target_arch = "wasm32"))]
pub extern crate yage_glutin as glutin;
#[cfg(target_arch = "wasm32")]
pub extern crate yage_web as web;
#[cfg(feature = "gltf")]
pub extern crate yage_gltf as gltf;

mod utils;

use cfg_if::cfg_if;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}
