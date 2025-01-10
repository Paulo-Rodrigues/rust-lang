use unity_conversor::convert_temperature;
use unity_conversor::TemperatureUnit;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_temperature() {
        assert_eq!(
            convert_temperature(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius),
            0.0
        )
    }
}
