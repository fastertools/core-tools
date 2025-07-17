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

pub fn modulo_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() ||
       input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    // Check for modulo by zero
    if input.b == 0.0 {
        return Err("Modulo by zero is not allowed".to_string());
    }
    
    // Rust's % operator follows the sign of the dividend (like C)
    // This is truncated modulo, not Euclidean modulo
    let result = input.a % input.b;
    
    Ok(ArithmeticResult {
        result,
        operation: "modulo".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_numbers() {
        let input = TwoNumberInput { a: 10.0, b: 3.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![10.0, 3.0]);
    }

    #[test]
    fn test_negative_dividend() {
        let input = TwoNumberInput { a: -10.0, b: 3.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, -1.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![-10.0, 3.0]);
    }

    #[test]
    fn test_negative_divisor() {
        let input = TwoNumberInput { a: 10.0, b: -3.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![10.0, -3.0]);
    }

    #[test]
    fn test_both_negative() {
        let input = TwoNumberInput { a: -10.0, b: -3.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, -1.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![-10.0, -3.0]);
    }

    #[test]
    fn test_zero_dividend() {
        let input = TwoNumberInput { a: 0.0, b: 5.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![0.0, 5.0]);
    }

    #[test]
    fn test_modulo_by_zero() {
        let input = TwoNumberInput { a: 10.0, b: 0.0 };
        let result = modulo_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Modulo by zero is not allowed");
    }

    #[test]
    fn test_exact_division() {
        let input = TwoNumberInput { a: 12.0, b: 4.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![12.0, 4.0]);
    }

    #[test]
    fn test_decimal_numbers() {
        let input = TwoNumberInput { a: 10.5, b: 3.0 };
        let result = modulo_numbers(input).unwrap();
        assert!((result.result - 1.5).abs() < 1e-15);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![10.5, 3.0]);
    }

    #[test]
    fn test_large_numbers() {
        let input = TwoNumberInput { a: 1000000.0, b: 7.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 5.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![1000000.0, 7.0]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput { a: f64::NAN, b: 3.0 };
        let result = modulo_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = TwoNumberInput { a: 5.0, b: f64::INFINITY };
        let result = modulo_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_modulo_by_one() {
        let input = TwoNumberInput { a: 42.0, b: 1.0 };
        let result = modulo_numbers(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![42.0, 1.0]);
    }

    #[test]
    fn test_fractional_modulo() {
        let input = TwoNumberInput { a: 5.5, b: 2.5 };
        let result = modulo_numbers(input).unwrap();
        assert!((result.result - 0.5).abs() < 1e-15);
        assert_eq!(result.operation, "modulo");
        assert_eq!(result.inputs, vec![5.5, 2.5]);
    }
}