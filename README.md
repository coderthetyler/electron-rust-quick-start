# electron-rust-quick-start
A minimal Electron + Rust (via WebAssembly) project template

## ✅ Prerequisites

- [Rust (≥ 1.51.0)](https://www.rust-lang.org/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack): `cargo install wasm-pack`

## ⚡️ Getting started

Here's a one-liner:
```
git clone https://github.com/tylerdaddio/electron-rust-quick-start && cd electron-rust-quick-start && npm i && npm run wasm-build && npm start
```

And here's the steps one-by-one:

- Clone this repo `git clone https://github.com/tylerdaddio/electron-rust-quick-start`
- Navigate into the folder: `cd electron-rust-quick-start`
- Download required npm packages listed in `package.json` (i.e. Electron & its dependencies): `npm i` or `npm install`
- Run the template: `npm start`


## About

This template was created by synthesizing the following guides:

- [*Compiling from Rust to WebAssembly*, Mozilla](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

