mod lib;
use crate::lib::data::Pattern;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
pub fn get_canonical_form(s: String) -> String {
    let mut pattern = Pattern::new(&s);
    match pattern.error() {
        Some(e) => return e.clone(),
        None => return pattern.get_canonical_form(),
    }
}
