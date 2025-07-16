use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleNumberInput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SquareRootResult {
    pub result: f64,
    pub input: f64,
    pub is_valid: bool,
    pub error: Option<String>,
}

pub fn calculate_sqrt(input: SingleNumberInput) -> Result<SquareRootResult, String> {
    // Validate input - check for invalid values
    if input.value.is_nan() || input.value.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    let result = if input.value < 0.0 {
        SquareRootResult {
            result: f64::NAN,
            input: input.value,
            is_valid: false,
            error: Some("Cannot compute square root of negative number".to_string()),
        }
    } else {
        SquareRootResult {
            result: input.value.sqrt(),
            input: input.value,
            is_valid: true,
            error: None,
        }
    };
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_squares() {
        let input = SingleNumberInput { value: 9.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 3.0);
        assert_eq!(result.input, 9.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_zero() {
        let input = SingleNumberInput { value: 0.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.input, 0.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_one() {
        let input = SingleNumberInput { value: 1.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.input, 1.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_four() {
        let input = SingleNumberInput { value: 4.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 2.0);
        assert_eq!(result.input, 4.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_non_perfect_square() {
        let input = SingleNumberInput { value: 2.0 };
        let result = calculate_sqrt(input).unwrap();
        assert!((result.result - 2.0_f64.sqrt()).abs() < 1e-15);
        assert_eq!(result.input, 2.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_decimal_input() {
        let input = SingleNumberInput { value: 6.25 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 2.5);
        assert_eq!(result.input, 6.25);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_large_number() {
        let input = SingleNumberInput { value: 10000.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 100.0);
        assert_eq!(result.input, 10000.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_small_number() {
        let input = SingleNumberInput { value: 0.0001 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 0.01);
        assert_eq!(result.input, 0.0001);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_negative_number() {
        let input = SingleNumberInput { value: -4.0 };
        let result = calculate_sqrt(input).unwrap();
        assert!(result.result.is_nan());
        assert_eq!(result.input, -4.0);
        assert!(!result.is_valid);
        assert_eq!(result.error, Some("Cannot compute square root of negative number".to_string()));
    }

    #[test]
    fn test_negative_zero() {
        let input = SingleNumberInput { value: -0.0 };
        let result = calculate_sqrt(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.input, -0.0);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_nan_input_error() {
        let input = SingleNumberInput { value: f64::NAN };
        let result = calculate_sqrt(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = SingleNumberInput { value: f64::INFINITY };
        let result = calculate_sqrt(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_very_small_positive() {
        let input = SingleNumberInput { value: 1e-10 };
        let result = calculate_sqrt(input).unwrap();
        assert!((result.result - 1e-5).abs() < 1e-15);
        assert!(result.is_valid);
        assert!(result.error.is_none());
    }
}