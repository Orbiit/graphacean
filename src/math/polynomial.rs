use super::complex::{
    basic_arithmetic::{add, multiply},
    Complex,
};

fn multiply_polynomials(coeffs1: &Vec<f64>, coeffs2: &Vec<f64>, degree: usize) -> Vec<f64> {
    let mut ret = vec![0.0; degree + 1];

    for (i, coeff1) in coeffs1.iter().enumerate() {
        for (j, coeff2) in coeffs2.iter().enumerate() {
            ret[i + j] += coeff1 * coeff2;
        }
    }

    ret
}

#[derive(Clone, Debug, PartialEq)]
pub struct SingleVariablePolynomial {
    /// Order: first is constant, second is linear, etc.
    coeffs: Vec<f64>,
}

impl SingleVariablePolynomial {
    pub fn new(coeffs: Vec<f64>) -> Self {
        Self { coeffs }
    }

    fn evaluate_float(&self, x: f64) -> f64 {
        let mut prod = 1.0;
        let mut sum = 0.0;

        for coeff in &self.coeffs {
            sum += coeff * prod;

            prod *= x;
        }

        sum
    }

    pub fn evaluate_complex(&self, z: Complex) -> Complex {
        let mut prod = Complex::one();
        let mut sum = Complex::new(0.0, 0.0);

        for coeff in &self.coeffs {
            sum = add(&sum, &multiply(&Complex::new(*coeff, 0.0), &prod));

            prod = multiply(&prod, &z);
        }

        sum
    }

    pub fn evaluate(&self, x: f64) -> f64 {
        let mut prod = 1.0;
        let mut sum = 0.0;

        for coeff in &self.coeffs {
            if *coeff != 0.0 {
                sum += coeff * prod;
            }

            prod *= x;
        }

        sum
    }

    pub fn degree(&self) -> usize {
        self.coeffs.len() - 1
    }

    pub fn derivative(&self) -> Self {
        let mut new_coeffs = Vec::with_capacity(self.coeffs.len() - 1);

        for (i, coeff) in self.coeffs.iter().enumerate() {
            new_coeffs.push(i as f64 * coeff);
        }

        Self::new(new_coeffs)
    }

    pub fn clone(&self) -> Self {
        Self {
            coeffs: self.coeffs.clone(),
        }
    }

    pub fn add(&mut self, poly: Self) -> &Self {
        self.coeffs.resize(poly.coeffs.len(), 0.0);
        for (i, other_coeff) in poly.coeffs.iter().enumerate() {
            self.coeffs[i] += other_coeff;
        }
        self
    }

    pub fn subtract(&mut self, poly: Self) -> &Self {
        self.coeffs.resize(poly.coeffs.len(), 0.0);
        for (i, other_coeff) in poly.coeffs.iter().enumerate() {
            self.coeffs[i] -= other_coeff;
        }
        self
    }

    pub fn multiply(&mut self, poly: Self) -> &Self {
        self.coeffs =
            multiply_polynomials(&poly.coeffs, &self.coeffs, poly.degree() + self.degree());
        self
    }

    pub fn integral() {
        unimplemented!()
    }
}
