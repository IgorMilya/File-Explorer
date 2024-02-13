pub fn bytes_to_gigabytes(bytes: u64) -> f64 {
    let number = (bytes as f64) / (1024.0 * 1024.0 * 1024.0);
    (number * 100.0).round() / 100.0
}