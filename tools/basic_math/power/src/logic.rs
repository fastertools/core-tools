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

pub fn power_numbers(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() || input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    // Special cases for power operations
    // 0^0 is mathematically undefined, but most systems return 1
    if input.a == 0.0 && input.b == 0.0 {
        return Err("0^0 is mathematically undefined".to_string());
    }

    // 0 raised to negative power is undefined (division by zero)
    if input.a == 0.0 && input.b < 0.0 {
        return Err("0 raised to negative power is undefined".to_string());
    }

    // Negative number raised to fractional power may result in complex numbers
    if input.a < 0.0 && input.b.fract() != 0.0 {
        return Err("Negative base with fractional exponent results in complex number".to_string());
    }

    let result = input.a.powf(input.b);

    // Check if result is valid
    if result.is_nan() || result.is_infinite() {
        return Err("Result is too large or undefined".to_string());
    }

    Ok(ArithmeticResult {
        result,
        operation: "exponentiation".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integers() {
        let input = TwoNumberInput { a: 2.0, b: 3.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 8.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![2.0, 3.0]);
    }

    #[test]
    fn test_square() {
        let input = TwoNumberInput { a: 5.0, b: 2.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 25.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![5.0, 2.0]);
    }

    #[test]
    fn test_cube() {
        let input = TwoNumberInput { a: 3.0, b: 3.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 27.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![3.0, 3.0]);
    }

    #[test]
    fn test_zero_exponent() {
        let input = TwoNumberInput { a: 5.0, b: 0.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![5.0, 0.0]);
    }

    #[test]
    fn test_one_exponent() {
        let input = TwoNumberInput { a: 42.0, b: 1.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 42.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![42.0, 1.0]);
    }

    #[test]
    fn test_negative_exponent() {
        let input = TwoNumberInput { a: 2.0, b: -3.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 0.125);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![2.0, -3.0]);
    }

    #[test]
    fn test_fractional_exponent() {
        let input = TwoNumberInput { a: 4.0, b: 0.5 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 2.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![4.0, 0.5]);
    }

    #[test]
    fn test_zero_to_zero() {
        let input = TwoNumberInput { a: 0.0, b: 0.0 };
        let result = power_numbers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "0^0 is mathematically undefined");
    }

    #[test]
    fn test_zero_to_negative() {
        let input = TwoNumberInput { a: 0.0, b: -2.0 };
        let result = power_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "0 raised to negative power is undefined"
        );
    }

    #[test]
    fn test_negative_base_fractional_exponent() {
        let input = TwoNumberInput { a: -4.0, b: 0.5 };
        let result = power_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Negative base with fractional exponent results in complex number"
        );
    }

    #[test]
    fn test_negative_base_integer_exponent() {
        let input = TwoNumberInput { a: -2.0, b: 3.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, -8.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![-2.0, 3.0]);
    }

    #[test]
    fn test_negative_base_even_exponent() {
        let input = TwoNumberInput { a: -2.0, b: 4.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 16.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![-2.0, 4.0]);
    }

    #[test]
    fn test_large_result() {
        let input = TwoNumberInput { a: 10.0, b: 100.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 1e100);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![10.0, 100.0]);
    }

    #[test]
    fn test_small_result() {
        let input = TwoNumberInput { a: 10.0, b: -100.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 1e-100);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![10.0, -100.0]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoNumberInput {
            a: f64::NAN,
            b: 3.0,
        };
        let result = power_numbers(input);
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
        let result = power_numbers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_one_base() {
        let input = TwoNumberInput { a: 1.0, b: 999.0 };
        let result = power_numbers(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "exponentiation");
        assert_eq!(result.inputs, vec![1.0, 999.0]);
    }
}
