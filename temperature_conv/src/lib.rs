
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * (9.0/5.0)) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let fahrenheit_to_celsius = fahrenheit_to_celsius(-459.67);
        assert_eq!(fahrenheit_to_celsius, -273.15);
        let celsius_to_fahrenheit = celsius_to_fahrenheit(0.0);
        assert_eq!(celsius_to_fahrenheit, 32.0);
    }
}
