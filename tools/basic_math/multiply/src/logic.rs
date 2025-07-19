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

pub fn multiply_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() || input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    let result = input.a * input.b;

    Ok(ArithmeticResult {
        result,
        operation: "multiplication".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let input = TwoNumberInput { a: 6.0, b: 7.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 42.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![6.0, 7.0]);
    }

    #[test]
    fn test_negative_numbers() {
        let input = TwoNumberInput { a: -4.0, b: -5.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 20.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![-4.0, -5.0]);
    }

    #[test]
    fn test_mixed_signs() {
        let input = TwoNumberInput { a: 8.0, b: -3.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, -24.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![8.0, -3.0]);
    }

    #[test]
    fn test_zero_multiplication() {
        let input = TwoNumberInput { a: 42.0, b: 0.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![42.0, 0.0]);
    }

    #[test]
    fn test_zero_by_zero() {
        let input = TwoNumberInput { a: 0.0, b: 0.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![0.0, 0.0]);
    }

    #[test]
    fn test_one_multiplication() {
        let input = TwoNumberInput { a: 42.0, b: 1.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 42.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![42.0, 1.0]);
    }

    #[test]
    fn test_decimal_numbers() {
        let input = TwoNumberInput { a: 2.5, b: 4.0 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 10.0);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![2.5, 4.0]);
    }

    #[test]
    fn test_large_numbers() {
        let input = TwoNumberInput { a: 1e6, b: 2e6 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 2e12);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![1e6, 2e6]);
    }

    #[test]
    fn test_small_numbers() {
        let input = TwoNumberInput { a: 1e-6, b: 2e-6 };
        let result = multiply_numbers(input).unwrap();
        assert_eq!(result.result, 2e-12);
        assert_eq!(result.operation, "multiplication");
        assert_eq!(result.inputs, vec![1e-6, 2e-6]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput {
            a: f64::NAN,
            b: 3.0,
        };
        let result = multiply_numbers(input);
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
        let result = multiply_numbers(input);
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
        let result = multiply_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }
}
