use std::f64::consts::PI;
use filter_utils::{self, Filter};

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
            numerator: [0f64; 3],
            denumerator: [0f64; 3],
            sample_rate: 0f64,
        }
    }
}

impl Filter<5> for Butt2Ord {
    fn get_num(&self, index: usize) -> Option<&f64> {
        self.numerator.get(index)
    }

    fn get_den(&self, index: usize) -> Option<&f64> {
        self.denumerator.get(index)
    }

    fn init(&mut self, fc: f64, fs: f64) {
        self.sample_rate = fs;

        let wc = 2f64 * PI * fc / fs;
        let c_wwc = (wc / 2f64).tan();
        let c_wwc2 = c_wwc.powi(2);
        let c = 1f64 + 2f64 * (PI / 4f64).cos() * c_wwc + c_wwc2;

        let a0 = c_wwc2 / c;
        let a1 = 2f64 * a0;
        let a2 = a0;

        let b1 = 2f64 * (c_wwc2 - 1f64) / c;
        let b2 = (1f64 - 2f64 * (3f64 * PI / 4f64) * c_wwc * c_wwc2) / c;

        self.numerator = [a0, a1, a2];
        self.denumerator = [1f64, b1, b2];
    }

    fn abs(&self) -> Vec<f64> {
        linspace(0f64, self.sample_rate, 256)
    }

    fn process(&self, buffer: &mut Vec<f64>) {
        let _ = buffer.len();
    }
}
