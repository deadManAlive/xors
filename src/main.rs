mod butter;
mod utils;

use butter::Butt2Ord;
use filter_utils::Filter;
use num::complex::ComplexFloat;
use plotly::{Plot, Scatter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = Butt2Ord::zero();
    f.init(10f64, 100f64);

    let (w, h) = f.freqz(16);

    dbg!(&h);

    let h = h.into_iter().map(|f| 20. * f64::log10(f.abs())).collect();

    let mut plot = Plot::new();
    let scatter = Scatter::new(w, h);
    plot.add_trace(scatter);

    plot.write_html("output.html");

    Ok(())
}
