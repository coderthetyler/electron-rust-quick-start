# electron-rust-quick-start
A minimal Electron + Rust (via WebAssembly) project template

## ✅ Prerequisites

- [Rust (≥ 1.51.0) 🦀 ](https://www.rust-lang.org/)
- [wasm-pack](https://github.com/rustwasm/wasm-pack): `cargo install wasm-pack`
- [Mandatory reading! 📚](https://www.electronjs.org/docs/tutorial/security)

## ⚡️ Getting started

Here's a one-liner:
```
git clone https://github.com/tylerdaddio/electron-rust-quick-start && cd electron-rust-quick-start && npm i && npm run wasm-build && npm start
```

And here's that one-linear broken into steps:

```
# Clone this repo
git clone https://github.com/tylerdaddio/electron-rust-quick-start

# Navigate into the folder
cd electron-rust-quick-start

# Download required npm packages listed in `package.json` (i.e. Electron & its dependencies)
npm install

# Build the sample `rust-lib` project into WebAssembly & JavaScript bindings. (See the full command in the `package.json`.)
npm run wasm-build

# Run the application. A window should open and an alert should appear.
npm start
```

If you plan to use this as a starting point for your project, definitely update the `package.json` and `Cargo.toml` with your information.

## 🌈 About

This template was created by synthesizing information from the following sources:

- [*Quick Start Guide*, Electron](https://www.electronjs.org/docs/tutorial/quick-start)
- [*Compiling from Rust to WebAssembly*, Mozilla](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm)

## ✨ Tidbits

> The `Content-Security-Policy` in the `index.html` is set to enable `unsafe-eval`. That seems unsafe. Why is that unsafe thing there?

Good question! 

At time of writing, `unsafe-eval` is required in Chromium if you want to call into a WASM module. [This shouldn't be required once `wasm-eval` is standardized](https://github.com/WebAssembly/content-security-policy/issues/7), but only [maybe (?)](https://bugs.chromium.org/p/chromium/issues/detail?id=948834&can=1&q=wasm-eval).

As Electron is built on a web brower *and* has access to the full resources of your computer, it is *severely unsafe by default*. Before getting started, please perform the [mandatory reading 📚](https://www.electronjs.org/docs/tutorial/security).
