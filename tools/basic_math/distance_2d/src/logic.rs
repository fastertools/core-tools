use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoPointInput {
    pub point1: Point2D,
    pub point2: Point2D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceResult {
    pub distance: f64,
    pub point1: Point2D,
    pub point2: Point2D,
    pub delta_x: f64,
    pub delta_y: f64,
    pub calculation_steps: Vec<String>,
    pub note: String,
}

// Pure business logic - format distance result with provided data
pub fn format_distance_result(
    input: TwoPointInput, 
    distance: f64, 
    calculation_steps: Vec<String>
) -> DistanceResult {
    let delta_x = input.point2.x - input.point1.x;
    let delta_y = input.point2.y - input.point1.y;
    
    DistanceResult {
        distance,
        point1: input.point1,
        point2: input.point2,
        delta_x,
        delta_y,
        calculation_steps,
        note: "Distance calculated via modular design".to_string(),
    }
}

// Helper function for input validation
pub fn validate_input(input: &TwoPointInput) -> Result<(), String> {
    if input.point1.x.is_nan() || input.point1.x.is_infinite() ||
       input.point1.y.is_nan() || input.point1.y.is_infinite() ||
       input.point2.x.is_nan() || input.point2.x.is_infinite() ||
       input.point2.y.is_nan() || input.point2.y.is_infinite() {
        return Err("Input points contain invalid values (NaN or Infinite)".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_distance() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 3.0, y: 4.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 5.0, steps);
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 3.0);
        assert_eq!(result.delta_y, 4.0);
        assert_eq!(result.point1.x, 0.0);
        assert_eq!(result.point2.x, 3.0);
    }

    #[test]
    fn test_same_point() {
        let input = TwoPointInput {
            point1: Point2D { x: 5.0, y: 7.0 },
            point2: Point2D { x: 5.0, y: 7.0 },
        };
        let steps = vec!["Same point test".to_string()];
        let result = format_distance_result(input, 0.0, steps);
        assert_eq!(result.distance, 0.0);
        assert_eq!(result.delta_x, 0.0);
        assert_eq!(result.delta_y, 0.0);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = TwoPointInput {
            point1: Point2D { x: -1.0, y: -1.0 },
            point2: Point2D { x: 2.0, y: 3.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 5.0, steps);
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 3.0);
        assert_eq!(result.delta_y, 4.0);
    }

    #[test]
    fn test_horizontal_distance() {
        let input = TwoPointInput {
            point1: Point2D { x: 1.0, y: 5.0 },
            point2: Point2D { x: 6.0, y: 5.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 5.0, steps);
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 5.0);
        assert_eq!(result.delta_y, 0.0);
    }

    #[test]
    fn test_vertical_distance() {
        let input = TwoPointInput {
            point1: Point2D { x: 3.0, y: 2.0 },
            point2: Point2D { x: 3.0, y: 8.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 6.0, steps);
        assert_eq!(result.distance, 6.0);
        assert_eq!(result.delta_x, 0.0);
        assert_eq!(result.delta_y, 6.0);
    }

    #[test]
    fn test_decimal_coordinates() {
        let input = TwoPointInput {
            point1: Point2D { x: 1.5, y: 2.5 },
            point2: Point2D { x: 4.5, y: 6.5 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 5.0, steps);
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 3.0);
        assert_eq!(result.delta_y, 4.0);
    }

    #[test]
    fn test_large_coordinates() {
        let input = TwoPointInput {
            point1: Point2D { x: 1000.0, y: 2000.0 },
            point2: Point2D { x: 1003.0, y: 2004.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 5.0, steps);
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 3.0);
        assert_eq!(result.delta_y, 4.0);
    }

    #[test]
    fn test_unit_distance() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 1.0, y: 0.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let result = format_distance_result(input, 1.0, steps);
        assert_eq!(result.distance, 1.0);
        assert_eq!(result.delta_x, 1.0);
        assert_eq!(result.delta_y, 0.0);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoPointInput {
            point1: Point2D { x: f64::NAN, y: 2.0 },
            point2: Point2D { x: 5.0, y: 6.0 },
        };
        let result = validate_input(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input points contain invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = TwoPointInput {
            point1: Point2D { x: 1.0, y: 2.0 },
            point2: Point2D { x: f64::INFINITY, y: 6.0 },
        };
        let result = validate_input(&input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input points contain invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_calculation_steps() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 3.0, y: 4.0 },
        };
        let steps = vec![
            "Step 1: Calculate differences".to_string(),
            "Δx = 3.0 - 0.0 = 3.0".to_string(),
            "Δy = 4.0 - 0.0 = 4.0".to_string(),
            "Step 2: Apply Pythagorean theorem".to_string()
        ];
        let result = format_distance_result(input, 5.0, steps);
        assert!(result.calculation_steps.len() >= 4);
        assert!(result.calculation_steps[0].contains("Calculate differences"));
        assert!(result.calculation_steps.iter().any(|step| step.contains("Pythagorean")));
    }

    #[test]
    fn test_sqrt_two_diagonal() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 1.0, y: 1.0 },
        };
        let steps = vec!["Test calculation".to_string()];
        let distance = 2.0_f64.sqrt();
        let result = format_distance_result(input, distance, steps);
        assert!((result.distance - 2.0_f64.sqrt()).abs() < 1e-15);
        assert_eq!(result.delta_x, 1.0);
        assert_eq!(result.delta_y, 1.0);
    }
}