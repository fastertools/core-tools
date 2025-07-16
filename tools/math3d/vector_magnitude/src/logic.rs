use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMagnitudeInput {
    pub vector: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorMagnitudeOutput {
    pub magnitude: f64,
    pub unit_vector: Vector3D,
    pub is_zero_vector: bool,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }

    pub fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }
}

pub fn compute_vector_magnitude(input: VectorMagnitudeInput) -> Result<VectorMagnitudeOutput, String> {
    let vector = input.vector;
    
    // Validate input - check for invalid values
    if vector.x.is_nan() || vector.x.is_infinite() ||
       vector.y.is_nan() || vector.y.is_infinite() ||
       vector.z.is_nan() || vector.z.is_infinite() {
        return Err("Input vector contains invalid values (NaN or Infinite)".to_string());
    }
    
    let magnitude = vector.magnitude();
    let is_zero_vector = vector.is_zero();
    
    let unit_vector = if is_zero_vector {
        Vector3D::new(0.0, 0.0, 0.0)
    } else {
        vector.normalize()?
    };
    
    Ok(VectorMagnitudeOutput {
        magnitude,
        unit_vector,
        is_zero_vector,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_standard_vector_magnitude() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(3.0, 4.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 5.0).abs() < 1e-10);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x - 0.6).abs() < 1e-10);
        assert!((result.unit_vector.y - 0.8).abs() < 1e-10);
        assert!((result.unit_vector.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_3d_vector_magnitude() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(1.0, 2.0, 2.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 3.0).abs() < 1e-10);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x - 1.0/3.0).abs() < 1e-10);
        assert!((result.unit_vector.y - 2.0/3.0).abs() < 1e-10);
        assert!((result.unit_vector.z - 2.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_vector() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(0.0, 0.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert_eq!(result.magnitude, 0.0);
        assert!(result.is_zero_vector);
        assert_eq!(result.unit_vector.x, 0.0);
        assert_eq!(result.unit_vector.y, 0.0);
        assert_eq!(result.unit_vector.z, 0.0);
    }

    #[test]
    fn test_near_zero_vector() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(1e-11, 1e-11, 1e-11),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!(result.is_zero_vector);
        assert_eq!(result.unit_vector.x, 0.0);
        assert_eq!(result.unit_vector.y, 0.0);
        assert_eq!(result.unit_vector.z, 0.0);
    }

    #[test]
    fn test_unit_vector() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(1.0, 0.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 1.0).abs() < 1e-10);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x - 1.0).abs() < 1e-10);
        assert!((result.unit_vector.y - 0.0).abs() < 1e-10);
        assert!((result.unit_vector.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_components() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(-3.0, -4.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 5.0).abs() < 1e-10);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x + 0.6).abs() < 1e-10);
        assert!((result.unit_vector.y + 0.8).abs() < 1e-10);
        assert!((result.unit_vector.z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_large_values() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(1e6, 0.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 1e6).abs() < 1e-4);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_small_values() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(1e-6, 0.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        assert!((result.magnitude - 1e-6).abs() < 1e-16);
        assert!(!result.is_zero_vector);
        assert!((result.unit_vector.x - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_nan_input_error() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(f64::NAN, 4.0, 0.0),
        };
        let result = compute_vector_magnitude(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input vector contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(3.0, f64::INFINITY, 0.0),
        };
        let result = compute_vector_magnitude(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input vector contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_negative_infinite_input_error() {
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(3.0, 4.0, f64::NEG_INFINITY),
        };
        let result = compute_vector_magnitude(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input vector contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_vector_magnitude_calculation() {
        // Test with Pythagorean theorem examples
        let test_cases = vec![
            ((3.0, 4.0, 0.0), 5.0),
            ((1.0, 1.0, 1.0), (3.0_f64).sqrt()),
            ((5.0, 12.0, 0.0), 13.0),
            ((8.0, 15.0, 0.0), 17.0),
            ((0.0, 0.0, 1.0), 1.0),
        ];

        for ((x, y, z), expected_magnitude) in test_cases {
            let input = VectorMagnitudeInput {
                vector: Vector3D::new(x, y, z),
            };
            let result = compute_vector_magnitude(input).unwrap();
            assert!((result.magnitude - expected_magnitude).abs() < 1e-10, 
                   "Failed for vector ({}, {}, {})", x, y, z);
        }
    }

    #[test]
    fn test_unit_vector_magnitude() {
        // Unit vectors should have magnitude 1
        let input = VectorMagnitudeInput {
            vector: Vector3D::new(3.0, 4.0, 0.0),
        };
        let result = compute_vector_magnitude(input).unwrap();
        let unit_magnitude = result.unit_vector.magnitude();
        assert!((unit_magnitude - 1.0).abs() < 1e-10);
    }
}