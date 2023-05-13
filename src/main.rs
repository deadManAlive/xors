mod utils;
mod butter;

use filter_utils::Filter;
use butter::Butt2Ord;

fn main() {
    let mut f = Butt2Ord::zero();
    f.init(10f64, 100f64);
    println!("{:#?}", f.abs());
}
