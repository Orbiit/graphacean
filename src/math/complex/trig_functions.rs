use super::{super::super::core::utils::modulo, basic_arithmetic::divide, complex::Complex};
use wasm_bindgen::prelude::*;

/// sin(a+bi) = sin a cosh b + i cos a sinh b
#[wasm_bindgen(js_name = ComplexFunctions_Sin)]
pub fn sin(z: &Complex) -> Complex {
    let (a, b) = (z.re, z.im);
    let sin_a = a.sin();
    let cos_a = a.cos();

    let sinh_b = b.sinh();
    let cosh_b = (1.0 + sinh_b * sinh_b).sqrt();

    Complex::new(sin_a * cosh_b, cos_a * sinh_b)
}

/// cos(a+bi) = cos a cosh b - i sin a sinh b
#[wasm_bindgen(js_name = ComplexFunctions_Cos)]
pub fn cos(z: &Complex) -> Complex {
    let (a, b) = (z.re, z.im);
    let sin_a = a.sin();
    let cos_a = a.cos();

    let sinh_b = b.sinh();
    let cosh_b = (1.0 + sinh_b * sinh_b).sqrt();

    Complex::new(cos_a * cosh_b, -sin_a * sinh_b)
}

/// tan(a+bi) = (tan a + i tanh b) / (1 - i tan a tanh b)
#[wasm_bindgen(js_name = ComplexFunctions_Tan)]
pub fn tan(z: &Complex) -> Complex {
    let (a, b) = (z.re, z.im);

    let tan_a = a.tan();
    let tanh_b = b.tanh();

    divide(
        &Complex::new(tan_a, tanh_b),
        &Complex::new(1.0, -tan_a * tanh_b),
    )
}

/// sec(a+bi) = 1 / cos(a+bi)
#[wasm_bindgen(js_name = ComplexFunctions_Sec)]
pub fn sec(z: &Complex) -> Complex {
    divide(&Complex::one(), &cos(z))
}

/// csc(a+bi) = 1 / sin(a+bi)
#[wasm_bindgen(js_name = ComplexFunctions_Csc)]
pub fn csc(z: &Complex) -> Complex {
    divide(&Complex::one(), &sin(z))
}

/// cot(a+bi) = 1 / tan(a+bi)
#[wasm_bindgen(js_name = ComplexFunctions_Cot)]
pub fn cot(z: &Complex) -> Complex {
    divide(&Complex::one(), &tan(z))
}
