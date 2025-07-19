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
pub struct CartesianToSphericalInput {
    pub coordinates: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CartesianToSphericalOutput {
    pub original_cartesian: Vector3D,
    pub spherical_coordinates: SphericalCoord,
    pub conversion_notes: String,
}

impl Vector3D {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn to_spherical(&self) -> SphericalCoord {
        let radius = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        let theta = self.y.atan2(self.x);
        let phi = if radius > 0.0 {
            (self.z / radius).acos()
        } else {
            0.0
        };

        SphericalCoord { radius, theta, phi }
    }

    #[allow(dead_code)]
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl SphericalCoord {
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.phi.is_finite()
            && self.radius >= 0.0
    }
}

pub fn cartesian_to_spherical_logic(
    input: CartesianToSphericalInput,
) -> Result<CartesianToSphericalOutput, String> {
    // Input validation
    if !input.coordinates.is_valid() {
        return Err("Invalid Cartesian coordinates: contains NaN or infinite values".to_string());
    }

    // Perform conversion
    let spherical = input.coordinates.to_spherical();

    // Validate result
    if !spherical.is_valid() {
        return Err("Conversion resulted in invalid spherical coordinates".to_string());
    }

    let conversion_notes = format!(
        "Converted from Cartesian ({:.3}, {:.3}, {:.3}) to Spherical (r={:.3}, θ={:.3} rad, φ={:.3} rad)",
        input.coordinates.x,
        input.coordinates.y,
        input.coordinates.z,
        spherical.radius,
        spherical.theta,
        spherical.phi
    );

    Ok(CartesianToSphericalOutput {
        original_cartesian: input.coordinates,
        spherical_coordinates: spherical,
        conversion_notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        let cartesian = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi).abs() < 1e-15);
        // theta can be anything for origin, but should be finite
        assert!(result.spherical_coordinates.theta.is_finite());
    }

