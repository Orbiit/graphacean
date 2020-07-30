// use super::super::math::{bounding_box::BoundingBox, gamma_function::ln_gamma, vec2::Vec2};
use std::ops::{Add, Rem};

/// Non-stupid mod function
pub fn modulo<N>(n: N, m: N) -> N
where
    N: Copy + Add<Output = N> + Rem<Output = N>,
{
    ((n % m) + m) % m
}

const eulerGamma: f64 = 0.57721566490153286060;
