use std::env;

pub fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    }
}

pub fn byte_to_gb(byte: u64) -> f64 {
    byte as f64 / 1_000_000_000.0
}

pub fn round_float(num: f64, scale: u32) -> f64 {
    let base = u64::pow(10, scale) as f64;
    (num * base).round() / base
}
