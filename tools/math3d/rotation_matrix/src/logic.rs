use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationMatrixInput {
    pub axis: String,
    pub angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationMatrixResponse {
    pub matrix: Matrix3x3,
}

impl Matrix3x3 {
    pub fn rotation_x(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: cos_a, m12: -sin_a,
            m20: 0.0, m21: sin_a, m22: cos_a,
        }
    }

    pub fn rotation_y(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: 0.0, m02: sin_a,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: -sin_a, m21: 0.0, m22: cos_a,
        }
    }

    pub fn rotation_z(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: -sin_a, m02: 0.0,
            m10: sin_a, m11: cos_a, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    }
}

pub fn compute_rotation_matrix(input: RotationMatrixInput) -> Result<RotationMatrixResponse, String> {
    // Validate angle for NaN and infinite values
    if input.angle.is_nan() {
        return Err("Angle cannot be NaN".to_string());
    }
    if input.angle.is_infinite() {
        return Err("Angle cannot be infinite".to_string());
    }
    
    let matrix = match input.axis.to_lowercase().as_str() {
        "x" => Matrix3x3::rotation_x(input.angle),
        "y" => Matrix3x3::rotation_y(input.angle),
        "z" => Matrix3x3::rotation_z(input.angle),
        _ => {
            return Err("Invalid axis. Use 'x', 'y', or 'z'".to_string());
        }
    };

    Ok(RotationMatrixResponse { matrix })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_rotation_x_zero() {
        let input = RotationMatrixInput {
            axis: "x".to_string(),
            angle: 0.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_x_90_degrees() {
        let input = RotationMatrixInput {
            axis: "x".to_string(),
            angle: PI / 2.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 0.0, m12: -1.0,
            m20: 0.0, m21: 1.0, m22: 0.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_y_zero() {
        let input = RotationMatrixInput {
            axis: "y".to_string(),
            angle: 0.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_y_90_degrees() {
        let input = RotationMatrixInput {
            axis: "y".to_string(),
            angle: PI / 2.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 0.0, m01: 0.0, m02: 1.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: -1.0, m21: 0.0, m22: 0.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_z_zero() {
        let input = RotationMatrixInput {
            axis: "z".to_string(),
            angle: 0.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_z_90_degrees() {
        let input = RotationMatrixInput {
            axis: "z".to_string(),
            angle: PI / 2.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 0.0, m01: -1.0, m02: 0.0,
            m10: 1.0, m11: 0.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_x_180_degrees() {
        let input = RotationMatrixInput {
            axis: "x".to_string(),
            angle: PI,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: -1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: -1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_negative_angle() {
        let input = RotationMatrixInput {
            axis: "z".to_string(),
            angle: -PI / 2.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 0.0, m01: 1.0, m02: 0.0,
            m10: -1.0, m11: 0.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_rotation_large_angle() {
        let input = RotationMatrixInput {
            axis: "x".to_string(),
            angle: 4.0 * PI, // 720 degrees
        };
        let result = compute_rotation_matrix(input).unwrap();
        let expected = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-14);
    }

    #[test]
    fn test_rotation_small_angle() {
        let input = RotationMatrixInput {
            axis: "y".to_string(),
            angle: 0.001,
        };
        let result = compute_rotation_matrix(input).unwrap();
        // For small angles, cos ≈ 1, sin ≈ angle
        let expected = Matrix3x3 {
            m00: (0.001_f64).cos(), m01: 0.0, m02: (0.001_f64).sin(),
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: -(0.001_f64).sin(), m21: 0.0, m22: (0.001_f64).cos(),
        };
        assert_matrix_eq(&result.matrix, &expected, 1e-15);
    }

    #[test]
    fn test_uppercase_axis() {
        let input = RotationMatrixInput {
            axis: "X".to_string(),
            angle: PI / 4.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        // Should work with uppercase
        assert!((result.matrix.m00 - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_mixed_case_axis() {
        let input = RotationMatrixInput {
            axis: "Y".to_string(),
            angle: PI / 6.0,
        };
        let result = compute_rotation_matrix(input).unwrap();
        // Should work with mixed case
        assert!((result.matrix.m11 - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_invalid_axis_error() {
        let input = RotationMatrixInput {
            axis: "w".to_string(),
            angle: PI / 2.0,
        };
        let result = compute_rotation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid axis. Use 'x', 'y', or 'z'");
    }

    #[test]
    fn test_empty_axis_error() {
        let input = RotationMatrixInput {
            axis: "".to_string(),
            angle: PI / 2.0,
        };
        let result = compute_rotation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid axis. Use 'x', 'y', or 'z'");
    }

    #[test]
    fn test_nan_angle_error() {
        let input = RotationMatrixInput {
            axis: "x".to_string(),
            angle: f64::NAN,
        };
        let result = compute_rotation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Angle cannot be NaN");
    }

    #[test]
    fn test_infinite_angle_error() {
        let input = RotationMatrixInput {
            axis: "y".to_string(),
            angle: f64::INFINITY,
        };
        let result = compute_rotation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Angle cannot be infinite");
    }

    #[test]
    fn test_negative_infinite_angle_error() {
        let input = RotationMatrixInput {
            axis: "z".to_string(),
            angle: f64::NEG_INFINITY,
        };
        let result = compute_rotation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Angle cannot be infinite");
    }

    // Helper function to compare matrices with tolerance
    fn assert_matrix_eq(actual: &Matrix3x3, expected: &Matrix3x3, tolerance: f64) {
        assert!((actual.m00 - expected.m00).abs() < tolerance, "m00: {} != {}", actual.m00, expected.m00);
        assert!((actual.m01 - expected.m01).abs() < tolerance, "m01: {} != {}", actual.m01, expected.m01);
        assert!((actual.m02 - expected.m02).abs() < tolerance, "m02: {} != {}", actual.m02, expected.m02);
        assert!((actual.m10 - expected.m10).abs() < tolerance, "m10: {} != {}", actual.m10, expected.m10);
        assert!((actual.m11 - expected.m11).abs() < tolerance, "m11: {} != {}", actual.m11, expected.m11);
        assert!((actual.m12 - expected.m12).abs() < tolerance, "m12: {} != {}", actual.m12, expected.m12);
        assert!((actual.m20 - expected.m20).abs() < tolerance, "m20: {} != {}", actual.m20, expected.m20);
        assert!((actual.m21 - expected.m21).abs() < tolerance, "m21: {} != {}", actual.m21, expected.m21);
        assert!((actual.m22 - expected.m22).abs() < tolerance, "m22: {} != {}", actual.m22, expected.m22);
    }
}