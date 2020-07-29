use super::{basic_arithmetic::divide, complex::Complex};

/// Returns sinh(z).
pub fn sinh(z: Complex) -> Complex {
    let (a, b) = (z.re, z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(sinh_a * cos_b, cosh_a * sin_b)
}

/// Returns cosh(z).
pub fn cosh(z: Complex) -> Complex {
    let (a, b) = (z.re, z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(cosh_a * cos_b, sinh_a * sin_b)
}

/// Returns tanh(z).
pub fn tanh(z: Complex) -> Complex {
    let (a, b) = (2.0 * z.re, 2.0 * z.im);

    let sinh_a = a.sinh();
    let cosh_a = (1.0 + sinh_a * sinh_a).sqrt();

    let sin_b = b.sin();
    let cos_b = b.cos();

    Complex::new(sinh_a, sin_b).scale(1.0 / (cosh_a + cos_b))
}

/// Returns sech(z).
pub fn sech(z: Complex) -> Complex {
    divide(Complex::one(), cosh(z))
}

/// Returns csch(z).
pub fn csch(z: Complex) -> Complex {
    divide(Complex::one(), sinh(z))
}

/// Returns coth(z).
pub fn coth(z: Complex) -> Complex {
    divide(Complex::one(), tanh(z))
}
