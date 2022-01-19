mod lib;
use crate::lib::data::Pattern;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen()]
#[allow(dead_code)]
pub fn get_canonical_form(s: String) -> String {
    let mut pattern = Pattern::new(&s);
    match pattern.error() {
        Some(e) => return e.clone(),
        None => return pattern.get_canonical_form(),
    }
}

#[wasm_bindgen()]
#[allow(dead_code)]
pub fn get_traditional_siteswap(s: String) -> String {
    let pattern = Pattern::new(&s);
    match pattern.error() {
        Some(e) => return e.clone(),
        None => return pattern.get_traditional_siteswap(),
    }
}
