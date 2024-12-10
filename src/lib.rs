use std::collections::HashMap;

pub fn pearson_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    let mean_x = mean(x);
    let mean_y = mean(y);
    let mut num = 0.0;
    let mut denom_x = 0.0;
    let mut denom_y = 0.0;
    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        num += dx * dy;
        denom_x += dx * dx;
        denom_y += dy * dy;
    }
    let denom = (denom_x * denom_y).sqrt();
    if denom == 0.0 {
        0.0
    } else {
        num / denom
    }
}

fn mean(vals: &[f64]) -> f64 {
    vals.iter().sum::<f64>() / vals.len() as f64
}

pub fn median(vals: &[u32]) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    let mut sorted = vals.to_vec();
    sorted.sort_unstable();
    let mid = sorted.len()/2;
    if sorted.len()%2==0 {
        (sorted[mid-1]+sorted[mid]) as f64 / 2.0
    } else {
        sorted[mid] as f64
    }
}

pub fn analyze_categorical_data(field_data: HashMap<String, Vec<u32>>, top_n: usize) -> Vec<(String, f64, f64)> {
    let mut stats: Vec<(String, f64, f64)> = field_data.into_iter().map(|(k, vals)| {
        let mean = vals.iter().map(|&v| v as f64).sum::<f64>() / vals.len() as f64;
        let med = median(&vals);
        (k, mean, med)
    }).collect();

    stats.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    stats.into_iter().take(top_n).collect()
}
