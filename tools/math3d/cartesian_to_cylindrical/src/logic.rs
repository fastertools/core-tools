use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CartesianCoordinates {
    /// X coordinate
    pub x: f64,
    /// Y coordinate  
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CylindricalCoordinates {
    /// Distance from z-axis (ρ)
    pub radius: f64,
    /// Azimuthal angle in radians (θ)
    pub theta: f64,
    /// Height along z-axis
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CartesianToCylindricalResult {
    /// Original Cartesian coordinates
    pub original_cartesian: CartesianCoordinates,
    /// Converted cylindrical coordinates
    pub cylindrical_coordinates: CylindricalCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

impl CartesianCoordinates {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn to_cylindrical(&self) -> CylindricalCoordinates {
        let radius = (self.x * self.x + self.y * self.y).sqrt();
        let theta = self.y.atan2(self.x);

        CylindricalCoordinates {
            radius,
            theta,
            z: self.z,
        }
    }
}

impl CylindricalCoordinates {
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.z.is_finite()
            && self.radius >= 0.0
    }
}

pub fn cartesian_to_cylindrical_logic(
    input: CartesianCoordinates,
) -> Result<CartesianToCylindricalResult, String> {
    // Input validation
    if !input.is_valid() {
        return Err("Invalid Cartesian coordinates: contains NaN or infinite values".to_string());
    }

    let cylindrical = input.to_cylindrical();

    // Validate conversion result
    if !cylindrical.is_valid() {
        return Err("Conversion to cylindrical coordinates resulted in invalid values".to_string());
    }

    let conversion_notes = format!(
        "Converted from Cartesian ({:.3}, {:.3}, {:.3}) to Cylindrical (ρ={:.3}, θ={:.3} rad, z={:.3})",
        input.x, input.y, input.z, cylindrical.radius, cylindrical.theta, cylindrical.z
    );

    Ok(CartesianToCylindricalResult {
        original_cartesian: input,
        cylindrical_coordinates: cylindrical,
        conversion_notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let input = CartesianCoordinates {
            x: 1.0,
            y: 0.0,
            z: 2.0,
        };

        let result = cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.theta).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.z - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_45_degree_angle() {
        let input = CartesianCoordinates {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };

        let result = cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius - 2.0_f64.sqrt()).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.theta - std::f64::consts::PI / 4.0).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_origin() {
        let input = CartesianCoordinates {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let result = cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.z).abs() < 1e-15);
        // theta is undefined at origin but should be finite
        assert!(result.cylindrical_coordinates.theta.is_finite());
    }

    #[test]
    fn test_negative_coordinates() {
        let input = CartesianCoordinates {
            x: -1.0,
            y: -1.0,
            z: -2.0,
        };

        let result = cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius - 2.0_f64.sqrt()).abs() < 1e-15);
        assert!(
            (result.cylindrical_coordinates.theta - (-3.0 * std::f64::consts::PI / 4.0)).abs()
                < 1e-15
        );
        assert!((result.cylindrical_coordinates.z - (-2.0)).abs() < 1e-15);
    }

    #[test]
    fn test_invalid_input() {
        let input = CartesianCoordinates {
            x: f64::NAN,
            y: 0.0,
            z: 0.0,
        };

        let result = cartesian_to_cylindrical_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid Cartesian coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_infinite_input() {
        let input = CartesianCoordinates {
            x: f64::INFINITY,
            y: 0.0,
            z: 0.0,
        };

        let result = cartesian_to_cylindrical_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid Cartesian coordinates: contains NaN or infinite values"
        );
    }

    #[test]
    fn test_coordinate_validation() {
        let valid = CartesianCoordinates {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        };
        assert!(valid.is_valid());

        let invalid_nan = CartesianCoordinates {
            x: f64::NAN,
            y: 2.0,
            z: 3.0,
        };
        assert!(!invalid_nan.is_valid());

        let invalid_inf = CartesianCoordinates {
            x: f64::INFINITY,
            y: 2.0,
            z: 3.0,
        };
        assert!(!invalid_inf.is_valid());
    }

    #[test]
    fn test_cylindrical_validation() {
        let valid = CylindricalCoordinates {
            radius: 1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(valid.is_valid());

        let invalid_negative_radius = CylindricalCoordinates {
            radius: -1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(!invalid_negative_radius.is_valid());

        let invalid_nan = CylindricalCoordinates {
            radius: f64::NAN,
            theta: 0.0,
            z: 1.0,
        };
        assert!(!invalid_nan.is_valid());
    }

    #[test]
    fn test_specific_angles() {
        // Test specific coordinate positions
        let test_cases = vec![
            // (x, y, z) -> expected (radius, theta)
            (1.0, 0.0, 5.0, 1.0, 0.0),                          // +X axis
            (0.0, 1.0, 5.0, 1.0, std::f64::consts::PI / 2.0),   // +Y axis
            (-1.0, 0.0, 5.0, 1.0, std::f64::consts::PI),        // -X axis
            (0.0, -1.0, 5.0, 1.0, -std::f64::consts::PI / 2.0), // -Y axis
        ];

        for (x, y, z, expected_radius, expected_theta) in test_cases {
            let input = CartesianCoordinates { x, y, z };
            let result = cartesian_to_cylindrical_logic(input).unwrap();

            assert!(
                (result.cylindrical_coordinates.radius - expected_radius).abs() < 1e-14,
                "Radius mismatch for ({}, {}, {})",
                x,
                y,
                z
            );
            assert!(
                (result.cylindrical_coordinates.theta - expected_theta).abs() < 1e-14,
                "Theta mismatch for ({}, {}, {})",
                x,
                y,
                z
            );
            assert!(
                (result.cylindrical_coordinates.z - z).abs() < 1e-14,
                "Z mismatch for ({}, {}, {})",
                x,
                y,
                z
            );
        }
    }
}
