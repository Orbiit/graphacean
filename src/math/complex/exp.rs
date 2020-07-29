use super::{cis::cis, complex::Complex};

/// Returns e^z for complex z.
pub fn exp(z: Complex) -> Complex {
    let magnitude = z.re.exp();

    let angle = z.im;

    cis(angle).scale(magnitude)
}
