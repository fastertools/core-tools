use serde::{Deserialize, Serialize};

const EPSILON: f64 = 1e-10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line3D {
    pub point: Vector3D,
    pub direction: Vector3D,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLineInput {
    pub point: Vector3D,
    pub line: Line3D,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PointLineDistanceResult {
    pub distance: f64,
    pub closest_point_on_line: Vector3D,
    pub parameter_on_line: f64,
    pub perpendicular_vector: Vector3D,
    pub point_is_on_line: bool,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude_squared() < EPSILON * EPSILON
    }
}

impl Line3D {
    pub fn new(point: Vector3D, direction: Vector3D) -> Self {
        Line3D { point, direction }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
    }
}

pub fn point_line_distance_logic(input: PointLineInput) -> Result<PointLineDistanceResult, String> {
    let point = &input.point;
    let line = &input.line;

    // Validate inputs for NaN and infinite values
    if point.x.is_nan() || point.x.is_infinite() ||
       point.y.is_nan() || point.y.is_infinite() ||
       point.z.is_nan() || point.z.is_infinite() {
        return Err("Point coordinates must be finite".to_string());
    }

    if line.point.x.is_nan() || line.point.x.is_infinite() ||
       line.point.y.is_nan() || line.point.y.is_infinite() ||
       line.point.z.is_nan() || line.point.z.is_infinite() {
        return Err("Line point coordinates must be finite".to_string());
    }

    if line.direction.x.is_nan() || line.direction.x.is_infinite() ||
       line.direction.y.is_nan() || line.direction.y.is_infinite() ||
       line.direction.z.is_nan() || line.direction.z.is_infinite() {
        return Err("Line direction coordinates must be finite".to_string());
    }

    // Validate line direction
    if line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }

    // Vector from line point to query point
    let to_point = point.subtract(&line.point);
    
    // Project this vector onto the line direction to find the closest point
    let line_dir_mag_sq = line.direction.magnitude_squared();
    let t = to_point.dot(&line.direction) / line_dir_mag_sq;
    
    // Find closest point on line
    let closest_point_on_line = line.point_at_parameter(t);
    
    // Calculate distance and perpendicular vector
    let perpendicular_vector = point.subtract(&closest_point_on_line);
    let distance = perpendicular_vector.magnitude();
    
    // Check if point is on line
    let point_is_on_line = distance < EPSILON;

    Ok(PointLineDistanceResult {
        distance,
        closest_point_on_line,
        parameter_on_line: t,
        perpendicular_vector,
        point_is_on_line,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_on_line() {
        let input = PointLineInput {
            point: Vector3D::new(2.0, 3.0, 4.0),
            line: Line3D::new(
                Vector3D::new(0.0, 1.0, 2.0),
                Vector3D::new(1.0, 1.0, 1.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!(result.point_is_on_line);
        assert!((result.distance - 0.0).abs() < EPSILON);
        assert!((result.parameter_on_line - 2.0).abs() < EPSILON);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - 2.0).abs() < EPSILON);
        assert!((closest.y - 3.0).abs() < EPSILON);
        assert!((closest.z - 4.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_not_on_line() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 0.0, 0.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 1.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!(!result.point_is_on_line);
        assert!((result.distance - 1.0).abs() < EPSILON);
        assert!((result.parameter_on_line - 0.0).abs() < EPSILON);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - 0.0).abs() < EPSILON);
        assert!((closest.y - 0.0).abs() < EPSILON);
        assert!((closest.z - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_perpendicular_distance() {
        let input = PointLineInput {
            point: Vector3D::new(0.0, 1.0, 0.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!(!result.point_is_on_line);
        assert!((result.distance - 1.0).abs() < EPSILON);
        assert!((result.parameter_on_line - 0.0).abs() < EPSILON);
        
        let perpendicular = &result.perpendicular_vector;
        assert!((perpendicular.x - 0.0).abs() < EPSILON);
        assert!((perpendicular.y - 1.0).abs() < EPSILON);
        assert!((perpendicular.z - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_diagonal_line() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        let expected_t = (1.0 + 2.0 + 3.0) / 3.0; // dot product / magnitude squared
        assert!((result.parameter_on_line - expected_t).abs() < EPSILON);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - expected_t).abs() < EPSILON);
        assert!((closest.y - expected_t).abs() < EPSILON);
        assert!((closest.z - expected_t).abs() < EPSILON);
    }

    #[test]
    fn test_negative_parameter() {
        let input = PointLineInput {
            point: Vector3D::new(-1.0, 0.0, 0.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!((result.parameter_on_line - (-1.0)).abs() < EPSILON);
        assert!(result.point_is_on_line);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - (-1.0)).abs() < EPSILON);
        assert!((closest.y - 0.0).abs() < EPSILON);
        assert!((closest.z - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_zero_direction_error() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line direction vector cannot be zero");
    }

    #[test]
    fn test_nan_point() {
        let input = PointLineInput {
            point: Vector3D::new(f64::NAN, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Point coordinates must be finite");
    }

    #[test]
    fn test_infinite_line_point() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(f64::INFINITY, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line point coordinates must be finite");
    }

    #[test]
    fn test_infinite_line_direction() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(f64::INFINITY, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line direction coordinates must be finite");
    }

    #[test]
    fn test_very_small_direction() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 2.0, 3.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1e-15, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line direction vector cannot be zero");
    }

    #[test]
    fn test_distance_precision() {
        let input = PointLineInput {
            point: Vector3D::new(0.0, 3.0, 4.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        let expected_distance = (3.0_f64 * 3.0 + 4.0 * 4.0).sqrt(); // 5.0
        assert!((result.distance - expected_distance).abs() < EPSILON);
        assert!(!result.point_is_on_line);
    }

    #[test]
    fn test_large_coordinates() {
        let input = PointLineInput {
            point: Vector3D::new(1000.0, 2000.0, 3000.0),
            line: Line3D::new(
                Vector3D::new(500.0, 1000.0, 1500.0),
                Vector3D::new(1.0, 1.0, 1.0),
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!(result.distance >= 0.0);
        assert!((result.parameter_on_line - 1000.0).abs() < 1e-10);
    }

    #[test]
    fn test_unit_direction_vector() {
        let input = PointLineInput {
            point: Vector3D::new(1.0, 1.0, 0.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0), // Unit vector
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!((result.parameter_on_line - 1.0).abs() < EPSILON);
        assert!((result.distance - 1.0).abs() < EPSILON);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - 1.0).abs() < EPSILON);
        assert!((closest.y - 0.0).abs() < EPSILON);
        assert!((closest.z - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_scaled_direction_vector() {
        let input = PointLineInput {
            point: Vector3D::new(2.0, 2.0, 0.0),
            line: Line3D::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(2.0, 0.0, 0.0), // Scaled vector
            ),
        };

        let result = point_line_distance_logic(input).unwrap();
        assert!((result.parameter_on_line - 1.0).abs() < EPSILON); // t = 4/4 = 1
        assert!((result.distance - 2.0).abs() < EPSILON);
        
        let closest = &result.closest_point_on_line;
        assert!((closest.x - 2.0).abs() < EPSILON);
        assert!((closest.y - 0.0).abs() < EPSILON);
        assert!((closest.z - 0.0).abs() < EPSILON);
    }
}