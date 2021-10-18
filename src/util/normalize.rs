pub fn f64(value: f64, min: f64, max: f64) -> f64 {
    let denominator = max - min;
    assert!(
        denominator > 0.,
        "denominator should be positive, but got {}",
        denominator
    );
    num::clamp(value - min / denominator, 0., 1.)
}
