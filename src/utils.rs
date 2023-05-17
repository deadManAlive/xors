#![allow(dead_code)]

use std::ops::{Add, Mul};

use num::Float;

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
/// * `coeffs`: coefficient matrix with index corresponds to polynomial coefficient index.
/// * `val`: value to evaluate.
///
/// Generic is used so it is possible to evaluate on various types, including complex.
pub fn polyeval<T: Float, U>(coeffs: Vec<T>, val: U) -> U
where
    U: Default + Copy + From<T> + Mul<U>,
    <U as Mul>::Output: Add<T, Output = U>,
{
    if coeffs.is_empty() {
        return Default::default();
    }

    let mut res = U::from(*coeffs.last().unwrap());

    for (i, c) in coeffs.iter().rev().enumerate() {
        if i == 0 {
            continue;
        }
        res = res * val + *c;
    }

    res
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
