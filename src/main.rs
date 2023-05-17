mod butter;
mod plot_utils;
mod utils;

use butter::Butt2Ord;
use filter_utils::Filter;
use num::complex::ComplexFloat;
use plotly::{
    common::{Mode, Title},
    Layout, Plot, Scatter,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fc = 10.;
    let fs = 100.;

    let mut f = Butt2Ord::zero();
    f.init(fc, fs);

    dbg!(f);

    let (w, h) = f.freqz(512);

    let h = h.iter().map(|f| 20. * f64::log10(f.abs())).collect();

    let mut plot = Plot::new();
    let scatter = Scatter::new(w, h).mode(Mode::Lines).name("Mag");
    plot.set_layout(Layout::new().title(Title::new(
        format!("2nd-Order BW, fs = {} Hz, fc = {} Hz", fs, fc).as_str(),
    )));
    plot.add_trace(scatter);

    plot.write_html("output.html");

    Ok(())
}
