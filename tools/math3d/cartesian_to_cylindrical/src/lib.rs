use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{CartesianCoordinates as LogicInput, cartesian_to_cylindrical_logic};

// Re-export for testing
pub use logic::{
    CartesianCoordinates as LogicCartesian, CartesianToCylindricalResult as LogicResult,
    CylindricalCoordinates as LogicCylindrical,
};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CartesianCoordinates {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CylindricalCoordinates {
    /// Distance from z-axis (ρ)
    pub radius: f64,
    /// Azimuthal angle in radians (θ)
    pub theta: f64,
    /// Height along z-axis
    pub z: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CartesianToCylindricalResult {
    /// Original Cartesian coordinates
    pub original_cartesian: CartesianCoordinates,
    /// Converted cylindrical coordinates
    pub cylindrical_coordinates: CylindricalCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

/// Convert Cartesian coordinates (x, y, z) to cylindrical coordinates (ρ, θ, z)
///
/// Cylindrical coordinates represent a point using:
/// - ρ (radius): distance from the z-axis
/// - θ (theta): azimuthal angle in radians around the z-axis
/// - z: height along the z-axis (unchanged from Cartesian)
#[cfg_attr(not(test), tool)]
pub fn cartesian_to_cylindrical(input: CartesianCoordinates) -> ToolResponse {
    let logic_input = LogicInput {
        x: input.x,
        y: input.y,
        z: input.z,
    };

    match cartesian_to_cylindrical_logic(logic_input) {
        Ok(logic_result) => {
            let result = CartesianToCylindricalResult {
                original_cartesian: CartesianCoordinates {
                    x: logic_result.original_cartesian.x,
                    y: logic_result.original_cartesian.y,
                    z: logic_result.original_cartesian.z,
                },
                cylindrical_coordinates: CylindricalCoordinates {
                    radius: logic_result.cylindrical_coordinates.radius,
                    theta: logic_result.cylindrical_coordinates.theta,
                    z: logic_result.cylindrical_coordinates.z,
                },
                conversion_notes: logic_result.conversion_notes,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        let input = LogicCartesian {
            x: 1.0,
            y: 0.0,
            z: 2.0,
        };

        let result = logic::cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius - 1.0).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.theta).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.z - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_45_degree_conversion() {
        let input = LogicCartesian {
            x: 1.0,
            y: 1.0,
            z: 0.0,
        };

        let result = logic::cartesian_to_cylindrical_logic(input).unwrap();
        assert!((result.cylindrical_coordinates.radius - 2.0_f64.sqrt()).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.theta - std::f64::consts::PI / 4.0).abs() < 1e-15);
        assert!((result.cylindrical_coordinates.z).abs() < 1e-15);
    }
}
