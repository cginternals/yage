# yage - Yet another graphics engine 
[![crates.io](https://img.shields.io/crates/v/yage.svg)](https://crates.io/crates/yage)
[![Maintenance](https://img.shields.io/badge/maintenance-experimental-blue.svg)](https://github.com/rust-lang/rfcs/blob/master/text/1824-crates.io-default-ranking.md#maintenance)
[![](https://tokei.rs/b1/github/bwasty/yage)](https://github.com/Aaronepower/tokei)
[![Build Status](https://travis-ci.com/bwasty/yage.svg?branch=master)](https://travis-ci.com/bwasty/yage)
[![Build status](https://ci.appveyor.com/api/projects/status/h088302uygm2hvgr/branch/master?svg=true)](https://ci.appveyor.com/project/bwasty/yage/branch/master)

Graphics engine targeting WebGL 2.0 (via WebAssembly) and OpenGL 3.3+.

## Getting started
### Requirements
- [Rust 1.32+](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- [npm](https://www.npmjs.com/get-npm)

### Building
- `wasm-pack build`

---

_GENERATED PART OF README_

---

# 🦀🕸️ `wasm-pack-template`

A template for kick starting a Rust and WebAssembly project using
[`wasm-pack`](https://github.com/rustwasm/wasm-pack).

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

* Want to use the published NPM package in a Website? [Check out
  `create-wasm-app`.](https://github.com/rustwasm/create-wasm-app)
* Want to make a monorepo-style Website without publishing to NPM? Check out
  [`rust-webpack-template`](https://github.com/rustwasm/rust-webpack-template)
  and/or
  [`rust-parcel-template`](https://github.com/rustwasm/rust-parcel-template).

## 🔋 Batteries Included

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```
