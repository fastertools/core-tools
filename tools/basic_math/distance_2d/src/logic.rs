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

pub fn calculate_distance_2d(input: TwoPointInput) -> Result<DistanceResult, String> {
    // Validate input - check for invalid values
    if input.point1.x.is_nan()
        || input.point1.x.is_infinite()
        || input.point1.y.is_nan()
        || input.point1.y.is_infinite()
        || input.point2.x.is_nan()
        || input.point2.x.is_infinite()
        || input.point2.y.is_nan()
        || input.point2.y.is_infinite()
    {
        return Err("Input points contain invalid values (NaN or Infinite)".to_string());
    }

    let mut calculation_steps = Vec::new();

    // Step 1: Calculate differences
    let delta_x = input.point2.x - input.point1.x;
    let delta_y = input.point2.y - input.point1.y;
    calculation_steps.push("Step 1: Calculate differences".to_string());
    calculation_steps.push(format!(
        "Δx = {} - {} = {}",
        input.point2.x, input.point1.x, delta_x
    ));
    calculation_steps.push(format!(
        "Δy = {} - {} = {}",
        input.point2.y, input.point1.y, delta_y
    ));

    // Step 2: Apply Pythagorean theorem directly
    calculation_steps.push("Step 2: Apply Pythagorean theorem (d = √(Δx² + Δy²))".to_string());

    let distance_squared = delta_x * delta_x + delta_y * delta_y;
    let distance = distance_squared.sqrt();

    calculation_steps.push(format!(
        "d² = {}² + {}² = {}",
        delta_x, delta_y, distance_squared
    ));
    calculation_steps.push(format!("d = √{} = {}", distance_squared, distance));

    Ok(DistanceResult {
        distance,
        point1: input.point1,
        point2: input.point2,
        delta_x,
        delta_y,
        calculation_steps,
        note: "Pure function implementation for unit testing".to_string(),
    })
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
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
        assert_eq!(result.distance, 5.0);
        assert_eq!(result.delta_x, 3.0);
        assert_eq!(result.delta_y, 4.0);
    }

    #[test]
    fn test_large_coordinates() {
        let input = TwoPointInput {
            point1: Point2D {
                x: 1000.0,
                y: 2000.0,
            },
            point2: Point2D {
                x: 1003.0,
                y: 2004.0,
            },
        };
        let result = calculate_distance_2d(input).unwrap();
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
        let result = calculate_distance_2d(input).unwrap();
        assert_eq!(result.distance, 1.0);
        assert_eq!(result.delta_x, 1.0);
        assert_eq!(result.delta_y, 0.0);
    }

    #[test]
    fn test_nan_input_error() {
        let input = TwoPointInput {
            point1: Point2D {
                x: f64::NAN,
                y: 2.0,
            },
            point2: Point2D { x: 5.0, y: 6.0 },
        };
        let result = calculate_distance_2d(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input points contain invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_infinite_input_error() {
        let input = TwoPointInput {
            point1: Point2D { x: 1.0, y: 2.0 },
            point2: Point2D {
                x: f64::INFINITY,
                y: 6.0,
            },
        };
        let result = calculate_distance_2d(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input points contain invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_calculation_steps() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 3.0, y: 4.0 },
        };
        let result = calculate_distance_2d(input).unwrap();
        assert!(result.calculation_steps.len() >= 4);
        assert!(result.calculation_steps[0].contains("Calculate differences"));
        assert!(
            result
                .calculation_steps
                .iter()
                .any(|step| step.contains("Pythagorean theorem"))
        );
    }

    #[test]
    fn test_sqrt_two_diagonal() {
        let input = TwoPointInput {
            point1: Point2D { x: 0.0, y: 0.0 },
            point2: Point2D { x: 1.0, y: 1.0 },
        };
        let result = calculate_distance_2d(input).unwrap();
        assert!((result.distance - 2.0_f64.sqrt()).abs() < 1e-15);
        assert_eq!(result.delta_x, 1.0);
        assert_eq!(result.delta_y, 1.0);
    }
}
