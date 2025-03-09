
pub fn round_float<T: Into<f64>>(f: T, places: u8) -> f64 {
    let multiplier = 10u32.pow(places as u32) as f64;
    (f.into() * multiplier).round() / multiplier
}
