use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Vector3D {
    /// X component of the vector
    pub x: f64,
    /// Y component of the vector
    pub y: f64,
    /// Z component of the vector
    pub z: f64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CrossProductInput {
    /// First 3D vector
    pub vector1: Vector3D,
    /// Second 3D vector
    pub vector2: Vector3D,
}

#[derive(Serialize, Clone, Debug)]
pub struct CrossProductResult {
    pub cross_product: Vector3D,
    pub magnitude: f64,
    pub area_parallelogram: f64,
    pub are_parallel: bool,
}

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn are_parallel(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        let cross = self.cross(other);
        cross.magnitude() < EPSILON
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn cross_product_logic(input: CrossProductInput) -> Result<CrossProductResult, String> {
    // Input validation
    if !input.vector1.is_valid() || !input.vector2.is_valid() {
        return Err("Invalid vector components: must be finite numbers".to_string());
    }

    let cross_product = input.vector1.cross(&input.vector2);
    let magnitude = cross_product.magnitude();
    let area_parallelogram = magnitude;
    let are_parallel = input.vector1.are_parallel(&input.vector2);
    
    Ok(CrossProductResult {
        cross_product,
        magnitude,
        area_parallelogram,
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
    fn test_basic_cross_product() {
        let input = CrossProductInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1.0, 0.0),
        };

        let result = cross_product_logic(input).unwrap();
        assert_eq!(result.cross_product, create_test_vector(0.0, 0.0, 1.0));
        assert_eq!(result.magnitude, 1.0);
        assert_eq!(result.area_parallelogram, 1.0);
        assert!(!result.are_parallel);
    }

    #[test]
    fn test_cross_product_anticommutative() {
        let v1 = create_test_vector(2.0, 3.0, 4.0);
        let v2 = create_test_vector(5.0, 6.0, 7.0);

        let input1 = CrossProductInput {
            vector1: v1.clone(),
            vector2: v2.clone(),
        };
        let input2 = CrossProductInput {
            vector1: v2,
            vector2: v1,
        };

        let result1 = cross_product_logic(input1).unwrap();
        let result2 = cross_product_logic(input2).unwrap();

        // Cross product should be anticommutative: a × b = -(b × a)
        assert_eq!(result1.cross_product.x, -result2.cross_product.x);
        assert_eq!(result1.cross_product.y, -result2.cross_product.y);
        assert_eq!(result1.cross_product.z, -result2.cross_product.z);
        assert_eq!(result1.magnitude, result2.magnitude);
    }

    #[test]
    fn test_parallel_vectors() {
        let input = CrossProductInput {
            vector1: create_test_vector(2.0, 4.0, 6.0),
            vector2: create_test_vector(1.0, 2.0, 3.0), // Parallel (same direction)
        };

        let result = cross_product_logic(input).unwrap();
        assert!(result.are_parallel);
        assert!(result.magnitude < 1e-10);
        assert!(result.area_parallelogram < 1e-10);
    }

    #[test]
    fn test_opposite_parallel_vectors() {
        let input = CrossProductInput {
            vector1: create_test_vector(3.0, 6.0, 9.0),
            vector2: create_test_vector(-1.0, -2.0, -3.0), // Parallel (opposite direction)
        };

        let result = cross_product_logic(input).unwrap();
        assert!(result.are_parallel);
        assert!(result.magnitude < 1e-10);
    }

    #[test]
    fn test_perpendicular_vectors() {
        let input = CrossProductInput {
            vector1: create_test_vector(1.0, 0.0, 0.0),
            vector2: create_test_vector(0.0, 1.0, 0.0),
        };

        let result = cross_product_logic(input).unwrap();
        assert!(!result.are_parallel);
        assert_eq!(result.magnitude, 1.0);
        // For perpendicular unit vectors, area of parallelogram = 1
        assert_eq!(result.area_parallelogram, 1.0);
    }

    #[test]
    fn test_zero_vector() {
        let input = CrossProductInput {
            vector1: create_test_vector(0.0, 0.0, 0.0),
            vector2: create_test_vector(1.0, 2.0, 3.0),
        };

        let result = cross_product_logic(input).unwrap();
        assert_eq!(result.cross_product, create_test_vector(0.0, 0.0, 0.0));
        assert_eq!(result.magnitude, 0.0);
        assert!(result.are_parallel);
    }

    #[test]
    fn test_cross_product_magnitude() {
        let input = CrossProductInput {
            vector1: create_test_vector(3.0, 4.0, 0.0), // Magnitude 5
            vector2: create_test_vector(0.0, 0.0, 2.0), // Magnitude 2
        };

        let result = cross_product_logic(input).unwrap();
        // Cross product should be (8, -6, 0) with magnitude 10
        assert_eq!(result.cross_product, create_test_vector(8.0, -6.0, 0.0));
        assert_eq!(result.magnitude, 10.0);
        assert!(!result.are_parallel);
    }

    #[test]
    fn test_cross_product_formula() {
        let input = CrossProductInput {
            vector1: create_test_vector(2.0, 3.0, 4.0),
            vector2: create_test_vector(5.0, 6.0, 7.0),
        };

        let result = cross_product_logic(input).unwrap();
        // Manual calculation: (3*7 - 4*6, 4*5 - 2*7, 2*6 - 3*5) = (-3, 6, -3)
        assert_eq!(result.cross_product, create_test_vector(-3.0, 6.0, -3.0));
        assert!((result.magnitude - (9.0 + 36.0 + 9.0_f64).sqrt()).abs() < 1e-10);
    }

    #[test]
    fn test_invalid_vector_nan() {
        let input = CrossProductInput {
            vector1: create_test_vector(f64::NAN, 1.0, 2.0),
            vector2: create_test_vector(3.0, 4.0, 5.0),
        };

        let result = cross_product_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_invalid_vector_infinite() {
        let input = CrossProductInput {
            vector1: create_test_vector(1.0, 2.0, 3.0),
            vector2: create_test_vector(f64::INFINITY, 4.0, 5.0),
        };

        let result = cross_product_logic(input);
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
    fn test_area_parallelogram() {
        // For two perpendicular unit vectors, area should be 1
        let input = CrossProductInput {
            vector1: create_test_vector(2.0, 0.0, 0.0), // Length 2
            vector2: create_test_vector(0.0, 3.0, 0.0), // Length 3
        };

        let result = cross_product_logic(input).unwrap();
        // Area of parallelogram = |a × b| = 2 * 3 * sin(90°) = 6
        assert_eq!(result.area_parallelogram, 6.0);
    }

    #[test]
    fn test_cross_product_orthogonality() {
        let v1 = create_test_vector(1.0, 2.0, 3.0);
        let v2 = create_test_vector(4.0, 5.0, 6.0);
        
        let input = CrossProductInput {
            vector1: v1.clone(),
            vector2: v2.clone(),
        };

        let result = cross_product_logic(input).unwrap();
        let cross = &result.cross_product;

        // Cross product should be orthogonal to both input vectors
        // v1 · (v1 × v2) = 0 and v2 · (v1 × v2) = 0
        let dot1 = v1.x * cross.x + v1.y * cross.y + v1.z * cross.z;
        let dot2 = v2.x * cross.x + v2.y * cross.y + v2.z * cross.z;

        assert!(dot1.abs() < 1e-10);
        assert!(dot2.abs() < 1e-10);
    }
}