pub fn convert_to_u64(amount: f64) -> u64 {
    (amount * f64::powf(10.0.into(), 9.into())) as u64
}
