mod lib;
use crate::lib::parse::parse;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
pub fn get_canonical_form(s: String) -> String {
    let mut pattern = parse(&s);
    return pattern.get_canonical_form();
}
