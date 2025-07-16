use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleNumberInput {
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArithmeticResult {
    pub result: f64,
    pub operation: String,
    pub inputs: Vec<f64>,
}

pub fn square_number(input: SingleNumberInput) -> Result<ArithmeticResult, String> {
    // Validate input - check for invalid values
    if input.value.is_nan() || input.value.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    let result = input.value * input.value;
    
    Ok(ArithmeticResult {
        result,
        operation: "square".to_string(),
        inputs: vec![input.value],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let input = SingleNumberInput { value: 5.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 25.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![5.0]);
    }

    #[test]
    fn test_negative_integer() {
        let input = SingleNumberInput { value: -4.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 16.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![-4.0]);
    }

    #[test]
    fn test_zero() {
        let input = SingleNumberInput { value: 0.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 0.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![0.0]);
    }

    #[test]
    fn test_one() {
        let input = SingleNumberInput { value: 1.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![1.0]);
    }

    #[test]
    fn test_negative_one() {
        let input = SingleNumberInput { value: -1.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 1.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![-1.0]);
    }

    #[test]
    fn test_decimal() {
        let input = SingleNumberInput { value: 2.5 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 6.25);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![2.5]);
    }

    #[test]
    fn test_negative_decimal() {
        let input = SingleNumberInput { value: -1.5 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 2.25);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![-1.5]);
    }

    #[test]
    fn test_large_number() {
        let input = SingleNumberInput { value: 100.0 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 10000.0);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![100.0]);
    }

    #[test]
    fn test_small_number() {
        let input = SingleNumberInput { value: 0.1 };
        let result = square_number(input).unwrap();
        assert!((result.result - 0.01).abs() < 1e-15);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![0.1]);
    }

    #[test]
    fn test_very_small_number() {
        let input = SingleNumberInput { value: 1e-5 };
        let result = square_number(input).unwrap();
        assert!((result.result - 1e-10).abs() < 1e-20);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![1e-5]);
    }

    #[test]
    fn test_fraction() {
        let input = SingleNumberInput { value: 0.5 };
        let result = square_number(input).unwrap();
        assert_eq!(result.result, 0.25);
        assert_eq!(result.operation, "square");
        assert_eq!(result.inputs, vec![0.5]);
    }

    #[test]
    fn test_nan_input_error() {
        let input = SingleNumberInput { value: f64::NAN };
        let result = square_number(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = SingleNumberInput { value: f64::INFINITY };
        let result = square_number(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_negative_infinite_input_error() {
        let input = SingleNumberInput { value: f64::NEG_INFINITY };
        let result = square_number(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }
}