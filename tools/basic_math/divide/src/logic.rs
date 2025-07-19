use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoNumberInput {
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticResult {
    pub result: f64,
    pub operation: String,
    pub inputs: Vec<f64>,
}

pub fn divide_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() || input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    // Check for division by zero
    if input.b == 0.0 {
        return Err("Division by zero is not allowed".to_string());
    }

    let result = input.a / input.b;

    Ok(ArithmeticResult {
        result,
        operation: "division".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let input = TwoNumberInput { a: 10.0, b: 2.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 5.0);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![10.0, 2.0]);
    }

    #[test]
    fn test_negative_numbers() {
        let input = TwoNumberInput { a: -10.0, b: -2.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 5.0);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![-10.0, -2.0]);
    }

    #[test]
    fn test_mixed_signs() {
        let input = TwoNumberInput { a: 10.0, b: -2.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, -5.0);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![10.0, -2.0]);
    }

    #[test]
    fn test_zero_dividend() {
        let input = TwoNumberInput { a: 0.0, b: 5.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![0.0, 5.0]);
    }

    #[test]
    fn test_division_by_zero() {
        let input = TwoNumberInput { a: 10.0, b: 0.0 };
        let result = divide_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Division by zero is not allowed");
    }

    #[test]
    fn test_large_numbers() {
        let input = TwoNumberInput { a: 1e10, b: 1e5 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 1e5);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![1e10, 1e5]);
    }

    #[test]
    fn test_small_numbers() {
        let input = TwoNumberInput { a: 1e-10, b: 1e-5 };
        let result = divide_numbers(input).unwrap();
        assert!((result.result - 1e-5).abs() < 1e-15);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![1e-10, 1e-5]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput {
            a: f64::NAN,
            b: 3.0,
        };
        let result = divide_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_infinite_input_error() {
        let input = TwoNumberInput {
            a: 5.0,
            b: f64::INFINITY,
        };
        let result = divide_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_negative_infinite_input_error() {
        let input = TwoNumberInput {
            a: f64::NEG_INFINITY,
            b: 3.0,
        };
        let result = divide_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_decimal_precision() {
        let input = TwoNumberInput { a: 1.0, b: 3.0 };
        let result = divide_numbers(input).unwrap();
        assert!((result.result - 0.333333333333333).abs() < 1e-15);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![1.0, 3.0]);
    }

    #[test]
    fn test_divide_by_one() {
        let input = TwoNumberInput { a: 42.0, b: 1.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 42.0);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![42.0, 1.0]);
    }

    #[test]
    fn test_fraction_result() {
        let input = TwoNumberInput { a: 7.0, b: 2.0 };
        let result = divide_numbers(input).unwrap();
        assert_eq!(result.result, 3.5);
        assert_eq!(result.operation, "division");
        assert_eq!(result.inputs, vec![7.0, 2.0]);
    }
}
