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

pub fn remainder_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() || input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    // Check for remainder by zero
    if input.b == 0.0 {
        return Err("Remainder by zero is not allowed".to_string());
    }

    // Rust's % operator is remainder (truncated division), not mathematical modulus
    // Result follows the sign of the dividend (left operand)
    // For example: -21 % 4 = -1 (remainder), not 3 (modulus)
    let result = input.a % input.b;

    Ok(ArithmeticResult {
        result,
        operation: "remainder".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let input = TwoNumberInput { a: 10.0, b: 3.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![10.0, 3.0]);
    }

    #[test]
    fn test_negative_dividend() {
        let input = TwoNumberInput { a: -10.0, b: 3.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, -1.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![-10.0, 3.0]);
    }

    #[test]
    fn test_negative_divisor() {
        let input = TwoNumberInput { a: 10.0, b: -3.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![10.0, -3.0]);
    }

    #[test]
    fn test_both_negative() {
        let input = TwoNumberInput { a: -10.0, b: -3.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, -1.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![-10.0, -3.0]);
    }

    #[test]
    fn test_remainder_vs_modulus_behavior() {
        // This test documents that we implement remainder (%) not mathematical modulus
        // For -21 % 4: remainder = -1, but mathematical modulus would be 3
        let input = TwoNumberInput { a: -21.0, b: 4.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, -1.0); // remainder behavior (follows dividend sign)
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![-21.0, 4.0]);
        // Note: Mathematical modulus would return 3, but Rust's % returns -1
    }

    #[test]
    fn test_zero_dividend() {
        let input = TwoNumberInput { a: 0.0, b: 5.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![0.0, 5.0]);
    }

    #[test]
    fn test_modulo_by_zero() {
        let input = TwoNumberInput { a: 10.0, b: 0.0 };
        let result = remainder_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Remainder by zero is not allowed");
    }

    #[test]
    fn test_exact_division() {
        let input = TwoNumberInput { a: 12.0, b: 4.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![12.0, 4.0]);
    }

    #[test]
    fn test_decimal_numbers() {
        let input = TwoNumberInput { a: 10.5, b: 3.0 };
        let result = remainder_numbers(input).unwrap();
        assert!((result.result - 1.5).abs() < 1e-15);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![10.5, 3.0]);
    }

    #[test]
    fn test_large_numbers() {
        let input = TwoNumberInput {
            a: 9876543210.0,
            b: 12345.0,
        };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 30.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![9876543210.0, 12345.0]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput {
            a: f64::NAN,
            b: 3.0,
        };
        let result = remainder_numbers(input);
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
        let result = remainder_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_modulo_by_one() {
        let input = TwoNumberInput { a: 42.0, b: 1.0 };
        let result = remainder_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![42.0, 1.0]);
    }

    #[test]
    fn test_fractional_modulo() {
        let input = TwoNumberInput { a: 5.5, b: 2.5 };
        let result = remainder_numbers(input).unwrap();
        assert!((result.result - 0.5).abs() < 1e-15);
        assert_eq!(result.operation, "remainder");
        assert_eq!(result.inputs, vec![5.5, 2.5]);
    }
}
