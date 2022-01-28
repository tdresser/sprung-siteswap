mod lib;
use crate::lib::data::Pattern;
use urlencoding::encode;
use wasm_bindgen::prelude::wasm_bindgen;

#[allow(dead_code)]
#[wasm_bindgen(js_name = "Pattern", getter_with_clone)]
pub struct ParsedPattern {
    pub canonical: String,
    pub siteswap: String,
    pub juggle_anim_url: String,
    pub hands: String,
    pub error: Option<String>,
}

#[wasm_bindgen()]
#[allow(dead_code)]
pub fn parse(s: String) -> ParsedPattern {
    let mut pattern = Pattern::new(&s);
    match pattern.error() {
        Some(e) => {
            return ParsedPattern {
                canonical: "".to_string(),
                siteswap: "".to_string(),
                juggle_anim_url: "".to_string(),
                hands: "".to_string(),
                error: Some(e.to_string()),
            }
        }
        None => {
            let siteswap = pattern.get_traditional_siteswap();
            let hands = pattern.get_hand_positions();
            let url = format!(
                "https://jugglinglab.org/anim?{}{}{}{}{}",
                "redirect=true;",
                "pattern=",
                encode(&siteswap),
                ";hands=",
                encode(&hands),
            );
            return ParsedPattern {
                canonical: pattern.get_canonical_form(),
                siteswap: siteswap,
                juggle_anim_url: url,
                hands: hands,
                error: None,
            };
        }
    }
}
