use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct Vector3D {
    /// X component of the vector
    pub x: f64,
    /// Y component of the vector
    pub y: f64,
    /// Z component of the vector
    pub z: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DotProductInput {
    /// First 3D vector
    pub vector1: Vector3D,
    /// Second 3D vector
    pub vector2: Vector3D,
}

#[derive(Serialize, Clone, Debug)]
pub struct DotProductResult {
    pub dot_product: f64,
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub are_perpendicular: bool,
    pub are_parallel: bool,
}

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }

    pub fn are_parallel(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        let cross = self.cross(other);
        cross.magnitude() < EPSILON
    }

    pub fn are_perpendicular(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        self.dot(other).abs() < EPSILON
    }

    pub fn angle_with(&self, other: &Vector3D) -> Result<f64, String> {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();

        if mag1 == 0.0 || mag2 == 0.0 {
            return Err("Cannot compute angle with zero vector".to_string());
        }

        let cos_angle = self.dot(other) / (mag1 * mag2);
        // Clamp to [-1, 1] to handle floating point errors
        let cos_angle = cos_angle.clamp(-1.0, 1.0);
        Ok(cos_angle.acos())
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn dot_product_logic(input: DotProductInput) -> Result<DotProductResult, String> {
    // Input validation
    if !input.vector1.is_valid() || !input.vector2.is_valid() {
        return Err("Invalid vector components: must be finite numbers".to_string());
    }

    let dot_product = input.vector1.dot(&input.vector2);
    let are_perpendicular = input.vector1.are_perpendicular(&input.vector2);
    let are_parallel = input.vector1.are_parallel(&input.vector2);

    let (angle_radians, angle_degrees) = if input.vector1.is_zero() || input.vector2.is_zero() {
        (0.0, 0.0)
    } else {
        match input.vector1.angle_with(&input.vector2) {
            Ok(angle_rad) => (angle_rad, angle_rad.to_degrees()),
            Err(_) => (0.0, 0.0),
        }
    };

    Ok(DotProductResult {
        dot_product,
        angle_radians,
        angle_degrees,
        are_perpendicular,
        are_parallel,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_vector(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    #[test]
    fn test_basic_dot_product() {
        let input = DotProductInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(4.0, 5.0, 6.0),
        };

        let result = dot_product_logic(input).unwrap();
        // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
        assert_eq!(result.dot_product, 32.0);
        assert!(!result.are_perpendicular);
        assert!(!result.are_parallel);
    }

    #[test]
    fn test_perpendicular_vectors() {
        let input = DotProductInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1.0, 0.0),
        };

        let result = dot_product_logic(input).unwrap();
        assert_eq!(result.dot_product, 0.0);
        assert!(result.are_perpendicular);
        assert!(!result.are_parallel);
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-10);
        assert!((result.angle_degrees - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_parallel_vectors() {
        let input = DotProductInput {
            vector1: create_test_vector(2.0, 4.0, 6.0),
            vector2: create_test_vector(1.0, 2.0, 3.0),
        };

        let result = dot_product_logic(input).unwrap();
        assert!(!result.are_perpendicular);
        assert!(result.are_parallel);
        assert!(result.angle_radians.abs() < 1e-10); // Should be 0 radians
        assert!(result.angle_degrees.abs() < 1e-10); // Should be 0 degrees
    }

    #[test]
    fn test_opposite_parallel_vectors() {
        let input = DotProductInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(-2.0, -4.0, -6.0),
        };

        let result = dot_product_logic(input).unwrap();
        assert!(!result.are_perpendicular);
        assert!(result.are_parallel);
        assert!((result.angle_radians - std::f64::consts::PI).abs() < 1e-10); // Should be π radians
        assert!((result.angle_degrees - 180.0).abs() < 1e-10); // Should be 180 degrees
    }

    #[test]
    fn test_dot_product_commutative() {
        let v1 = create_test_vector(2.0, 3.0, 4.0);
        let v2 = create_test_vector(5.0, 6.0, 7.0);

        let input1 = DotProductInput {
            vector1: v1.clone(),
            vector2: v2.clone(),
        };
        let input2 = DotProductInput {
            vector1: v2,
            vector2: v1,
        };

        let result1 = dot_product_logic(input1).unwrap();
        let result2 = dot_product_logic(input2).unwrap();

        // Dot product should be commutative: a · b = b · a
        assert_eq!(result1.dot_product, result2.dot_product);
        assert_eq!(result1.angle_radians, result2.angle_radians);
        assert_eq!(result1.angle_degrees, result2.angle_degrees);
    }

    #[test]
    fn test_zero_vector() {
        let input = DotProductInput {
            vector1: create_test_vector(0.0, 0.0, 0.0),
            vector2: create_test_vector(1.0, 2.0, 3.0),
        };

        let result = dot_product_logic(input).unwrap();
        assert_eq!(result.dot_product, 0.0);
        assert_eq!(result.angle_radians, 0.0);
        assert_eq!(result.angle_degrees, 0.0);
        // Zero vector is considered parallel to everything for this implementation
        assert!(result.are_parallel);
    }

    #[test]
    fn test_unit_vector_angle() {
        let input = DotProductInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.5_f64.sqrt(), 0.5_f64.sqrt(), 0.0), // 45-degree angle
        };

        let result = dot_product_logic(input).unwrap();
        assert!((result.dot_product - 0.5_f64.sqrt()).abs() < 1e-10);
        assert!((result.angle_radians - std::f64::consts::PI / 4.0).abs() < 1e-10);
        assert!((result.angle_degrees - 45.0).abs() < 1e-10);
    }

    #[test]
    fn test_dot_product_magnitude_relationship() {
        let v1 = create_test_vector(3.0, 4.0, 0.0); // Magnitude 5
        let v2 = create_test_vector(1.0, 0.0, 0.0); // Magnitude 1

        let input = DotProductInput {
            vector1: v1.clone(),
            vector2: v2.clone(),
        };

        let result = dot_product_logic(input).unwrap();
        // v1 · v2 = |v1| |v2| cos(θ)
        // For our vectors: dot = 3, |v1| = 5, |v2| = 1
        // So cos(θ) = 3/5 = 0.6
        let expected_cos = result.dot_product / (v1.magnitude() * v2.magnitude());
        assert!((expected_cos - 0.6).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_vector_nan() {
        let input = DotProductInput {
            vector1: create_test_vector(f64::NAN, 1.0, 2.0),
            vector2: create_test_vector(3.0, 4.0, 5.0),
        };

        let result = dot_product_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_invalid_vector_infinite() {
        let input = DotProductInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(f64::INFINITY, 4.0, 5.0),
        };

        let result = dot_product_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_vector_magnitude() {
        let v = create_test_vector(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);

        let v2 = create_test_vector(1.0, 1.0, 1.0);
        assert!((v2.magnitude() - 3.0_f64.sqrt()).abs() < 1e-10);
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
    fn test_angle_computation_edge_cases() {
        // Test vectors with very small magnitudes
        let v1 = create_test_vector(1e-15, 0.0, 0.0);
        let v2 = create_test_vector(0.0, 1e-15, 0.0);

        // These should be considered zero vectors
        assert!(v1.is_zero());
        assert!(v2.is_zero());
    }

    #[test]
    fn test_dot_product_linearity() {
        // Test that dot product is linear: (a + b) · c = a · c + b · c
        let a = create_test_vector(1.0, 2.0, 3.0);
        let b = create_test_vector(2.0, 3.0, 4.0);
        let c = create_test_vector(5.0, 6.0, 7.0);

        let a_plus_b = Vector3D {
            x: a.x + b.x,
            y: a.y + b.y,
            z: a.z + b.z,
        };

        let dot_a_c = a.dot(&c);
        let dot_b_c = b.dot(&c);
        let dot_ab_c = a_plus_b.dot(&c);

        assert!((dot_ab_c - (dot_a_c + dot_b_c)).abs() < 1e-10);
    }

    #[test]
    fn test_angle_clamping() {
        // Test that angle computation handles floating point errors correctly
        let v1 = create_test_vector(1.0, 0.0, 0.0);
        let v2 = create_test_vector(1.0, 0.0, 0.0);

        let angle = v1.angle_with(&v2).unwrap();
        assert!(angle.abs() < 1e-10); // Should be 0 for identical vectors
    }
}
