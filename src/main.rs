mod utils;
mod butter;

use filter_utils::Filter;
use butter::Butt2Ord;

fn main() {
    let mut f = Butt2Ord::zero();
    f.init(10f64, 100f64);

    let mag = f.abs(32);
    let mag: Vec<f64> = mag.into_iter().map(|val| 20.0 * val.log10()).collect();

    println!("{:#?}", mag);
}
