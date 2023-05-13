pub fn linspace(start: f64, stop: f64, num: usize) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();
    
    let delta = (stop - start)/((num - 1)  as f64);

    for i in 0..num {
        res.push(start + (i as f64) * delta);
    }

    res
}