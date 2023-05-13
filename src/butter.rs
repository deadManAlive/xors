use std::f64::consts::PI;

use filter_utils::{self, Filter};

#[derive(Debug, Clone, Copy)]
pub struct Butt2Ord {
    coeffs: [f64; 5],
}

impl Butt2Ord {
    /// Initiate and empty (all coeffs are zero) 2nd-order butterworth filter
    pub fn zero() -> Self {
        Self { coeffs: [0f64; 5] }
    }
}

impl Filter<5> for Butt2Ord {
    fn get(&self, index: usize) -> Option<&f64> {
        self.coeffs.get(index)
    }

    fn init(&mut self, fc: f64, fs: f64) {
        let wc = 2f64 * PI * fc / fs;
        let c_wwc = (wc/2f64).tan();
        let c_wwc2 = c_wwc.powi(2);
        let c = 1f64 + 2f64 * (PI / 4f64).cos() * c_wwc + c_wwc2;

        let a0 = c_wwc2 / c;
        let a1 = 2f64 * a0;
        let a2 = a0;

        let b1 = 2f64 * (c_wwc2 - 1f64) / c;
        let b2 = (1f64 - 2f64 * (3f64 * PI / 4f64) * c_wwc * c_wwc2) / c;

        self.coeffs = [a0, a1, a2, b1, b2];
    }
}