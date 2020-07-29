use super::{basic_arithmetic::divide, complex::Complex};
use wasm_bindgen::prelude::*;

/// Returns sinh(z).
#[wasm_bindgen(js_name = Sinh)]
pub fn sinh(z: &Complex) -> Complex {
    let (a, b) = (z.re, z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(sinh_a * cos_b, cosh_a * sin_b)
}

/// Returns cosh(z).
#[wasm_bindgen(js_name = Cosh)]
pub fn cosh(z: &Complex) -> Complex {
    let (a, b) = (z.re, z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(cosh_a * cos_b, sinh_a * sin_b)
}

/// Returns tanh(z).
#[wasm_bindgen(js_name = Tanh)]
pub fn tanh(z: &Complex) -> Complex {
    let (a, b) = (2.0 * z.re, 2.0 * z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(sinh_a, sin_b).scale(1.0 / (cosh_a + cos_b))
}

/// Returns sech(z).
#[wasm_bindgen(js_name = Sech)]
pub fn sech(z: &Complex) -> Complex {
    divide(&Complex::one(), &cosh(z))
}

/// Returns csch(z).
#[wasm_bindgen(js_name = Csch)]
pub fn csch(z: &Complex) -> Complex {
    divide(&Complex::one(), &sinh(z))
}

/// Returns coth(z).
#[wasm_bindgen(js_name = Coth)]
pub fn coth(z: &Complex) -> Complex {
    divide(&Complex::one(), &tanh(z))
}
