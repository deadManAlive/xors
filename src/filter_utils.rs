//! # Filter utilities

use std::fmt::Debug;

/// Generic filter trait.
///
/// * `N`: generic parameter with the number of
/// filter coefficients which is 2M + 1, where M
/// is filter order.
pub trait Filter<const N: usize>
where
    Self: Clone + Copy,
{
    fn get(&self, index: usize) -> Option<&f64>;
    /// Calculate coefficients from cutoff freq (fc) and sample
    /// rate (fs). Both in Hz.
    fn init(&mut self, fc: f64, fs: f64);
}

/// Generic struct implementation.
#[derive(Debug, Clone, Copy)]
pub struct GenericFilter<const N: usize> {
    coeffs: [f64; N],
}

impl<const N: usize> GenericFilter<N> {
    pub fn new() -> Self {
        GenericFilter::<N> { coeffs: [0f64; N] }
    }
}

impl GenericFilter<5> {
    pub fn set_coeffs(c0: f64, c1: f64, c2: f64, c3: f64, c4: f64) -> Self {
        Self {
            coeffs: [c0, c1, c2, c3, c4],
        }
    }
}

impl<const N: usize> Filter<N> for GenericFilter<N> {
    fn get(&self, index: usize) -> Option<&f64> {
        self.coeffs.get(index)
    }

    fn init(&mut self, fc: f64, fs: f64) {
        let _ = (fc, fs);
    }
}

#[macro_export]
macro_rules! filt2ord {
    () => {
        $crate::GenericFilter::<5>::new()
    };
    ($c0:expr, $c1:expr, $c2:expr, $c3:expr, $c4:expr) => {
        $crate::GenericFilter::<5>::set_coeffs($c0, $c1, $c2, $c3, $c4)
    };
}

#[macro_export]
macro_rules! filtNord {
    ($order:expr) => {
        $crate::GenericFilter::<order>::new()
    };
}
