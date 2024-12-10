use std::collections::HashMap;
use project::*;

#[test]
fn test_pearson_correlation() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
    let corr = pearson_correlation(&x, &y);
    assert!((corr - 1.0).abs() < 1e-6);
}

#[test]
fn test_pearson_correlation_no_relation() {
    let x = vec![1.0, 2.0, 3.0];
    let y = vec![10.0, 10.0, 10.0];
    let corr = pearson_correlation(&x, &y);
    assert_eq!(corr, 0.0);
}

#[test]
fn test_median_even() {
    let vals = vec![2, 4, 6, 8];
    let med = median(&vals);
    assert_eq!(med, (4+6) as f64 / 2.0);
}

#[test]
fn test_median_odd() {
    let vals = vec![3, 1, 2];
    let med = median(&vals);
    assert_eq!(med, 2.0);
}

#[test]
fn test_analyze_categorical_data() {
    let mut field_data = HashMap::new();
    field_data.insert("DevA".to_string(), vec![100, 200, 300]);
    field_data.insert("DevB".to_string(), vec![1000, 5000]);
    field_data.insert("DevC".to_string(), vec![50, 50, 50]);
    let top_2 = analyze_categorical_data(field_data, 2);
    assert_eq!(top_2.len(), 2);
    assert_eq!(top_2[0].0, "DevB");
    assert_eq!(top_2[1].0, "DevA");
}

#[test]
fn test_analyze_categorical_data_empty() {
    let field_data: HashMap<String, Vec<u32>> = HashMap::new();
    let top = analyze_categorical_data(field_data, 5);
    assert!(top.is_empty());
}
