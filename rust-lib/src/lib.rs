use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

/// This is the function we call from Electron!
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello from Rust, {}!", name));
}