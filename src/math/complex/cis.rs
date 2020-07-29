use super::complex::Complex;

/// Returns e^(i theta) for real theta.
pub fn cis(theta: f64) -> Complex {
    let c = theta.cos();
    let s = theta.sin();
    Complex::new(c, s)
}
