use super::super::vec::Vec2;
use super::super::complex::Complex;
use wasm_bindgen::prelude::*;

/// Add two Vec2's and return it.
#[wasm_bindgen(js_name = VectorFunctions_Add)]
pub fn add(v1: &Vec2, v2: &Vec2) -> Vec2 {
    Vec2::new(v1.x + v2.x, v1.y + v2.y)
}

/// Subtract two Vec2's and return it.
#[wasm_bindgen(js_name = VectorFunctions_Subtract)]
pub fn subtract(v1: &Vec2, v2: &Vec2) -> Vec2 {
    Vec2::new(v1.x - v2.x, v1.y - v2.y)
}

/// Return the dot product of two Vec2's.
#[wasm_bindgen(js_name = VectorFunctions_Dot)]
pub fn dot(v1: &Vec2, v2: &Vec2) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

/// Construct and return a new Vec2.
#[wasm_bindgen(js_name = VectorFunctions_Construct)]
pub fn construct(x: f64, y: f64) -> Vec2 {
    Vec2::new(x, y)
}

#[wasm_bindgen(js_name = VectorFunctions_FromComplex)]
pub fn from_complex(z: &Complex) -> Vec2 {
    Vec2::new(z.re, z.im)
}