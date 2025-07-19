use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CylinderVolumeInput {
    pub base_center: Vector3D,
    pub axis: Vector3D,
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CylinderVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub base_center: Vector3D,
    pub axis: Vector3D,
    pub radius: f64,
    pub height: f64,
}

pub fn compute_cylinder_volume(
    input: CylinderVolumeInput,
) -> Result<CylinderVolumeResponse, String> {
    // Validate radius
    if input.radius < 0.0 {
        return Err("Radius cannot be negative".to_string());
    }
    if input.radius.is_nan() {
        return Err("Radius cannot be NaN".to_string());
    }
    if input.radius.is_infinite() {
        return Err("Radius cannot be infinite".to_string());
    }

    // Validate height
    if input.height < 0.0 {
        return Err("Height cannot be negative".to_string());
    }
    if input.height.is_nan() {
        return Err("Height cannot be NaN".to_string());
    }
    if input.height.is_infinite() {
        return Err("Height cannot be infinite".to_string());
    }

    // Validate base_center
    if input.base_center.x.is_nan() || input.base_center.y.is_nan() || input.base_center.z.is_nan()
    {
        return Err("Base center coordinates cannot contain NaN values".to_string());
    }
    if input.base_center.x.is_infinite()
        || input.base_center.y.is_infinite()
        || input.base_center.z.is_infinite()
    {
        return Err("Base center coordinates cannot contain infinite values".to_string());
    }

    // Validate axis
    if input.axis.x.is_nan() || input.axis.y.is_nan() || input.axis.z.is_nan() {
        return Err("Axis coordinates cannot contain NaN values".to_string());
    }
    if input.axis.x.is_infinite() || input.axis.y.is_infinite() || input.axis.z.is_infinite() {
        return Err("Axis coordinates cannot contain infinite values".to_string());
    }

    // Volume = π * r² * h
    let volume = std::f64::consts::PI * input.radius.powi(2) * input.height;

    Ok(CylinderVolumeResponse {
        volume,
        calculation_method: "Cylinder formula: πr²h".to_string(),
        base_center: input.base_center,
        axis: input.axis,
        radius: input.radius,
        height: input.height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_cylinder() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 1.0,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI;
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.radius, 1.0);
        assert_eq!(result.height, 1.0);
    }

    #[test]
    fn test_radius_2_height_3_cylinder() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            radius: 2.0,
            height: 3.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI * 4.0 * 3.0; // π * r² * h = π * 4 * 3
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.radius, 2.0);
        assert_eq!(result.height, 3.0);
    }

    #[test]
    fn test_zero_radius_cylinder() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.0,
            height: 5.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
    }

    #[test]
    fn test_zero_height_cylinder() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 5.0,
            height: 0.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
    }

    #[test]
    fn test_fractional_dimensions() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 0.5,
            height: 2.5,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI * 0.25 * 2.5; // π * 0.5² * 2.5
        assert!((result.volume - expected).abs() < 1e-15);
    }

    #[test]
    fn test_large_dimensions() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 100.0,
            height: 50.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI * 10000.0 * 50.0; // π * 100² * 50
        assert!((result.volume - expected).abs() < 1e-9);
    }

    #[test]
    fn test_small_dimensions() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 0.001,
            height: 0.002,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI * 0.000001 * 0.002; // π * 0.001² * 0.002
        assert!((result.volume - expected).abs() < 1e-18);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: -5.0,
                y: -10.0,
                z: -15.0,
            },
            axis: Vector3D {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 2.0,
            height: 4.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        let expected = std::f64::consts::PI * 4.0 * 4.0; // π * 2² * 4
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.base_center.x, -5.0);
        assert_eq!(result.axis.x, -1.0);
    }

    #[test]
    fn test_calculation_method_field() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 1.0,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input).unwrap();
        assert_eq!(result.calculation_method, "Cylinder formula: πr²h");
    }

    #[test]
    fn test_negative_radius_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: -1.0,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius cannot be negative");
    }

    #[test]
    fn test_negative_height_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 1.0,
            height: -1.0,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Height cannot be negative");
    }

    #[test]
    fn test_nan_radius_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: f64::NAN,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius cannot be NaN");
    }

    #[test]
    fn test_infinite_height_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 1.0,
            height: f64::INFINITY,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Height cannot be infinite");
    }

    #[test]
    fn test_nan_base_center_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: f64::NAN,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            radius: 1.0,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Base center"));
    }

    #[test]
    fn test_infinite_axis_error() {
        let input = CylinderVolumeInput {
            base_center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            axis: Vector3D {
                x: 0.0,
                y: f64::INFINITY,
                z: 0.0,
            },
            radius: 1.0,
            height: 1.0,
        };
        let result = compute_cylinder_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Axis"));
    }
}
