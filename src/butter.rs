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
    fn get_num(&self, index: usize) -> Option<&f64> {
        self.numerator.get(index)
    }

    fn get_den(&self, index: usize) -> Option<&f64> {
        self.denumerator.get(index)
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

impl Butt2Ord {
    pub fn abs(&self, n_segments: usize) -> (Vec<f64>, Vec<f64>) {
        let re_gen = |val: &f64| Complex64::new(*val, 0.0);

        let z_gen = |w: f64| Complex64::new(0.0, w);

        let num = |w: f64| {
            let n0 = re_gen(self.get_num(0).unwrap());
            let n1 = re_gen(self.get_num(1).unwrap()) * z_gen(-w).exp();
            let n2 = re_gen(self.get_num(2).unwrap()) * z_gen(-2.0 * w).exp();

            let d0 = re_gen(&1.0);
            let d1 = re_gen(self.get_den(1).unwrap()) * z_gen(-w).exp();
            let d2 = re_gen(self.get_den(2).unwrap()) * z_gen(-2.0 * w).exp();

            (n0 + n1 + n2) / (d0 + d1 + d2)
        };

        let f_arr = linspace(-PI, PI, n_segments);

        let mut res: Vec<f64> = vec![];

        for f in &f_arr {
            let mag = (num(*f).re.powi(2) + num(*f).im.powi(2)).sqrt();
            res.push(mag);
        }

        (res, f_arr)
    }
}
