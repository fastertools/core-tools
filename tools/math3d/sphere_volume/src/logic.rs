use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphereVolumeInput {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphereVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub center: Vector3D,
    pub radius: f64,
}

pub fn compute_sphere_volume(input: SphereVolumeInput) -> Result<SphereVolumeResponse, String> {
    // Validate input
    if input.radius < 0.0 {
        return Err("Radius cannot be negative".to_string());
    }

    // Check for NaN and infinite values
    if input.radius.is_nan() {
        return Err("Radius cannot be NaN".to_string());
    }
    if input.radius.is_infinite() {
        return Err("Radius cannot be infinite".to_string());
    }
    if input.center.x.is_nan() || input.center.y.is_nan() || input.center.z.is_nan() {
        return Err("Center coordinates cannot contain NaN values".to_string());
    }
    if input.center.x.is_infinite() || input.center.y.is_infinite() || input.center.z.is_infinite()
    {
        return Err("Center coordinates cannot contain infinite values".to_string());
    }

    // Volume = (4/3) * π * r³
    let volume = (4.0 / 3.0) * std::f64::consts::PI * input.radius.powi(3);

    Ok(SphereVolumeResponse {
        volume,
        calculation_method: "Sphere formula: (4/3)πr³".to_string(),
        center: input.center,
        radius: input.radius,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_radius_sphere() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI;
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.radius, 1.0);
        assert_eq!(result.center.x, 0.0);
        assert_eq!(result.center.y, 0.0);
        assert_eq!(result.center.z, 0.0);
    }

    #[test]
    fn test_radius_2_sphere() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            radius: 2.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI * 8.0; // r³ = 8
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.radius, 2.0);
        assert_eq!(result.center.x, 1.0);
        assert_eq!(result.center.y, 2.0);
        assert_eq!(result.center.z, 3.0);
    }

    #[test]
    fn test_zero_radius_sphere() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 5.0,
                y: 10.0,
                z: 15.0,
            },
            radius: 0.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
        assert_eq!(result.radius, 0.0);
    }

    #[test]
    fn test_fractional_radius() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.5,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI * 0.125; // r³ = 0.125
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.radius, 0.5);
    }

    #[test]
    fn test_large_radius() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 100.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI * 1000000.0; // r³ = 1,000,000
        assert!((result.volume - expected).abs() < 1e-9);
        assert_eq!(result.radius, 100.0);
    }

    #[test]
    fn test_small_radius() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 0.001,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI * 0.000000001; // r³ = 1e-9
        assert!((result.volume - expected).abs() < 1e-18);
        assert_eq!(result.radius, 0.001);
    }

    #[test]
    fn test_negative_center_coordinates() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: -5.0,
                y: -10.0,
                z: -15.0,
            },
            radius: 3.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        let expected = (4.0 / 3.0) * std::f64::consts::PI * 27.0; // r³ = 27
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.center.x, -5.0);
        assert_eq!(result.center.y, -10.0);
        assert_eq!(result.center.z, -15.0);
    }

    #[test]
    fn test_calculation_method_field() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let result = compute_sphere_volume(input).unwrap();
        assert_eq!(result.calculation_method, "Sphere formula: (4/3)πr³");
    }

    #[test]
    fn test_negative_radius_error() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: -1.0,
        };
        let result = compute_sphere_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius cannot be negative");
    }

    #[test]
    fn test_nan_radius_error() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: f64::NAN,
        };
        let result = compute_sphere_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius cannot be NaN");
    }

    #[test]
    fn test_infinite_radius_error() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            radius: f64::INFINITY,
        };
        let result = compute_sphere_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius cannot be infinite");
    }

    #[test]
    fn test_nan_center_error() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: f64::NAN,
                y: 0.0,
                z: 0.0,
            },
            radius: 1.0,
        };
        let result = compute_sphere_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NaN values"));
    }

    #[test]
    fn test_infinite_center_error() {
        let input = SphereVolumeInput {
            center: Vector3D {
                x: 0.0,
                y: f64::INFINITY,
                z: 0.0,
            },
            radius: 1.0,
        };
        let result = compute_sphere_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite values"));
    }
}
