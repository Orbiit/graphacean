use super::{
    basic_arithmetic::{add, multiply},
    cis::cis,
    complex::Complex,
    exp::exp,
};
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;

/// Return the principal value of z^w.
#[wasm_bindgen(js_name = Pow)]
pub fn pow(z: &Complex, w: &Complex) -> Complex {
    exp(&multiply(w, &Complex::new(z.magnitude().ln(), z.arg())))
}

/// Multivalued version of z^w.
#[wasm_bindgen(js_name = PowBranched)]
pub fn pow_branched(z: &Complex, w: &Complex, branch: i32) -> Complex {
    multiply(
        &pow(z, w),
        &exp(&multiply(
            &Complex::i(),
            &w.scale(2.0 * PI * (branch as f64)),
        )),
    )
}

/// z^r, where r is a real number.
#[wasm_bindgen(js_name = PowR)]
pub fn pow_r(z: &Complex, r: f64) -> Complex {
    pow(z, &Complex::new(r, 0.0))
}

#[wasm_bindgen(js_name = PowZ)]
pub fn pow_z(r: f64, z: &Complex) -> Complex {
    if r == 0.0 {
        Complex::new(0.0, 0.0)
    } else {
        exp(&multiply(
            z,
            &Complex::new(r.abs().ln(), if r > 0.0 { 0.0 } else { PI }),
        ))
    }
}

/// z^r, where r is a real number, branched.
#[wasm_bindgen(js_name = PowRBranched)]
pub fn pow_r_branched(z: &Complex, r: f64, branch: i32) -> Complex {
    pow_branched(z, &Complex::new(r, 0.0), branch)
}

/// Returns z^n, where n is an integer
#[wasm_bindgen(js_name = PowN)]
pub fn pow_n(z: &Complex, n: i32) -> Complex {
    match n {
        0 => Complex::new(1.0, 0.0),
        1 => z.clone(),
        -1 => z.conj().scale(1.0 / z.magnitude_squared()),
        2 => multiply(z, z),
        _ => {
            let mag = z.magnitude();
            let angle = z.arg();

            let new_mag = mag.powi(n);
            let new_angle = angle * (n as f64);

            cis(new_angle).scale(new_mag)
        }
    }
}

/// Returns the principal value of sqrt(z).
#[wasm_bindgen(js_name = Sqrt)]
pub fn sqrt(z: &Complex) -> Complex {
    if z.im.abs() < 1.0e-17 {
        let r = z.re;

        if r >= 0.0 {
            Complex::new(r.sqrt(), 0.0)
        } else {
            Complex::new(0.0, (-r).sqrt())
        }
    } else {
        let r = z.magnitude();

        let z_r = add(z, &Complex::new(r, 0.0)).normalize();

        z_r.scale(r.sqrt())
    }
}

/// Branched version of Sqrt(z).
#[wasm_bindgen(js_name = SqrtBranched)]
pub fn sqrt_branched(z: &Complex, branch: i32) -> Complex {
    if branch % 2 == 0 {
        sqrt(z)
    } else {
        multiply(&Complex::new(-1.0, 0.0), &sqrt(z))
    }
}

/// Principal value of cbrt(z).
#[wasm_bindgen(js_name = Cbrt)]
pub fn cbrt(z: &Complex) -> Complex {
    pow_r(z, 1.0 / 3.0)
}

/// Multivalued version of Cbrt(z).
#[wasm_bindgen(js_name = CbrtBranched)]
pub fn cbrt_branched(z: &Complex, branch: i32) -> Complex {
    pow_r_branched(z, 1.0 / 3.0, branch)
}
