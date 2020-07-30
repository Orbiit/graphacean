use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ComplexInterval {
    re_min: f64,
    re_max: f64,
    im_min: f64,
    im_max: f64,
    def_min: f64,
    def_max: f64,
}