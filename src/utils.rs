pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();
    
    let delta = (stop - start)/((num - 1)  as f64);

    for i in 0..num {
        res.push(start + (i as f64) * delta);
    }

    res
}

pub fn polyeval(coeffs: Vec<f64>, val: f64) -> f64 {
    if coeffs.is_empty() {
        return 0.;
    }

    let mut res = *coeffs.last().unwrap();

    for (i, c) in coeffs.into_iter().rev().enumerate() {
        if i == 0 {
            continue;
        }
        res = res * val + c;
    }

    res
}