    #[test]
    fn test_positive_x_axis() {
        let cartesian = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.spherical_coordinates.theta).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi - std::f64::consts::PI / 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_positive_y_axis() {
        let cartesian = Vector3D {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.spherical_coordinates.theta - std::f64::consts::PI / 2.0).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi - std::f64::consts::PI / 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_positive_z_axis() {
        let cartesian = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi).abs() < 1e-15);
        // theta can be anything for points on z-axis, but should be finite
        assert!(result.spherical_coordinates.theta.is_finite());
    }

    #[test]
    fn test_negative_z_axis() {
        let cartesian = Vector3D {
            x: 0.0,
            y: 0.0,
            z: -1.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi - std::f64::consts::PI).abs() < 1e-15);
        // theta can be anything for points on z-axis, but should be finite
        assert!(result.spherical_coordinates.theta.is_finite());
    }

    #[test]
    fn test_arbitrary_point() {
        let cartesian = Vector3D {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();

        // Verify radius (should be sqrt(3² + 4² + 5²) = sqrt(50))
        let expected_radius = (9.0_f64 + 16.0 + 25.0).sqrt();
        assert!((result.spherical_coordinates.radius - expected_radius).abs() < 1e-14);

        // Verify theta (should be atan2(4, 3))
        let expected_theta = 4.0_f64.atan2(3.0);
        assert!((result.spherical_coordinates.theta - expected_theta).abs() < 1e-14);

        // Verify phi (should be acos(5/sqrt(50)))
        let expected_phi = (5.0_f64 / expected_radius).acos();
        assert!((result.spherical_coordinates.phi - expected_phi).abs() < 1e-14);
    }

    #[test]
    fn test_round_trip_conversion() {
        let original_points = vec![
            Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
            Vector3D {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
            Vector3D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vector3D {
                x: -1.0,
                y: 2.0,
                z: -3.0,
            },
        ];

        for original in original_points {
            let input = CartesianToSphericalInput {
                coordinates: original.clone(),
            };

            let result = cartesian_to_spherical_logic(input).unwrap();
            let spherical = &result.spherical_coordinates;

            // Convert back to Cartesian
            let sin_phi = spherical.phi.sin();
            let cos_phi = spherical.phi.cos();
            let sin_theta = spherical.theta.sin();
            let cos_theta = spherical.theta.cos();

            let converted_back = Vector3D {
                x: spherical.radius * sin_phi * cos_theta,
                y: spherical.radius * sin_phi * sin_theta,
                z: spherical.radius * cos_phi,
            };

            // Should match original within tolerance
            assert!(
                (converted_back.x - original.x).abs() < 1e-14,
                "X mismatch: {} vs {}",
                converted_back.x,
                original.x
            );
            assert!(
                (converted_back.y - original.y).abs() < 1e-14,
                "Y mismatch: {} vs {}",
                converted_back.y,
                original.y
            );
            assert!(
                (converted_back.z - original.z).abs() < 1e-14,
                "Z mismatch: {} vs {}",
                converted_back.z,
                original.z
            );
        }
    }

    #[test]
    fn test_nan_coordinates() {
        let cartesian = Vector3D {
            x: f64::NAN,
            y: 0.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid Cartesian coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_infinite_coordinates() {
        let cartesian = Vector3D {
            x: f64::INFINITY,
            y: 0.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid Cartesian coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_large_coordinates() {
        let cartesian = Vector3D {
            x: 1e10,
            y: 0.0,
            z: 0.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!((result.spherical_coordinates.radius - 1e10).abs() < 1e-5);
        assert!((result.spherical_coordinates.theta).abs() < 1e-15);
        assert!((result.spherical_coordinates.phi - std::f64::consts::PI / 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_negative_coordinates() {
        let cartesian = Vector3D {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();

        // Radius should be sqrt(3)
        let expected_radius = 3.0_f64.sqrt();
        assert!((result.spherical_coordinates.radius - expected_radius).abs() < 1e-14);

        // theta should be in third quadrant (atan2(-1, -1))
        let expected_theta = (-1.0_f64).atan2(-1.0);
        assert!((result.spherical_coordinates.theta - expected_theta).abs() < 1e-14);

        // phi should be acos(-1/sqrt(3))
        let expected_phi = (-1.0 / expected_radius).acos();
        assert!((result.spherical_coordinates.phi - expected_phi).abs() < 1e-14);
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
    fn test_spherical_validation() {
        let valid_spherical = SphericalCoord {
            radius: 1.0,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(valid_spherical.is_valid());

        let invalid_spherical = SphericalCoord {
            radius: f64::NAN,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(!invalid_spherical.is_valid());
    }

    #[test]
    fn test_conversion_notes() {
        let cartesian = Vector3D {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };

        let input = CartesianToSphericalInput {
            coordinates: cartesian,
        };

        let result = cartesian_to_spherical_logic(input).unwrap();
        assert!(result.conversion_notes.contains("Converted from Cartesian"));
        assert!(result.conversion_notes.contains("(1.000, 2.000, 3.000)"));
        assert!(result.conversion_notes.contains("to Spherical"));
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
    fn test_quadrant_angles() {
        // Test all four quadrants in xy-plane
        let test_cases = vec![
            (
                Vector3D {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                std::f64::consts::PI / 4.0,
            ), // First quadrant
            (
                Vector3D {
                    x: -1.0,
                    y: 1.0,
                    z: 0.0,
                },
                3.0 * std::f64::consts::PI / 4.0,
            ), // Second quadrant
            (
                Vector3D {
                    x: -1.0,
                    y: -1.0,
                    z: 0.0,
                },
                -3.0 * std::f64::consts::PI / 4.0,
            ), // Third quadrant
            (
                Vector3D {
                    x: 1.0,
                    y: -1.0,
                    z: 0.0,
                },
                -std::f64::consts::PI / 4.0,
            ), // Fourth quadrant
        ];

        for (cartesian, expected_theta) in test_cases {
            let input = CartesianToSphericalInput {
                coordinates: cartesian.clone(),
            };
            let result = cartesian_to_spherical_logic(input).unwrap();

            assert!(
                (result.spherical_coordinates.theta - expected_theta).abs() < 1e-14,
                "Theta mismatch for ({}, {}): expected {}, got {}",
                cartesian.x,
                cartesian.y,
                expected_theta,
                result.spherical_coordinates.theta
            );
        }
    }
}
