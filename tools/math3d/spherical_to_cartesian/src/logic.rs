use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SphericalCoord {
    pub radius: f64,
    pub theta: f64, // azimuthal angle (around z-axis)
    pub phi: f64,   // polar angle (from z-axis)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphericalToCartesianInput {
    pub coordinates: SphericalCoord,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SphericalToCartesianOutput {
    pub original_spherical: SphericalCoord,
    pub cartesian_coordinates: Vector3D,
    pub conversion_notes: String,
}

impl SphericalCoord {
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.phi.is_finite()
            && self.radius >= 0.0
    }

    pub fn to_cartesian(&self) -> Vector3D {
        let sin_phi = self.phi.sin();
        let cos_phi = self.phi.cos();
        let sin_theta = self.theta.sin();
        let cos_theta = self.theta.cos();

        Vector3D {
            x: self.radius * sin_phi * cos_theta,
            y: self.radius * sin_phi * sin_theta,
            z: self.radius * cos_phi,
        }
    }
}

impl Vector3D {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn spherical_to_cartesian_logic(
    input: SphericalToCartesianInput,
) -> Result<SphericalToCartesianOutput, String> {
    // Input validation
    if !input.coordinates.is_valid() {
        if input.coordinates.radius < 0.0 {
            return Err("Radius must be non-negative".to_string());
        }
        return Err("Invalid spherical coordinates: contains NaN or infinite values".to_string());
    }

    // Perform conversion
    let cartesian = input.coordinates.to_cartesian();

    // Validate result
    if !cartesian.is_valid() {
        return Err("Conversion resulted in invalid Cartesian coordinates".to_string());
    }

    let conversion_notes = format!(
        "Converted from Spherical (r={:.3}, θ={:.3} rad, φ={:.3} rad) to Cartesian ({:.3}, {:.3}, {:.3})",
        input.coordinates.radius,
        input.coordinates.theta,
        input.coordinates.phi,
        cartesian.x,
        cartesian.y,
        cartesian.z
    );

    Ok(SphericalToCartesianOutput {
        original_spherical: input.coordinates,
        cartesian_coordinates: cartesian,
        conversion_notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        let spherical = SphericalCoord {
            radius: 0.0,
            theta: 0.0,
            phi: 0.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_positive_z_axis() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: 0.0,
            phi: 0.0, // phi = 0 means pointing along positive z-axis
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_negative_z_axis() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: 0.0,
            phi: std::f64::consts::PI, // phi = π means pointing along negative z-axis
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z + 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_positive_x_axis() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: 0.0, // theta = 0 means in xz-plane toward positive x
            phi: std::f64::consts::PI / 2.0, // phi = π/2 means in xy-plane
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_positive_y_axis() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: std::f64::consts::PI / 2.0, // theta = π/2 means toward positive y
            phi: std::f64::consts::PI / 2.0,   // phi = π/2 means in xy-plane
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_arbitrary_point() {
        let radius = 5.0;
        let theta = std::f64::consts::PI / 4.0; // 45 degrees
        let phi = std::f64::consts::PI / 3.0; // 60 degrees

        let spherical = SphericalCoord { radius, theta, phi };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();

        // Manual calculation for verification
        let sin_phi = phi.sin();
        let cos_phi = phi.cos();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let expected_x = radius * sin_phi * cos_theta;
        let expected_y = radius * sin_phi * sin_theta;
        let expected_z = radius * cos_phi;

        assert!((result.cartesian_coordinates.x - expected_x).abs() < 1e-14);
        assert!((result.cartesian_coordinates.y - expected_y).abs() < 1e-14);
        assert!((result.cartesian_coordinates.z - expected_z).abs() < 1e-14);
    }

    #[test]
    fn test_negative_radius() {
        let spherical = SphericalCoord {
            radius: -1.0,
            theta: 0.0,
            phi: 0.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius must be non-negative");
    }

    #[test]
    fn test_nan_coordinates() {
        let spherical = SphericalCoord {
            radius: f64::NAN,
            theta: 0.0,
            phi: 0.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid spherical coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_infinite_coordinates() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: f64::INFINITY,
            phi: 0.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid spherical coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_large_radius() {
        let spherical = SphericalCoord {
            radius: 1e10,
            theta: 0.0,
            phi: std::f64::consts::PI / 2.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1e10).abs() < 1e-5);
        assert!((result.cartesian_coordinates.y).abs() < 1e-5);
        assert!((result.cartesian_coordinates.z).abs() < 1e-5);
    }

    #[test]
    fn test_full_rotation() {
        let spherical = SphericalCoord {
            radius: 1.0,
            theta: 2.0 * std::f64::consts::PI, // Full rotation
            phi: std::f64::consts::PI / 2.0,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        // Should be same as theta = 0
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-14);
        assert!((result.cartesian_coordinates.y).abs() < 1e-14);
        assert!((result.cartesian_coordinates.z).abs() < 1e-14);
    }

    #[test]
    fn test_spherical_validation() {
        let valid_coord = SphericalCoord {
            radius: 1.0,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(valid_coord.is_valid());

        let invalid_coord = SphericalCoord {
            radius: -1.0,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(!invalid_coord.is_valid());

        let nan_coord = SphericalCoord {
            radius: f64::NAN,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(!nan_coord.is_valid());
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
    }

    #[test]
    fn test_conversion_notes() {
        let spherical = SphericalCoord {
            radius: 2.0,
            theta: 1.0,
            phi: 0.5,
        };

        let input = SphericalToCartesianInput {
            coordinates: spherical,
        };

        let result = spherical_to_cartesian_logic(input).unwrap();
        assert!(result.conversion_notes.contains("Converted from Spherical"));
        assert!(result.conversion_notes.contains("r=2.000"));
        assert!(result.conversion_notes.contains("θ=1.000"));
        assert!(result.conversion_notes.contains("φ=0.500"));
    }

    #[test]
    fn test_multiple_conversions() {
        let test_cases = vec![
            (1.0, 0.0, 0.0, 0.0, 0.0, 1.0),                        // +Z axis
            (1.0, std::f64::consts::PI, 0.0, 0.0, 0.0, 1.0), // +Z axis (theta doesn't matter when phi=0)
            (1.0, 0.0, std::f64::consts::PI, 0.0, 0.0, -1.0), // -Z axis
            (1.0, 0.0, std::f64::consts::PI / 2.0, 1.0, 0.0, 0.0), // +X axis
            (
                1.0,
                std::f64::consts::PI / 2.0,
                std::f64::consts::PI / 2.0,
                0.0,
                1.0,
                0.0,
            ), // +Y axis
        ];

        for (radius, theta, phi, expected_x, expected_y, expected_z) in test_cases {
            let spherical = SphericalCoord { radius, theta, phi };
            let input = SphericalToCartesianInput {
                coordinates: spherical,
            };
            let result = spherical_to_cartesian_logic(input).unwrap();

            assert!(
                (result.cartesian_coordinates.x - expected_x).abs() < 1e-14,
                "X mismatch for r={}, θ={}, φ={}",
                radius,
                theta,
                phi
            );
            assert!(
                (result.cartesian_coordinates.y - expected_y).abs() < 1e-14,
                "Y mismatch for r={}, θ={}, φ={}",
                radius,
                theta,
                phi
            );
            assert!(
                (result.cartesian_coordinates.z - expected_z).abs() < 1e-14,
                "Z mismatch for r={}, θ={}, φ={}",
                radius,
                theta,
                phi
            );
        }
    }
}
