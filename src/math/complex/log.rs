use super::{
    basic_arithmetic::{add, divide},
    complex::Complex,
};
use std::f64::consts::{LN_10, LN_2, PI};

/// Returns ln(z), where ln is the natural logarithm.
pub fn ln(z: &Complex) -> Complex {
    let mag = z.magnitude().ln();
    let theta = z.arg();

    Complex::new(mag, theta)
}

/// The multivalued version of ln(z). In other words, if ln(z) is the principal value of ln(z), it
/// returns ln(z) + 2 * pi * i * branch, where branch is an integer.
pub fn ln_branched(z: &Complex, branch: i32) -> Complex {
    add(&ln(z), &Complex::i().scale(2.0 * PI * (branch as f64)))
}

// TODO: `log` aliases for the ln functions.

/// log10(z) (principal value)
pub fn log10(z: &Complex) -> Complex {
    ln(z).scale(1.0 / LN_10)
}

/// log10(z) (branched)
pub fn log10_branched(z: &Complex, branch: i32) -> Complex {
    ln_branched(z, branch).scale(1.0 / LN_10)
}

/// log2(z) (principal value)
pub fn log2(z: &Complex) -> Complex {
    ln(z).scale(1.0 / LN_2)
}

/// log2(z) (branched)
pub fn log2_branched(z: &Complex, branch: i32) -> Complex {
    ln_branched(z, branch).scale(1.0 / LN_2)
}

/// Log base b of z
pub fn log_b(b: &Complex, z: &Complex) -> Complex {
    if b == z {
        Complex::one()
    } else {
        divide(&ln(z), &ln(b))
    }
}

/// Log base b of z, multivalued
pub fn log_b_branched(b: &Complex, z: &Complex, branch: i32) -> Complex {
    if branch == 0 && b == z {
        Complex::one()
    } else {
        divide(&ln_branched(z, branch), &ln_branched(b, branch))
    }
}
