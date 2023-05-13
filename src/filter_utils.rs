//! # Filter utilities

/// Generic filter trait.
pub trait Filter<const N: usize>
where
    Self: Clone + Copy,
{
    fn get_num(&self, index: usize) -> Option<&f64>;
    fn get_den(&self, index: usize) -> Option<&f64>;
    /// Calculate coefficients from cutoff freq (fc) and sample
    /// rate (fs). Both in Hz.
    fn init(&mut self, fc: f64, fs: f64);
    fn abs(&self) -> Vec<f64>;
    fn process(&self, buffer: &mut Vec<f64>);
}
