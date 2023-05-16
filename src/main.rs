mod utils;
mod butter;

use filter_utils::Filter;
use butter::Butt2Ord;
use plotly::{Plot, Scatter};
use utils::polyeval;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut f = Butt2Ord::zero();
    f.init(10f64, 100f64);

    let (mag, freq) = f.abs(32);
    let mag: Vec<f64> = mag.into_iter().map(|val| 20. * val.log10()).collect();

    let mut plot = Plot::new();
    let scatter = Scatter::new(freq, mag);
    plot.add_trace(scatter);

    plot.write_html("output.html");

    Ok(())
}
