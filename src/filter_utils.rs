//! # Filter utilities
mod utils;

use num::complex::Complex64;
use std::f64::consts::PI;
use utils::{linspace, polyeval};

/// Generic filter trait.
pub trait Filter<const N: usize>
where
    Self: Clone + Copy,
{
    fn get_num(&self) -> &[f64];
    fn get_den(&self) -> &[f64];
    fn get_sample_rate(&self) -> &f64;
    /// Calculate coefficients from cutoff freq (fc) and sample
    /// rate (fs). Both in Hz.
    fn init(&mut self, fc: f64, fs: f64);
    // fn abs(&self, n_segments: usize) -> Vec<f64>; // NOTE: unnecessary for processing trait
    fn process(&self, buffer: &mut Vec<f64>);
    fn freqz(&self, n_segment: usize) -> (Vec<f64>, Vec<Complex64>) {
        let w = linspace(0., PI, n_segment);
        let w_norm: Vec<f64> = w
            .clone()
            .into_iter()
            .map(|val| 2. * PI * val / self.get_sample_rate())
            .collect();

        let mut h = Vec::with_capacity(w_norm.len());

        for f in &w_norm {
            let zm1 = Complex64::new(0., -f).exp();
            let num_eval = polyeval(self.get_num().to_vec(), zm1);
            let den_eval = polyeval(self.get_den().to_vec(), zm1);

            h.push(num_eval / den_eval);
        }

        (w, h)
    }
}
