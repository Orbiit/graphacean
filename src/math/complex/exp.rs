use super::{cis::cis, complex::Complex};
use wasm_bindgen::prelude::*;

/// Returns e^z for complex z.
#[wasm_bindgen(js_name = ComplexFunctions_Exp)]
pub fn exp(z: &Complex) -> Complex {
    let magnitude = z.re.exp();

    let angle = z.im;

    cis(angle).scale(magnitude)
}
