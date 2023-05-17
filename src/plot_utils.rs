#![allow(dead_code)]

use plotly::{
    common::{Mode, Title},
    layout::Axis,
    Layout, Plot, Scatter,
};

pub fn semilogx(x: Vec<f64>, y: Vec<f64>) -> Plot {
    let x_log: Vec<f64> = x.iter().map(|&val| val.log10()).collect();

    let line = Scatter::new(x_log, y).mode(Mode::Lines);

    let mut plot = Plot::new();

    let layout = Layout::new()
        .x_axis(Axis::new().title(Title::new("f (10^N Hz)")))
        .y_axis(Axis::new().title(Title::new("Mag (dB)")));

    plot.add_trace(line);
    plot.set_layout(layout);

    plot
}
