pub fn convert_temperature(temperature_type: char, temperature: f64) -> f64 {
    match temperature_type {
        'c' => temperature * 9.0 / 5.0 + 32.0,
        _ => (temperature - 32.0) * 5.0 / 9.0,
    }
}
