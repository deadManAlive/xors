mod butter;
mod plot_utils;
mod utils;

use butter::Butt2Ord;
use filter_utils::Filter;
use num::complex::ComplexFloat;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fc = 3000.;
    let fs = 44100.;

    let mut f = Butt2Ord::zero();
    f.init(fc, fs);

    let (w, h) = f.freqz(512);

    let h = h.iter().map(|&f| 20. * f64::log10(f.abs())).collect();

    let plot = plot_utils::semilogx(w, h);

    plot.write_html("output.html");

    Ok(())
}
