use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen(start)]
pub fn main () {
    console::log_1(&"Hello World".into())
}
