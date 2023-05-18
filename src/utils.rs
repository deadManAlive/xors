#![allow(dead_code)]

use std::ops::{Add, Mul};

use num::{
    complex::{Complex64, ComplexFloat},
    Float, Zero,
};
use rustfft::FftPlanner;

pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();

    let delta = (stop - start) / ((num - 1) as f64);

    for i in 0..num {
        res.push(start + (i as f64) * delta);
    }

    res
}

/// Polynomial evaluating function
/// based on Horner's method.
///
/// * `k`: coefficient matrix with index corresponds to polynomial coefficient index.
/// * `val`: value to evaluate.
///
/// Generic is used so it is possible to evaluate on various types, including complex.
pub fn polyeval<T: Float, U>(k: Vec<T>, val: U) -> U
where
    U: Default + Copy + From<T> + Mul<U>,
    <U as Mul>::Output: Add<T, Output = U>,
{
    if k.is_empty() {
        return Default::default();
    }

    let mut res = U::from(*k.last().unwrap());

    for (i, c) in k.iter().rev().enumerate() {
        if i == 0 {
            continue;
        }
        res = res * val + *c;
    }

    res
}

pub fn polymul(c0: &[f64], c1: &[f64]) -> Vec<f64> {
    let res_deg = c0.len() + c1.len() - 1;

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(res_deg);
    let ifft = planner.plan_fft_inverse(res_deg);

    let mut w0: Vec<Complex64> = c0.iter().map(Complex64::from).collect();
    let mut w1: Vec<Complex64> = c1.iter().map(Complex64::from).collect();

    w0.resize(res_deg, Complex64::zero());
    w1.resize(res_deg, Complex64::zero());

    fft.process(&mut w0);
    fft.process(&mut w1);

    let mut res: Vec<Complex64> = w0.iter().zip(w1.iter()).map(|(a, b)| a * b).collect();

    ifft.process(&mut res);

    res.iter().map(|val| val.abs() / (res_deg as f64)).collect() // NOTE: who could guessed the ifft process isn't normalized?
}

#[cfg(test)]
mod tests {
    use super::{linspace, polyeval};

    #[test]
    fn ls() {
        let l = linspace(0., 1., 5);
        let r = vec![0., 0.25, 0.5, 0.75, 1.];
        assert_eq!(l, r);
    }

    #[test]
    fn pe() {
        let l = polyeval(vec![-1., 2., -6., 2.], 3.);
        let r = 5.;
        assert_eq!(l, r);
    }
}
