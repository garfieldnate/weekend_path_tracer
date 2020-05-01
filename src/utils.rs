pub fn clamp(n: f64, min: f64, max: f64) -> f64 {
    n.max(min).min(max)
}
