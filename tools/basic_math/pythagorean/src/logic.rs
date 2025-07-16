use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythagoreanInput {
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythagoreanResult {
    pub hypotenuse: f64,
    pub leg_a: f64,
    pub leg_b: f64,
    pub calculation_steps: Vec<String>,
    pub tool_calls: Vec<String>,
}

pub fn calculate_pythagorean(input: PythagoreanInput) -> Result<PythagoreanResult, String> {
    // Validate input - check for invalid values
    if input.a.is_nan() || input.a.is_infinite() ||
       input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    // Check for negative values (triangle legs must be positive)
    if input.a < 0.0 || input.b < 0.0 {
        return Err("Triangle legs must be non-negative".to_string());
    }
    
    let mut calculation_steps = Vec::new();
    let mut tool_calls = Vec::new();
    
    // Step 1: Square first leg (a²)
    calculation_steps.push(format!("Step 1: Square first leg: {}² = ?", input.a));
    tool_calls.push(format!("Pure function: square({}) via a²", input.a));
    
    let a_squared = input.a * input.a;
    calculation_steps.push(format!("Result: {}² = {}", input.a, a_squared));
    
    // Step 2: Square second leg (b²)
    calculation_steps.push(format!("Step 2: Square second leg: {}² = ?", input.b));
    tool_calls.push(format!("Pure function: square({}) via b²", input.b));
    
    let b_squared = input.b * input.b;
    calculation_steps.push(format!("Result: {}² = {}", input.b, b_squared));
    
    // Step 3: Add the squares (a² + b²)
    calculation_steps.push(format!("Step 3: Add squares: {} + {} = ?", a_squared, b_squared));
    tool_calls.push(format!("Pure function: add({}, {}) via a² + b²", a_squared, b_squared));
    
    let sum_of_squares = a_squared + b_squared;
    calculation_steps.push(format!("Result: {} + {} = {}", a_squared, b_squared, sum_of_squares));
    
    // Step 4: Take square root (sqrt(a² + b²))
    calculation_steps.push(format!("Step 4: Take square root: sqrt({}) = ?", sum_of_squares));
    tool_calls.push(format!("Pure function: sqrt({}) via f64::sqrt()", sum_of_squares));
    
    let hypotenuse = sum_of_squares.sqrt();
    calculation_steps.push(format!("Result: sqrt({}) = {}", sum_of_squares, hypotenuse));
    
    Ok(PythagoreanResult {
        hypotenuse,
        leg_a: input.a,
        leg_b: input.b,
        calculation_steps,
        tool_calls,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classic_3_4_5_triangle() {
        let input = PythagoreanInput { a: 3.0, b: 4.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 5.0);
        assert_eq!(result.leg_a, 3.0);
        assert_eq!(result.leg_b, 4.0);
        assert!(result.calculation_steps.len() >= 7);
        assert!(result.tool_calls.len() >= 4);
    }

    #[test]
    fn test_5_12_13_triangle() {
        let input = PythagoreanInput { a: 5.0, b: 12.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 13.0);
        assert_eq!(result.leg_a, 5.0);
        assert_eq!(result.leg_b, 12.0);
    }

    #[test]
    fn test_unit_triangle() {
        let input = PythagoreanInput { a: 1.0, b: 1.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert!((result.hypotenuse - 2.0_f64.sqrt()).abs() < 1e-15);
        assert_eq!(result.leg_a, 1.0);
        assert_eq!(result.leg_b, 1.0);
    }

    #[test]
    fn test_zero_leg() {
        let input = PythagoreanInput { a: 0.0, b: 5.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 5.0);
        assert_eq!(result.leg_a, 0.0);
        assert_eq!(result.leg_b, 5.0);
    }

    #[test]
    fn test_both_legs_zero() {
        let input = PythagoreanInput { a: 0.0, b: 0.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 0.0);
        assert_eq!(result.leg_a, 0.0);
        assert_eq!(result.leg_b, 0.0);
    }

    #[test]
    fn test_decimal_values() {
        let input = PythagoreanInput { a: 1.5, b: 2.0 };
        let result = calculate_pythagorean(input).unwrap();
        let expected = (1.5_f64 * 1.5 + 2.0 * 2.0).sqrt();
        assert!((result.hypotenuse - expected).abs() < 1e-15);
        assert_eq!(result.leg_a, 1.5);
        assert_eq!(result.leg_b, 2.0);
    }

    #[test]
    fn test_large_values() {
        let input = PythagoreanInput { a: 300.0, b: 400.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 500.0);
        assert_eq!(result.leg_a, 300.0);
        assert_eq!(result.leg_b, 400.0);
    }

    #[test]
    fn test_small_values() {
        let input = PythagoreanInput { a: 0.003, b: 0.004 };
        let result = calculate_pythagorean(input).unwrap();
        assert!((result.hypotenuse - 0.005).abs() < 1e-15);
        assert_eq!(result.leg_a, 0.003);
        assert_eq!(result.leg_b, 0.004);
    }

    #[test]
    fn test_negative_leg_error() {
        let input = PythagoreanInput { a: -3.0, b: 4.0 };
        let result = calculate_pythagorean(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Triangle legs must be non-negative");
    }

    #[test]
    fn test_both_negative_legs_error() {
        let input = PythagoreanInput { a: -3.0, b: -4.0 };
        let result = calculate_pythagorean(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Triangle legs must be non-negative");
    }

    #[test]
    fn test_nan_input_error() {
        let input = PythagoreanInput { a: f64::NAN, b: 4.0 };
        let result = calculate_pythagorean(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = PythagoreanInput { a: 3.0, b: f64::INFINITY };
        let result = calculate_pythagorean(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_calculation_steps_content() {
        let input = PythagoreanInput { a: 3.0, b: 4.0 };
        let result = calculate_pythagorean(input).unwrap();
        
        assert!(result.calculation_steps.iter().any(|step| step.contains("Square first leg")));
        assert!(result.calculation_steps.iter().any(|step| step.contains("Square second leg")));
        assert!(result.calculation_steps.iter().any(|step| step.contains("Add squares")));
        assert!(result.calculation_steps.iter().any(|step| step.contains("Take square root")));
    }

    #[test]
    fn test_tool_calls_content() {
        let input = PythagoreanInput { a: 3.0, b: 4.0 };
        let result = calculate_pythagorean(input).unwrap();
        
        assert!(result.tool_calls.iter().any(|call| call.contains("square")));
        assert!(result.tool_calls.iter().any(|call| call.contains("add")));
        assert!(result.tool_calls.iter().any(|call| call.contains("sqrt")));
    }

    #[test]
    fn test_isosceles_right_triangle() {
        // 45-45-90 triangle with legs of 1
        let input = PythagoreanInput { a: 1.0, b: 1.0 };
        let result = calculate_pythagorean(input).unwrap();
        let expected = 2.0_f64.sqrt();
        assert!((result.hypotenuse - expected).abs() < 1e-15);
    }

    #[test]
    fn test_8_15_17_triangle() {
        let input = PythagoreanInput { a: 8.0, b: 15.0 };
        let result = calculate_pythagorean(input).unwrap();
        assert_eq!(result.hypotenuse, 17.0);
        assert_eq!(result.leg_a, 8.0);
        assert_eq!(result.leg_b, 15.0);
    }
}