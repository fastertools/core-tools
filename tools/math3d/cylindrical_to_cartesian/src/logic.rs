use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
pub struct CartesianCoordinates {
    /// X coordinate
    pub x: f64,
    /// Y coordinate  
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CylindricalToCartesianResult {
    /// Original cylindrical coordinates
    pub original_cylindrical: CylindricalCoordinates,
    /// Converted Cartesian coordinates
    pub cartesian_coordinates: CartesianCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

impl CylindricalCoordinates {
    pub fn is_valid(&self) -> bool {
        self.radius.is_finite()
            && self.theta.is_finite()
            && self.z.is_finite()
            && self.radius >= 0.0
    }

    pub fn to_cartesian(&self) -> CartesianCoordinates {
        let cos_theta = self.theta.cos();
        let sin_theta = self.theta.sin();

        CartesianCoordinates {
            x: self.radius * cos_theta,
            y: self.radius * sin_theta,
            z: self.z,
        }
    }
}

impl CartesianCoordinates {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn cylindrical_to_cartesian_logic(
    input: CylindricalCoordinates,
) -> Result<CylindricalToCartesianResult, String> {
    // Input validation
    if !input.is_valid() {
        return Err("Invalid cylindrical coordinates: radius must be non-negative and all values must be finite".to_string());
    }

    let cartesian = input.to_cartesian();

    // Validate conversion result
    if !cartesian.is_valid() {
        return Err("Conversion to Cartesian coordinates resulted in invalid values".to_string());
    }

    let conversion_notes = format!(
        "Converted from Cylindrical (ρ={:.3}, θ={:.3} rad, z={:.3}) to Cartesian ({:.3}, {:.3}, {:.3})",
        input.radius, input.theta, input.z, cartesian.x, cartesian.y, cartesian.z
    );

    Ok(CylindricalToCartesianResult {
        original_cylindrical: input,
        cartesian_coordinates: cartesian,
        conversion_notes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let input = CylindricalCoordinates {
            radius: 1.0,
            theta: 0.0,
            z: 2.0,
        };

        let result = cylindrical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_45_degree_angle() {
        let input = CylindricalCoordinates {
            radius: 2.0_f64.sqrt(),
            theta: std::f64::consts::PI / 4.0,
            z: 0.0,
        };

        let result = cylindrical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_origin() {
        let input = CylindricalCoordinates {
            radius: 0.0,
            theta: 0.0,
            z: 0.0,
        };

        let result = cylindrical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }

    #[test]
    fn test_negative_z() {
        let input = CylindricalCoordinates {
            radius: 1.0,
            theta: std::f64::consts::PI,
            z: -2.0,
        };

        let result = cylindrical_to_cartesian_logic(input).unwrap();
        assert!((result.cartesian_coordinates.x - (-1.0)).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z - (-2.0)).abs() < 1e-15);
    }

    #[test]
    fn test_invalid_negative_radius() {
        let input = CylindricalCoordinates {
            radius: -1.0,
            theta: 0.0,
            z: 0.0,
        };

        let result = cylindrical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid cylindrical coordinates: radius must be non-negative and all values must be finite"
        );
    }

    #[test]
    fn test_invalid_nan() {
        let input = CylindricalCoordinates {
            radius: f64::NAN,
            theta: 0.0,
            z: 0.0,
        };

        let result = cylindrical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid cylindrical coordinates: radius must be non-negative and all values must be finite"
        );
    }

    #[test]
    fn test_invalid_infinite() {
        let input = CylindricalCoordinates {
            radius: f64::INFINITY,
            theta: 0.0,
            z: 0.0,
        };

        let result = cylindrical_to_cartesian_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Invalid cylindrical coordinates: radius must be non-negative and all values must be finite"
        );
    }

    #[test]
    fn test_coordinate_validation() {
        let valid = CylindricalCoordinates {
            radius: 1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(valid.is_valid());

        let invalid_negative = CylindricalCoordinates {
            radius: -1.0,
            theta: 0.0,
            z: 1.0,
        };
        assert!(!invalid_negative.is_valid());

        let invalid_nan = CylindricalCoordinates {
            radius: f64::NAN,
            theta: 0.0,
            z: 1.0,
        };
        assert!(!invalid_nan.is_valid());
    }

    #[test]
    fn test_cartesian_validation() {
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
    fn test_specific_angles() {
        // Test specific angle positions
        let test_cases = vec![
            // (radius, theta, z) -> expected (x, y, z)
            (1.0, 0.0, 5.0, 1.0, 0.0, 5.0), // 0 radians
            (1.0, std::f64::consts::PI / 2.0, 5.0, 0.0, 1.0, 5.0), // π/2 radians
            (1.0, std::f64::consts::PI, 5.0, -1.0, 0.0, 5.0), // π radians
            (1.0, -std::f64::consts::PI / 2.0, 5.0, 0.0, -1.0, 5.0), // -π/2 radians
            (1.0, 3.0 * std::f64::consts::PI / 2.0, 5.0, 0.0, -1.0, 5.0), // 3π/2 radians
        ];

        for (radius, theta, z, expected_x, expected_y, expected_z) in test_cases {
            let input = CylindricalCoordinates { radius, theta, z };
            let result = cylindrical_to_cartesian_logic(input).unwrap();

            assert!(
                (result.cartesian_coordinates.x - expected_x).abs() < 1e-14,
                "X mismatch for (ρ={radius}, θ={theta}, z={z})"
            );
            assert!(
                (result.cartesian_coordinates.y - expected_y).abs() < 1e-14,
                "Y mismatch for (ρ={radius}, θ={theta}, z={z})"
            );
            assert!(
                (result.cartesian_coordinates.z - expected_z).abs() < 1e-14,
                "Z mismatch for (ρ={radius}, θ={theta}, z={z})"
            );
        }
    }

    #[test]
    fn test_round_trip_precision() {
        // Test that converting back preserves precision
        let original_cartesian = (3.0_f64, 4.0_f64, 5.0_f64);

        // Convert to cylindrical manually
        let radius = (original_cartesian.0 * original_cartesian.0
            + original_cartesian.1 * original_cartesian.1)
            .sqrt();
        let theta = original_cartesian.1.atan2(original_cartesian.0);

        let cylindrical_input = CylindricalCoordinates {
            radius,
            theta,
            z: original_cartesian.2,
        };

        let result = cylindrical_to_cartesian_logic(cylindrical_input).unwrap();

        assert!((result.cartesian_coordinates.x - original_cartesian.0).abs() < 1e-14);
        assert!((result.cartesian_coordinates.y - original_cartesian.1).abs() < 1e-14);
        assert!((result.cartesian_coordinates.z - original_cartesian.2).abs() < 1e-14);
    }
}
