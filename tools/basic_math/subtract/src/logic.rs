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

pub fn subtract_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() || input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    let result = input.a - input.b;

    Ok(ArithmeticResult {
        result,
        operation: "subtraction".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let input = TwoNumberInput { a: 5.0, b: 3.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, 2.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![5.0, 3.0]);
    }

    #[test]
    fn test_negative_numbers() {
        let input = TwoNumberInput { a: -5.0, b: -3.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, -2.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![-5.0, -3.0]);
    }

    #[test]
    fn test_mixed_signs() {
        let input = TwoNumberInput { a: 10.0, b: -3.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, 13.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![10.0, -3.0]);
    }

    #[test]
    fn test_zero_values() {
        let input = TwoNumberInput { a: 0.0, b: 0.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![0.0, 0.0]);
    }

    #[test]
    fn test_zero_subtraction() {
        let input = TwoNumberInput { a: 42.0, b: 0.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, 42.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![42.0, 0.0]);
    }

    #[test]
    fn test_large_numbers() {
        let input = TwoNumberInput { a: 3e10, b: 1e10 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, 2e10);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![3e10, 1e10]);
    }

    #[test]
    fn test_small_numbers() {
        let input = TwoNumberInput { a: 3e-10, b: 1e-10 };
        let result = subtract_numbers(input).unwrap();
        assert!((result.result - 2e-10).abs() < 1e-20);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![3e-10, 1e-10]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput {
            a: f64::NAN,
            b: 3.0,
        };
        let result = subtract_numbers(input);
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
        let result = subtract_numbers(input);
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
        let result = subtract_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_decimal_precision() {
        let input = TwoNumberInput { a: 0.3, b: 0.1 };
        let result = subtract_numbers(input).unwrap();
        assert!((result.result - 0.2).abs() < 1e-15);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![0.3, 0.1]);
    }

    #[test]
    fn test_negative_result() {
        let input = TwoNumberInput { a: 3.0, b: 5.0 };
        let result = subtract_numbers(input).unwrap();
        assert_eq!(result.result, -2.0);
        assert_eq!(result.operation, "subtraction");
        assert_eq!(result.inputs, vec![3.0, 5.0]);
    }
}
