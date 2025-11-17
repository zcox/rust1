//! A library for temperature conversion between Celsius and Fahrenheit.
//!
//! This library provides functions to convert temperatures and validate
//! that they are above absolute zero.

/// Error type for temperature conversion operations
#[derive(Debug, PartialEq)]
pub enum TempError {
    /// Temperature is below absolute zero
    BelowAbsoluteZero,
}

/// Converts Celsius to Fahrenheit
///
/// # Arguments
///
/// * `celsius` - Temperature in Celsius
///
/// # Returns
///
/// * `Result<f64, TempError>` - Temperature in Fahrenheit or error if below absolute zero
///
/// # Examples
///
/// ```
/// use rust1::celsius_to_fahrenheit;
///
/// let result = celsius_to_fahrenheit(0.0).unwrap();
/// assert_eq!(result, 32.0);
/// ```
pub fn celsius_to_fahrenheit(celsius: f64) -> Result<f64, TempError> {
    // Absolute zero in Celsius is -273.15
    if celsius < -273.15 {
        return Err(TempError::BelowAbsoluteZero);
    }

    Ok(celsius * 9.0 / 5.0 + 32.0)
}

/// Converts Fahrenheit to Celsius
///
/// # Arguments
///
/// * `fahrenheit` - Temperature in Fahrenheit
///
/// # Returns
///
/// * `Result<f64, TempError>` - Temperature in Celsius or error if below absolute zero
///
/// # Examples
///
/// ```
/// use rust1::fahrenheit_to_celsius;
///
/// let result = fahrenheit_to_celsius(32.0).unwrap();
/// assert_eq!(result, 0.0);
/// ```
pub fn fahrenheit_to_celsius(fahrenheit: f64) -> Result<f64, TempError> {
    // Absolute zero in Fahrenheit is -459.67
    if fahrenheit < -459.67 {
        return Err(TempError::BelowAbsoluteZero);
    }

    Ok((fahrenheit - 32.0) * 5.0 / 9.0)
}

// Unit tests - these run when you execute `cargo test`
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_to_fahrenheit_freezing() {
        let result = celsius_to_fahrenheit(0.0).unwrap();
        assert_eq!(result, 32.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit_boiling() {
        let result = celsius_to_fahrenheit(100.0).unwrap();
        assert_eq!(result, 212.0);
    }

    #[test]
    fn test_celsius_to_fahrenheit_negative() {
        let result = celsius_to_fahrenheit(-40.0).unwrap();
        assert_eq!(result, -40.0); // -40C = -40F
    }

    #[test]
    fn test_celsius_below_absolute_zero() {
        let result = celsius_to_fahrenheit(-300.0);
        assert_eq!(result, Err(TempError::BelowAbsoluteZero));
    }

    #[test]
    fn test_fahrenheit_to_celsius_freezing() {
        let result = fahrenheit_to_celsius(32.0).unwrap();
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius_boiling() {
        let result = fahrenheit_to_celsius(212.0).unwrap();
        assert_eq!(result, 100.0);
    }

    #[test]
    fn test_fahrenheit_to_celsius_negative() {
        let result = fahrenheit_to_celsius(-40.0).unwrap();
        assert_eq!(result, -40.0); // -40F = -40C
    }

    #[test]
    fn test_fahrenheit_below_absolute_zero() {
        let result = fahrenheit_to_celsius(-500.0);
        assert_eq!(result, Err(TempError::BelowAbsoluteZero));
    }

    #[test]
    fn test_absolute_zero_celsius() {
        let result = celsius_to_fahrenheit(-273.15).unwrap();
        assert!((result - (-459.67)).abs() < 0.01); // Allow small floating point error
    }

    #[test]
    fn test_absolute_zero_fahrenheit() {
        let result = fahrenheit_to_celsius(-459.67).unwrap();
        assert!((result - (-273.15)).abs() < 0.01); // Allow small floating point error
    }
}
