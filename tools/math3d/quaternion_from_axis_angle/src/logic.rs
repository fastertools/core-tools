use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionFromAxisAngleInput {
    pub axis: Vector3D,
    pub angle: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionFromAxisAngleResponse {
    pub quaternion: Quaternion,
}

impl Quaternion {
    pub fn from_axis_angle(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        let magnitude = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        let half_angle = angle * 0.5;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();

        Ok(Quaternion {
            x: (axis.x / magnitude) * sin_half,
            y: (axis.y / magnitude) * sin_half,
            z: (axis.z / magnitude) * sin_half,
            w: cos_half,
        })
    }
}

pub fn compute_quaternion_from_axis_angle(
    input: QuaternionFromAxisAngleInput,
) -> Result<QuaternionFromAxisAngleResponse, String> {
    // Validate axis for NaN and infinite values
    if input.axis.x.is_nan() || input.axis.y.is_nan() || input.axis.z.is_nan() {
        return Err("Axis coordinates cannot contain NaN values".to_string());
    }
    if input.axis.x.is_infinite() || input.axis.y.is_infinite() || input.axis.z.is_infinite() {
        return Err("Axis coordinates cannot contain infinite values".to_string());
    }

    // Validate angle for NaN and infinite values
    if input.angle.is_nan() {
        return Err("Angle cannot be NaN".to_string());
    }
    if input.angle.is_infinite() {
        return Err("Angle cannot be infinite".to_string());
    }

    let quaternion = Quaternion::from_axis_angle(&input.axis, input.angle)?;

    Ok(QuaternionFromAxisAngleResponse { quaternion })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_zero_angle() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: 0.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        // Zero rotation should give identity quaternion
        assert_quaternion_eq(
            &result.quaternion,
            &Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            1e-15,
        );
    }

    #[test]
    fn test_x_axis_90_degrees() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: (PI / 4.0).sin(),
            y: 0.0,
            z: 0.0,
            w: (PI / 4.0).cos(),
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_y_axis_180_degrees() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            angle: PI,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_z_axis_90_degrees() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: (PI / 4.0).sin(),
            w: (PI / 4.0).cos(),
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_normalized_axis_automatically() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 2.0,
                y: 0.0,
                z: 0.0,
            }, // Unnormalized
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: (PI / 4.0).sin(),
            y: 0.0,
            z: 0.0,
            w: (PI / 4.0).cos(),
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_arbitrary_axis() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            angle: PI / 3.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();

        // Check quaternion magnitude is 1 (unit quaternion)
        let magnitude = (result.quaternion.x.powi(2)
            + result.quaternion.y.powi(2)
            + result.quaternion.z.powi(2)
            + result.quaternion.w.powi(2))
        .sqrt();
        assert!((magnitude - 1.0).abs() < 1e-15);

        // Axis components should be equal after normalization
        let sqrt3_inv = 1.0 / 3.0_f64.sqrt();
        let sin_sixth_pi = (PI / 6.0).sin();
        let expected_xyz = sqrt3_inv * sin_sixth_pi;
        assert!((result.quaternion.x - expected_xyz).abs() < 1e-15);
        assert!((result.quaternion.y - expected_xyz).abs() < 1e-15);
        assert!((result.quaternion.z - expected_xyz).abs() < 1e-15);
    }

    #[test]
    fn test_negative_angle() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            angle: -PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: -(PI / 4.0).sin(),
            w: (PI / 4.0).cos(),
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_large_angle() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: 4.0 * PI, // 720 degrees
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        // 720 degrees = 360 degrees x 2, should be identity
        assert_quaternion_eq(
            &result.quaternion,
            &Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            1e-14,
        );
    }

    #[test]
    fn test_small_angle() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: 0.001,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();
        let expected = Quaternion {
            x: (0.001_f64 / 2.0).sin(),
            y: 0.0,
            z: 0.0,
            w: (0.001_f64 / 2.0).cos(),
        };
        assert_quaternion_eq(&result.quaternion, &expected, 1e-15);
    }

    #[test]
    fn test_unit_quaternion_property() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            angle: PI / 4.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();

        // All quaternions from axis-angle should be unit quaternions
        let magnitude_squared = result.quaternion.x.powi(2)
            + result.quaternion.y.powi(2)
            + result.quaternion.z.powi(2)
            + result.quaternion.w.powi(2);
        assert!((magnitude_squared - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: -1.0,
                y: -1.0,
                z: -1.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input).unwrap();

        // Should work with negative axis coordinates
        let magnitude = (result.quaternion.x.powi(2)
            + result.quaternion.y.powi(2)
            + result.quaternion.z.powi(2)
            + result.quaternion.w.powi(2))
        .sqrt();
        assert!((magnitude - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_zero_axis_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Axis vector cannot be zero");
    }

    #[test]
    fn test_near_zero_axis_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1e-12,
                y: 1e-12,
                z: 1e-12,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Axis vector cannot be zero");
    }

    #[test]
    fn test_nan_axis_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: f64::NAN,
                y: 1.0,
                z: 0.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NaN"));
    }

    #[test]
    fn test_infinite_axis_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: f64::INFINITY,
                z: 0.0,
            },
            angle: PI / 2.0,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite"));
    }

    #[test]
    fn test_nan_angle_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: f64::NAN,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Angle cannot be NaN");
    }

    #[test]
    fn test_infinite_angle_error() {
        let input = QuaternionFromAxisAngleInput {
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            angle: f64::INFINITY,
        };
        let result = compute_quaternion_from_axis_angle(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Angle cannot be infinite");
    }

    // Helper function to compare quaternions with tolerance
    fn assert_quaternion_eq(actual: &Quaternion, expected: &Quaternion, tolerance: f64) {
        assert!(
            (actual.x - expected.x).abs() < tolerance,
            "x: {} != {}",
            actual.x,
            expected.x
        );
        assert!(
            (actual.y - expected.y).abs() < tolerance,
            "y: {} != {}",
            actual.y,
            expected.y
        );
        assert!(
            (actual.z - expected.z).abs() < tolerance,
            "z: {} != {}",
            actual.z,
            expected.z
        );
        assert!(
            (actual.w - expected.w).abs() < tolerance,
            "w: {} != {}",
            actual.w,
            expected.w
        );
    }
}
