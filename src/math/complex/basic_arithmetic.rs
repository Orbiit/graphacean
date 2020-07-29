use super::complex::Complex;
use wasm_bindgen::prelude::*;

/// Returns a + b.
#[wasm_bindgen(js_name = ComplexFunctions_Add)]
pub fn add(a: &Complex, b: &Complex) -> Complex {
    Complex::new(a.re + b.re, a.im + b.im)
}

/// Returns a * b.
#[wasm_bindgen(js_name = ComplexFunctions_Multiply)]
pub fn multiply(a: &Complex, b: &Complex) -> Complex {
    Complex::new(a.re * b.re, a.im * b.im)
}

/// Returns a / b.
#[wasm_bindgen(js_name = ComplexFunctions_Divide)]
pub fn divide(a: &Complex, b: &Complex) -> Complex {
    let div = b.magnitude_squared();
    multiply(a, &b.conj()).scale(1.0 / div)
}

/// Returns a - b.
#[wasm_bindgen(js_name = ComplexFunctions_Subtract)]
pub fn subtract(a: &Complex, b: &Complex) -> Complex {
    Complex::new(a.re - b.re, a.im - b.im)
}

/// Returns Re(z).
#[wasm_bindgen(js_name = ComplexFunctions_Re)]
pub fn re(z: &Complex) -> f64 {
    z.re
}

/// Returns Im(z).
#[wasm_bindgen(js_name = ComplexFunctions_Im)]
pub fn im(z: &Complex) -> f64 {
    z.im
}

#[wasm_bindgen(js_name = ComplexFunctions_Construct)]
pub fn construct(a: f64, b: f64) -> Complex {
    Complex::new(a, b)
}

// TODO: `piecewise` is variadic function. How to implement in Rust?

#[wasm_bindgen(js_name = ComplexFunctions_Abs)]
pub fn abs(z: &Complex) -> f64 {
    z.magnitude()
}

#[wasm_bindgen(js_name = ComplexFunctions_IsFinite)]
pub fn is_finite(z: &Complex) -> bool {
    z.re.is_finite() && z.im.is_finite()
}
