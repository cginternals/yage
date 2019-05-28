# yage - Yet another graphics engine 
[![crates.io](https://img.shields.io/crates/v/yage.svg)](https://crates.io/crates/yage)
[![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)](https://github.com/rust-lang/rfcs/blob/master/text/1824-crates.io-default-ranking.md#maintenance)
[![](https://tokei.rs/b1/github/bwasty/yage)](https://github.com/Aaronepower/tokei)
[![Build Status](https://travis-ci.com/bwasty/yage.svg?branch=master)](https://travis-ci.com/bwasty/yage)
[![Build status](https://ci.appveyor.com/api/projects/status/h088302uygm2hvgr/branch/master?svg=true)](https://ci.appveyor.com/project/bwasty/yage/branch/master)

Graphics engine targeting WebGL 2.0 (via WebAssembly) and OpenGL 3.3+.

## Current state
Very, very early. Currently it's mostly [wasm-pack-template](https://github.com/rustwasm/wasm-pack-template) and [create-wasm-app](https://github.com/rustwasm/create-wasm-app) combined with the [wasm-bindgen WebGL example](https://rustwasm.github.io/wasm-bindgen/examples/webgl.html).

## Getting started
### Requirements
- [Rust 1.32+](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/get-npm)

### Building
```
wasm-pack build
```

### Examples
* Native
  - `cd examples/native-glutin`
  - `cargo run`
* Web
  - `cd examples/rust-webpack`
  - `npm install`
  - `npm start`

### Testing
```
wasm-pack test --headless --firefox
```

### Development Hints
* Make sure to check both native and wasm compilation, e.g. with
  - `cargo check` (native)
  - `cargo check --target wasm32-unknown-unknown --lib` (wasm)
  - with [cargo-watch](https://github.com/passcod/cargo-watch) installed, the above can be wrapped in `cargo watch -x "..."` for a permanent watch (use `-w`/`-i` to fine-tune watched/ignored files)
