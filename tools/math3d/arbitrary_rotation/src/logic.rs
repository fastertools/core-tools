use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Matrix3x3 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitraryRotationInput {
    pub axis: Vector3D,
    pub angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbitraryRotationOutput {
    pub matrix: Matrix3x3,
}

impl Vector3D {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Result<Self, String> {
        let magnitude = self.magnitude();
        if magnitude < 1e-10 {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(Vector3D {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
        })
    }
}

impl Matrix3x3 {
    pub fn is_valid(&self) -> bool {
        let values = [
            self.m00, self.m01, self.m02, self.m10, self.m11, self.m12, self.m20, self.m21,
            self.m22,
        ];
        values.iter().all(|&val| val.is_finite())
    }

    pub fn rotation_around_axis(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        if !axis.is_valid() {
            return Err("Invalid axis vector: contains NaN or infinite values".to_string());
        }

        if !angle.is_finite() {
            return Err("Invalid angle: must be finite".to_string());
        }

        let magnitude = axis.magnitude();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        // Normalize the axis vector
        let ux = axis.x / magnitude;
        let uy = axis.y / magnitude;
        let uz = axis.z / magnitude;

        // Rodrigues' rotation formula
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let one_minus_cos = 1.0 - cos_a;

        let matrix = Matrix3x3 {
            m00: cos_a + ux * ux * one_minus_cos,
            m01: ux * uy * one_minus_cos - uz * sin_a,
            m02: ux * uz * one_minus_cos + uy * sin_a,
            m10: uy * ux * one_minus_cos + uz * sin_a,
            m11: cos_a + uy * uy * one_minus_cos,
            m12: uy * uz * one_minus_cos - ux * sin_a,
            m20: uz * ux * one_minus_cos - uy * sin_a,
            m21: uz * uy * one_minus_cos + ux * sin_a,
            m22: cos_a + uz * uz * one_minus_cos,
        };

        if !matrix.is_valid() {
            return Err("Generated rotation matrix contains invalid values".to_string());
        }

        Ok(matrix)
    }

    pub fn multiply_vector(&self, v: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.m00 * v.x + self.m01 * v.y + self.m02 * v.z,
            y: self.m10 * v.x + self.m11 * v.y + self.m12 * v.z,
            z: self.m20 * v.x + self.m21 * v.y + self.m22 * v.z,
        }
    }

    pub fn determinant(&self) -> f64 {
        self.m00 * (self.m11 * self.m22 - self.m12 * self.m21)
            - self.m01 * (self.m10 * self.m22 - self.m12 * self.m20)
            + self.m02 * (self.m10 * self.m21 - self.m11 * self.m20)
    }
}

pub fn arbitrary_rotation_logic(
    input: ArbitraryRotationInput,
) -> Result<ArbitraryRotationOutput, String> {
    // Input validation
    if !input.axis.is_valid() {
        return Err("Invalid axis vector: contains NaN or infinite values".to_string());
    }

    if !input.angle.is_finite() {
        return Err("Invalid angle: must be finite".to_string());
    }

    // Generate rotation matrix
    let matrix = Matrix3x3::rotation_around_axis(&input.axis, input.angle)?;

    Ok(ArbitraryRotationOutput { matrix })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_rotation() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let angle = 0.0;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Should be identity matrix
        assert!((result.matrix.m00 - 1.0).abs() < 1e-15);
        assert!((result.matrix.m01).abs() < 1e-15);
        assert!((result.matrix.m02).abs() < 1e-15);
        assert!((result.matrix.m10).abs() < 1e-15);
        assert!((result.matrix.m11 - 1.0).abs() < 1e-15);
        assert!((result.matrix.m12).abs() < 1e-15);
        assert!((result.matrix.m20).abs() < 1e-15);
        assert!((result.matrix.m21).abs() < 1e-15);
        assert!((result.matrix.m22 - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_90_degree_z_rotation() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let angle = std::f64::consts::PI / 2.0; // 90 degrees

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Test rotation of unit vector along x-axis
        let test_vector = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let rotated = result.matrix.multiply_vector(&test_vector);

        // Should rotate to y-axis
        assert!(rotated.x.abs() < 1e-15);
        assert!((rotated.y - 1.0).abs() < 1e-15);
        assert!(rotated.z.abs() < 1e-15);
    }

    #[test]
    fn test_180_degree_x_rotation() {
        let axis = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let angle = std::f64::consts::PI; // 180 degrees

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Test rotation of unit vector along y-axis
        let test_vector = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let rotated = result.matrix.multiply_vector(&test_vector);

        // Should rotate to negative y-axis
        assert!(rotated.x.abs() < 1e-15);
        assert!((rotated.y + 1.0).abs() < 1e-15);
        assert!(rotated.z.abs() < 1e-15);
    }

    #[test]
    fn test_arbitrary_axis_rotation() {
        // Normalize axis (1,1,1)
        let axis = Vector3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let angle = std::f64::consts::PI / 3.0; // 60 degrees

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Verify it's a valid rotation matrix
        assert!(result.matrix.is_valid());

        // Rotation matrices should have determinant = 1
        let det = result.matrix.determinant();
        assert!((det - 1.0).abs() < 1e-14);
    }

    #[test]
    fn test_axis_remains_unchanged() {
        let axis = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let angle = std::f64::consts::PI / 4.0; // 45 degrees

        let input = ArbitraryRotationInput {
            axis: axis.clone(),
            angle,
        };
        let result = arbitrary_rotation_logic(input).unwrap();

        // The axis vector should remain unchanged after rotation
        let rotated_axis = result.matrix.multiply_vector(&axis);
        assert!((rotated_axis.x - axis.x).abs() < 1e-15);
        assert!((rotated_axis.y - axis.y).abs() < 1e-15);
        assert!((rotated_axis.z - axis.z).abs() < 1e-15);
    }

    #[test]
    fn test_zero_axis_vector() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let angle = std::f64::consts::PI / 2.0;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Axis vector cannot be zero");
    }

    #[test]
    fn test_nan_axis_vector() {
        let axis = Vector3D {
            x: f64::NAN,
            y: 0.0,
            z: 1.0,
        };
        let angle = std::f64::consts::PI / 2.0;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid axis vector: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_infinite_angle() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let angle = f64::INFINITY;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid angle: must be finite");
    }

    #[test]
    fn test_negative_angle() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let angle = -std::f64::consts::PI / 2.0; // -90 degrees

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Test rotation of unit vector along x-axis
        let test_vector = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let rotated = result.matrix.multiply_vector(&test_vector);

        // Should rotate to negative y-axis
        assert!(rotated.x.abs() < 1e-15);
        assert!((rotated.y + 1.0).abs() < 1e-15);
        assert!(rotated.z.abs() < 1e-15);
    }

    #[test]
    fn test_full_rotation() {
        let axis = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let angle = 2.0 * std::f64::consts::PI; // 360 degrees

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Should be approximately identity matrix
        assert!((result.matrix.m00 - 1.0).abs() < 1e-14);
        assert!((result.matrix.m11 - 1.0).abs() < 1e-14);
        assert!((result.matrix.m22 - 1.0).abs() < 1e-14);
        assert!(result.matrix.m01.abs() < 1e-14);
        assert!(result.matrix.m02.abs() < 1e-14);
        assert!(result.matrix.m10.abs() < 1e-14);
        assert!(result.matrix.m12.abs() < 1e-14);
        assert!(result.matrix.m20.abs() < 1e-14);
        assert!(result.matrix.m21.abs() < 1e-14);
    }

    #[test]
    fn test_large_axis_vector() {
        let axis = Vector3D {
            x: 1000.0,
            y: 0.0,
            z: 0.0,
        };
        let angle = std::f64::consts::PI / 2.0;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // Large axis should be normalized and work correctly
        let test_vector = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let rotated = result.matrix.multiply_vector(&test_vector);

        // Should rotate y to z
        assert!(rotated.x.abs() < 1e-15);
        assert!(rotated.y.abs() < 1e-15);
        assert!((rotated.z - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_vector_validation() {
        let valid_vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert!(valid_vector.is_valid());

        let invalid_vector = Vector3D {
            x: f64::NAN,
            y: 2.0,
            z: 3.0,
        };
        assert!(!invalid_vector.is_valid());

        let infinite_vector = Vector3D {
            x: f64::INFINITY,
            y: 2.0,
            z: 3.0,
        };
        assert!(!infinite_vector.is_valid());
    }

    #[test]
    fn test_vector_magnitude() {
        let vector = Vector3D {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let magnitude = vector.magnitude();
        assert!((magnitude - 5.0).abs() < 1e-15); // 3-4-5 triangle

        let zero_vector = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let magnitude = zero_vector.magnitude();
        assert!((magnitude).abs() < 1e-15);
    }

    #[test]
    fn test_vector_normalization() {
        let vector = Vector3D {
            x: 3.0,
            y: 4.0,
            z: 0.0,
        };
        let normalized = vector.normalize().unwrap();

        let magnitude = normalized.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-15);

        assert!((normalized.x - 0.6).abs() < 1e-15);
        assert!((normalized.y - 0.8).abs() < 1e-15);
        assert!(normalized.z.abs() < 1e-15);
    }

    #[test]
    fn test_zero_vector_normalization() {
        let zero_vector = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let result = zero_vector.normalize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot normalize zero vector");
    }

    #[test]
    fn test_matrix_validation() {
        let valid_matrix = Matrix3x3 {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        };
        assert!(valid_matrix.is_valid());

        let invalid_matrix = Matrix3x3 {
            m00: f64::NAN,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        };
        assert!(!invalid_matrix.is_valid());
    }

    #[test]
    fn test_matrix_determinant() {
        // Identity matrix should have determinant 1
        let identity = Matrix3x3 {
            m00: 1.0,
            m01: 0.0,
            m02: 0.0,
            m10: 0.0,
            m11: 1.0,
            m12: 0.0,
            m20: 0.0,
            m21: 0.0,
            m22: 1.0,
        };
        assert!((identity.determinant() - 1.0).abs() < 1e-15);

        // Test with arbitrary matrix
        let matrix = Matrix3x3 {
            m00: 2.0,
            m01: 1.0,
            m02: 3.0,
            m10: 0.0,
            m11: 4.0,
            m12: 1.0,
            m20: 0.0,
            m21: 0.0,
            m22: 5.0,
        };
        // Upper triangular matrix: det = product of diagonal = 2*4*5 = 40
        assert!((matrix.determinant() - 40.0).abs() < 1e-14);
    }

    #[test]
    fn test_rotation_matrix_properties() {
        let axis = Vector3D {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        };
        let angle = std::f64::consts::PI / 4.0;

        let input = ArbitraryRotationInput { axis, angle };
        let result = arbitrary_rotation_logic(input).unwrap();

        // All rotation matrices should have determinant 1
        let det = result.matrix.determinant();
        assert!((det - 1.0).abs() < 1e-14);

        // All rotation matrices should preserve vector lengths
        let test_vector = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        let original_length = test_vector.magnitude();
        let rotated = result.matrix.multiply_vector(&test_vector);
        let rotated_length = rotated.magnitude();

        assert!((original_length - rotated_length).abs() < 1e-14);
    }
}
