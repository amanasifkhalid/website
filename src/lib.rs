use wasm_bindgen::prelude::*;

#[wasm_bindgen(module="/index.js")]
extern {
    pub fn output(msg: &str);
}

#[wasm_bindgen]
pub fn exec(cmd: &str) {
    output(cmd);
}