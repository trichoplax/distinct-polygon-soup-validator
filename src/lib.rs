use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn validate_distinct_polygon_soup(s: &str) -> String {
    if s.len()==0{
        return "The string is empty.".to_string();
    }
    "The string is non-empty.".to_string()
}
