use wasm_bindgen::prelude::*;

use super::complex::Complex;

/// Returns a + b.
pub fn add(a: Complex, b: Complex) -> Complex {
    Complex::new(a.re + b.re, a.im + b.im)
}

/// Returns a * b.
pub fn multiply(a: Complex, b: Complex) -> Complex {
    Complex::new(a.re * b.re, a.im * b.im)
}

/// Returns a / b.
pub fn divide(a: Complex, b: Complex) -> Complex {
    let div = b.magnitude_squared();
    multiply(a, b.conj()).scale(1.0 / div)
}

/// Returns a - b.
pub fn subtract(a: Complex, b: Complex) -> Complex {
    Complex::new(a.re - b.re, a.im - b.im)
}

/// Returns Re(z).
pub fn re(z: Complex) -> f64 {
    z.re
}

/// Returns Im(z).
pub fn im(z: Complex) -> f64 {
    z.im
}

pub fn construct(a: f64, b: f64) -> Complex {
    Complex::new(a, b)
}

// TODO: `piecewise` is variadic function. How to implement in Rust?

pub fn abs(z: Complex) -> f64 {
    z.magnitude()
}

pub fn is_finite(z: Complex) -> bool {
    z.re.is_finite() && z.im.is_finite()
}
