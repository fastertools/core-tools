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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CylindricalCoord {
    pub radius: f64, // distance from z-axis
    pub theta: f64,  // azimuthal angle (around z-axis)
    pub z: f64,      // height along z-axis
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateConversionInput {
    pub from_type: String,
    pub to_type: String,
    pub coordinates: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinateConversionOutput {
    pub original: Vector3D,
    pub converted: Vector3D,
    pub from_type: String,
    pub to_type: String,
}

impl Vector3D {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub fn to_cylindrical(&self) -> CylindricalCoord {
        let radius = (self.x * self.x + self.y * self.y).sqrt();
        let theta = self.y.atan2(self.x);

        CylindricalCoord {
            radius,
            theta,
            z: self.z,
        }
    }
}

impl SphericalCoord {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.phi.is_finite()
            && self.radius >= 0.0
    }

    #[cfg(test)]
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

impl CylindricalCoord {
    #[cfg(test)]
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.z.is_finite()
            && self.radius >= 0.0
    }

    #[cfg(test)]
    pub fn to_cartesian(&self) -> Vector3D {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        Vector3D {
            x: self.radius * cos_theta,
            y: self.radius * sin_theta,
            z: self.z,
        }
    }
}

#[cfg(test)]
pub fn coordinate_conversion_logic(
    input: CoordinateConversionInput,
) -> Result<CoordinateConversionOutput, String> {
    // Input validation
    if !input.coordinates.is_valid() {
        return Err("Invalid coordinates: contains NaN or infinite values".to_string());
    }

    // Normalize coordinate system names
    let from_type = input.from_type.to_lowercase();
    let to_type = input.to_type.to_lowercase();

    let converted = match (from_type.as_str(), to_type.as_str()) {
        ("cartesian", "spherical") => {
            let spherical = input.coordinates.to_spherical();
            if !spherical.is_valid() {
                return Err(
                    "Conversion to spherical coordinates resulted in invalid values".to_string(),
                );
            }
            Vector3D {
                x: spherical.radius,
                y: spherical.theta,
                z: spherical.phi,
            }
        }
        ("spherical", "cartesian") => {
            let spherical = SphericalCoord {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                phi: input.coordinates.z,
            };
            if !spherical.is_valid() {
                return Err(
                    "Invalid spherical coordinates: radius must be non-negative".to_string()
                );
            }
            let cartesian = spherical.to_cartesian();
            if !cartesian.is_valid() {
                return Err(
                    "Conversion from spherical coordinates resulted in invalid values".to_string(),
                );
            }
            cartesian
        }
        ("cartesian", "cylindrical") => {
            let cylindrical = input.coordinates.to_cylindrical();
            if !cylindrical.is_valid() {
                return Err(
                    "Conversion to cylindrical coordinates resulted in invalid values".to_string(),
                );
            }
            Vector3D {
                x: cylindrical.radius,
                y: cylindrical.theta,
                z: cylindrical.z,
            }
        }
        ("cylindrical", "cartesian") => {
            let cylindrical = CylindricalCoord {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                z: input.coordinates.z,
            };
            if !cylindrical.is_valid() {
                return Err(
                    "Invalid cylindrical coordinates: radius must be non-negative".to_string(),
                );
            }
            let cartesian = cylindrical.to_cartesian();
            if !cartesian.is_valid() {
                return Err(
                    "Conversion from cylindrical coordinates resulted in invalid values"
                        .to_string(),
                );
            }
            cartesian
        }
        _ => {
            return Err("Invalid coordinate conversion. Supported: cartesian↔spherical, cartesian↔cylindrical".to_string());
        }
    };

    // Validate final result
    if !converted.is_valid() {
        return Err("Coordinate conversion resulted in invalid values".to_string());
    }

    Ok(CoordinateConversionOutput {
        original: input.coordinates,
        converted,
        from_type: input.from_type,
        to_type: input.to_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_to_spherical() {
        let cartesian = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let input = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "spherical".to_string(),
            coordinates: cartesian,
        };

        let result = coordinate_conversion_logic(input).unwrap();
        assert!((result.converted.x - 1.0).abs() < 1e-15); // radius
        assert!((result.converted.y).abs() < 1e-15); // theta
        assert!((result.converted.z - std::f64::consts::PI / 2.0).abs() < 1e-15); // phi
    }

    #[test]
    fn test_spherical_to_cartesian() {
        let spherical_as_cartesian = Vector3D {
            x: 1.0,                        // radius
            y: 0.0,                        // theta
            z: std::f64::consts::PI / 2.0, // phi
        };
        let input = CoordinateConversionInput {
            from_type: "spherical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: spherical_as_cartesian,
        };

        let result = coordinate_conversion_logic(input).unwrap();
        assert!((result.converted.x - 1.0).abs() < 1e-15);
        assert!((result.converted.y).abs() < 1e-15);
        assert!((result.converted.z).abs() < 1e-15);
    }

    #[test]
    fn test_cartesian_to_cylindrical() {
        let cartesian = Vector3D {
            x: 1.0,
            y: 0.0,
            z: 2.0,
        };
        let input = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "cylindrical".to_string(),
            coordinates: cartesian,
        };

        let result = coordinate_conversion_logic(input).unwrap();
        assert!((result.converted.x - 1.0).abs() < 1e-15); // radius
        assert!((result.converted.y).abs() < 1e-15); // theta
        assert!((result.converted.z - 2.0).abs() < 1e-15); // z
    }

    #[test]
    fn test_cylindrical_to_cartesian() {
        let cylindrical_as_cartesian = Vector3D {
            x: 1.0, // radius
            y: 0.0, // theta
            z: 2.0, // z
        };
        let input = CoordinateConversionInput {
            from_type: "cylindrical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: cylindrical_as_cartesian,
        };

        let result = coordinate_conversion_logic(input).unwrap();
        assert!((result.converted.x - 1.0).abs() < 1e-15);
        assert!((result.converted.y).abs() < 1e-15);
        assert!((result.converted.z - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_round_trip_cartesian_spherical() {
        let original = Vector3D {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        // Cartesian -> Spherical
        let to_spherical = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "spherical".to_string(),
            coordinates: original.clone(),
        };
        let spherical_result = coordinate_conversion_logic(to_spherical).unwrap();

        // Spherical -> Cartesian
        let back_to_cartesian = CoordinateConversionInput {
            from_type: "spherical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: spherical_result.converted,
        };
        let final_result = coordinate_conversion_logic(back_to_cartesian).unwrap();

        assert!((final_result.converted.x - original.x).abs() < 1e-14);
        assert!((final_result.converted.y - original.y).abs() < 1e-14);
        assert!((final_result.converted.z - original.z).abs() < 1e-14);
    }

    #[test]
    fn test_round_trip_cartesian_cylindrical() {
        let original = Vector3D {
            x: 3.0,
            y: 4.0,
            z: 5.0,
        };

        // Cartesian -> Cylindrical
        let to_cylindrical = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "cylindrical".to_string(),
            coordinates: original.clone(),
        };
        let cylindrical_result = coordinate_conversion_logic(to_cylindrical).unwrap();

        // Cylindrical -> Cartesian
        let back_to_cartesian = CoordinateConversionInput {
            from_type: "cylindrical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: cylindrical_result.converted,
        };
        let final_result = coordinate_conversion_logic(back_to_cartesian).unwrap();

        assert!((final_result.converted.x - original.x).abs() < 1e-14);
        assert!((final_result.converted.y - original.y).abs() < 1e-14);
        assert!((final_result.converted.z - original.z).abs() < 1e-14);
    }

    #[test]
    fn test_invalid_conversion_type() {
        let input = CoordinateConversionInput {
            from_type: "invalid".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: Vector3D {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        };

        let result = coordinate_conversion_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid coordinate conversion. Supported: cartesian↔spherical, cartesian↔cylindrical"
        );
    }

    #[test]
    fn test_case_insensitive_conversion_types() {
        let input = CoordinateConversionInput {
            from_type: "CARTESIAN".to_string(),
            to_type: "Spherical".to_string(),
            coordinates: Vector3D {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        };

        let result = coordinate_conversion_logic(input).unwrap();
        assert!((result.converted.x - 1.0).abs() < 1e-15); // radius should be 1
    }

    #[test]
    fn test_nan_coordinates() {
        let input = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "spherical".to_string(),
            coordinates: Vector3D {
                x: f64::NAN,
                y: 0.0,
                z: 0.0,
            },
        };

        let result = coordinate_conversion_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_infinite_coordinates() {
        let input = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "spherical".to_string(),
            coordinates: Vector3D {
                x: f64::INFINITY,
                y: 0.0,
                z: 0.0,
            },
        };

        let result = coordinate_conversion_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_negative_spherical_radius() {
        let input = CoordinateConversionInput {
            from_type: "spherical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: Vector3D {
                x: -1.0,
                y: 0.0,
                z: 0.0,
            }, // negative radius
        };

        let result = coordinate_conversion_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid spherical coordinates: radius must be non-negative"
        );
    }

    #[test]
    fn test_negative_cylindrical_radius() {
        let input = CoordinateConversionInput {
            from_type: "cylindrical".to_string(),
            to_type: "cartesian".to_string(),
            coordinates: Vector3D {
                x: -1.0,
                y: 0.0,
                z: 1.0,
            }, // negative radius
        };

        let result = coordinate_conversion_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid cylindrical coordinates: radius must be non-negative"
        );
    }

    #[test]
    fn test_origin_conversions() {
        let origin = Vector3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        // Origin to spherical
        let to_spherical = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "spherical".to_string(),
            coordinates: origin.clone(),
        };
        let spherical_result = coordinate_conversion_logic(to_spherical).unwrap();
        assert!((spherical_result.converted.x).abs() < 1e-15); // radius should be 0

        // Origin to cylindrical
        let to_cylindrical = CoordinateConversionInput {
            from_type: "cartesian".to_string(),
            to_type: "cylindrical".to_string(),
            coordinates: origin,
        };
        let cylindrical_result = coordinate_conversion_logic(to_cylindrical).unwrap();
        assert!((cylindrical_result.converted.x).abs() < 1e-15); // radius should be 0
        assert!((cylindrical_result.converted.z).abs() < 1e-15); // z should be 0
    }

    #[test]
    fn test_coordinate_validation() {
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

        let valid_spherical = SphericalCoord {
            radius: 1.0,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(valid_spherical.is_valid());

        let invalid_spherical = SphericalCoord {
            radius: -1.0,
            theta: 0.0,
            phi: 0.0,
        };
        assert!(!invalid_spherical.is_valid());

        let valid_cylindrical = CylindricalCoord {
            radius: 1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(valid_cylindrical.is_valid());

        let invalid_cylindrical = CylindricalCoord {
            radius: -1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(!invalid_cylindrical.is_valid());
    }

    #[test]
    fn test_specific_angle_conversions() {
        // Test specific angles for spherical conversions
        let test_cases = vec![
            // (x, y, z) -> expected (radius, theta, phi)
            (1.0, 0.0, 0.0, 1.0, 0.0, std::f64::consts::PI / 2.0), // +X axis
            (
                0.0,
                1.0,
                0.0,
                1.0,
                std::f64::consts::PI / 2.0,
                std::f64::consts::PI / 2.0,
            ), // +Y axis
            (0.0, 0.0, 1.0, 1.0, 0.0, 0.0),                        // +Z axis
            (0.0, 0.0, -1.0, 1.0, 0.0, std::f64::consts::PI),      // -Z axis
        ];

        for (x, y, z, expected_r, expected_theta, expected_phi) in test_cases {
            let input = CoordinateConversionInput {
                from_type: "cartesian".to_string(),
                to_type: "spherical".to_string(),
                coordinates: Vector3D { x, y, z },
            };

            let result = coordinate_conversion_logic(input).unwrap();
            assert!(
                (result.converted.x - expected_r).abs() < 1e-14,
                "Radius mismatch for ({x}, {y}, {z})"
            );
            // Note: theta can vary for points on z-axis, so we only check it for off-axis points
            if x != 0.0 || y != 0.0 {
                assert!(
                    (result.converted.y - expected_theta).abs() < 1e-14,
                    "Theta mismatch for ({x}, {y}, {z})"
                );
            }
            assert!(
                (result.converted.z - expected_phi).abs() < 1e-14,
                "Phi mismatch for ({x}, {y}, {z})"
            );
        }
    }
}
