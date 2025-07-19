use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Clone)]
pub struct TwoVectorInput {
    pub vector1: Vector3D,
    pub vector2: Vector3D,
}

#[derive(Serialize, Clone, Debug)]
pub struct VectorAngleResult {
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub cos_angle: f64,
    pub vector1_magnitude: f64,
    pub vector2_magnitude: f64,
    pub is_perpendicular: bool,
    pub is_parallel: bool,
}

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_with(&self, other: &Vector3D) -> Result<f64, String> {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 == 0.0 || mag2 == 0.0 {
            return Err("Cannot compute angle with zero vector".to_string());
        }

        let cos_angle = self.dot(other) / (mag1 * mag2);

        // Clamp to [-1, 1] to handle numerical precision issues
        let cos_angle = cos_angle.max(-1.0).min(1.0);

        Ok(cos_angle.acos())
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn vector_angle_logic(input: TwoVectorInput) -> Result<VectorAngleResult, String> {
    let v1 = &input.vector1;
    let v2 = &input.vector2;

    // Input validation
    if !v1.is_valid() || !v2.is_valid() {
        return Err("Invalid vector components: must be finite numbers".to_string());
    }

    if v1.is_zero() || v2.is_zero() {
        return Err("Cannot compute angle with zero vector".to_string());
    }

    let mag1 = v1.magnitude();
    let mag2 = v2.magnitude();
    let angle_radians = v1.angle_with(v2)?;
    let angle_degrees = angle_radians.to_degrees();
    let cos_angle = v1.dot(v2) / (mag1 * mag2);

    // Check for special relationships
    const EPSILON: f64 = 1e-10;
    let is_perpendicular = (angle_radians - std::f64::consts::PI / 2.0).abs() < EPSILON;
    let is_parallel =
        angle_radians < EPSILON || (angle_radians - std::f64::consts::PI).abs() < EPSILON;

    Ok(VectorAngleResult {
        angle_radians,
        angle_degrees,
        cos_angle,
        vector1_magnitude: mag1,
        vector2_magnitude: mag2,
        is_perpendicular,
        is_parallel,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_vector(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    #[test]
    fn test_perpendicular_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1.0, 0.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
        assert!((result.angle_degrees - 90.0).abs() < 1e-10);
        assert!(result.cos_angle.abs() < 1e-10);
        assert!(result.is_perpendicular);
        assert!(!result.is_parallel);
    }

    #[test]
    fn test_parallel_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(2.0, 4.0, 6.0),
            vector2: create_test_vector(1.0, 2.0, 3.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!(result.angle_radians.abs() < 1e-10);
        assert!(result.angle_degrees.abs() < 1e-10);
        assert!((result.cos_angle - 1.0).abs() < 1e-10);
        assert!(!result.is_perpendicular);
        assert!(result.is_parallel);
    }

    #[test]
    fn test_opposite_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(-1.0, -2.0, -3.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_radians - std::f64::consts::PI).abs() < 1e-10);
        assert!((result.angle_degrees - 180.0).abs() < 1e-10);
        assert!((result.cos_angle + 1.0).abs() < 1e-10);
        assert!(!result.is_perpendicular);
        assert!(result.is_parallel);
    }

    #[test]
    fn test_45_degree_angle() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(1.0, 1.0, 0.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_radians - std::f64::consts::PI / 4.0).abs() < 1e-10);
        assert!((result.angle_degrees - 45.0).abs() < 1e-10);
        assert!((result.cos_angle - (2.0_f64.sqrt() / 2.0)).abs() < 1e-10);
        assert!(!result.is_perpendicular);
        assert!(!result.is_parallel);
    }

    #[test]
    fn test_vector_magnitudes() {
        let input = TwoVectorInput {
            vector1: create_test_vector(3.0, 4.0, 0.0), // Magnitude 5
            vector2: create_test_vector(0.0, 0.0, 2.0), // Magnitude 2
        };

        let result = vector_angle_logic(input).unwrap();
        assert_eq!(result.vector1_magnitude, 5.0);
        assert_eq!(result.vector2_magnitude, 2.0);
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
        assert!(result.is_perpendicular);
    }

    #[test]
    fn test_zero_vector_error() {
        let input = TwoVectorInput {
            vector1: create_test_vector(0.0, 0.0, 0.0),
            vector2: create_test_vector(1.0, 2.0, 3.0),
        };

        let result = vector_angle_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("zero vector"));
    }

    #[test]
    fn test_invalid_vector_nan() {
        let input = TwoVectorInput {
            vector1: create_test_vector(f64::NAN, 1.0, 2.0),
            vector2: create_test_vector(3.0, 4.0, 5.0),
        };

        let result = vector_angle_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_invalid_vector_infinite() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(f64::INFINITY, 4.0, 5.0),
        };

        let result = vector_angle_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_very_small_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1e-15, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1e-15, 0.0),
        };

        let result = vector_angle_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("zero vector"));
    }

    #[test]
    fn test_unit_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1.0, 0.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert_eq!(result.vector1_magnitude, 1.0);
        assert_eq!(result.vector2_magnitude, 1.0);
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_cos_angle_calculation() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.5, 0.5 * 3.0_f64.sqrt(), 0.0), // 60 degrees
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_degrees - 60.0).abs() < 1e-10);
        assert!((result.cos_angle - 0.5).abs() < 1e-10);
        assert!(!result.is_perpendicular);
        assert!(!result.is_parallel);
    }

    #[test]
    fn test_vector_validation() {
        let valid_vector = create_test_vector(1.0, 2.0, 3.0);
        assert!(valid_vector.is_valid());

        let invalid_vector = create_test_vector(f64::NAN, 2.0, 3.0);
        assert!(!invalid_vector.is_valid());

        let infinite_vector = create_test_vector(1.0, f64::INFINITY, 3.0);
        assert!(!infinite_vector.is_valid());
    }

    #[test]
    fn test_angle_precision() {
        // Test with vectors that should give exact known angles
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 1.0, 0.0),
            vector2: create_test_vector(1.0, -1.0, 0.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
        assert!((result.angle_degrees - 90.0).abs() < 1e-10);
        assert!(result.is_perpendicular);
    }

    #[test]
    fn test_3d_vectors() {
        let input = TwoVectorInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 0.0, 1.0),
        };

        let result = vector_angle_logic(input).unwrap();
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
        assert!(result.is_perpendicular);
        assert!(!result.is_parallel);
    }
}
