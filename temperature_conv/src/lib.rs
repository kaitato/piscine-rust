
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
        let celsius = fahrenheit_to_celsius(-459.67);
        assert_eq!(celsius, -273.15);
        let fahrenheit = celsius_to_fahrenheit(0.0);
        assert_eq!(fahrenheit, 32.0);
    }
}
