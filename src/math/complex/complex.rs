use wasm_bindgen::prelude::*;

/// Represents a complex number, with a real part and an imaginary part both represented by floats.
#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

#[wasm_bindgen]
impl Complex {
    /// Construct a new complex number.
    ///
    /// # Arguments
    ///
    /// * `re` - The real part of the complex number.
    ///
    /// * `im` - The imaginary part of the complex number.
    pub fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Get i.
    pub fn i() -> Self {
        Self { re: 0.0, im: 1.0 }
    }

    /// Get 1.
    pub fn one() -> Self {
        Self { re: 1.0, im: 0.0 }
    }

    /// Return the complex argument (principal value) corresponding to the complex number.
    pub fn arg(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Returns |z|.
    pub fn magnitude(&self) -> f64 {
        self.re.hypot(self.im)
    }

    /// Returns |z|^2.
    pub fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Returns z bar.
    pub fn conj(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Scale this complex number by the real factor r.
    pub fn scale(&self, r: f64) -> Self {
        Self {
            re: self.re * r,
            im: self.im * r,
        }
    }

    /// Return a complex number pointing in the same direction, with magnitude 1.
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self.scale(1.0 / mag)
    }
}
