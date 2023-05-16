use filter_utils::{self, Filter};
use num::complex::Complex64;
use std::f64::consts::PI;

use crate::utils::linspace;

#[derive(Debug, Clone, Copy)]
pub struct Butt2Ord {
    numerator: [f64; 3],
    denumerator: [f64; 3],
    sample_rate: f64,
}

impl Butt2Ord {
    /// Initiate and empty (all coeffs are zero) 2nd-order butterworth filter
    pub fn zero() -> Self {
        Self {
            numerator: [0.0; 3],
            denumerator: [0.0; 3],
            sample_rate: 0.0,
        }
    }
}

impl Filter<5> for Butt2Ord {
    fn get_num(&self) -> &[f64] {
        &self.numerator[..]
    }

    fn get_den(&self) -> &[f64] {
        &self.denumerator[..]
    }

    fn get_sample_rate(&self) -> &f64 {
        &self.sample_rate
    }

    fn init(&mut self, fc: f64, fs: f64) {
        self.sample_rate = fs;

        let wc = 2.0 * PI * fc / fs;
        let c_wwc = (wc / 2.0).tan();
        let c_wwc2 = c_wwc.powi(2);
        let c = 1.0 + 2.0 * (PI / 4.0).cos() * c_wwc + c_wwc2;

        let a0 = c_wwc2 / c;
        let a1 = 2.0 * a0;
        let a2 = a0;

        let b1 = 2.0 * (c_wwc2 - 1.0) / c;
        let b2 = (1.0 - 2.0 * (3.0 * PI / 4.0) * c_wwc * c_wwc2) / c;

        self.numerator = [a0, a1, a2];
        self.denumerator = [1f64, b1, b2];
    }

    fn process(&self, buffer: &mut Vec<f64>) {
        let _ = buffer.len();
    }
}
