use super::complex::Complex;
use wasm_bindgen::prelude::*;

/// Returns e^(i theta) for real theta.
#[wasm_bindgen]
pub fn cis(theta: f64) -> Complex {
    let c = theta.cos();
    let s = theta.sin();
    Complex::new(c, s)
}
