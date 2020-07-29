use super::complex::Complex;
use wasm_bindgen::prelude::*;

/// Returns e^(i theta) for real theta.
#[wasm_bindgen(js_name = Cis)]
pub fn cis(theta: f64) -> Complex {
    let c = theta.cos();
    let s = theta.sin();
    Complex::new(c, s)
}
