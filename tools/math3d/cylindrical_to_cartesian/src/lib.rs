use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{CylindricalCoordinates as LogicInput, cylindrical_to_cartesian_logic};

// Re-export for testing
pub use logic::{
    CylindricalCoordinates as LogicCylindrical,
    CartesianCoordinates as LogicCartesian,
    CylindricalToCartesianResult as LogicResult,
};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CylindricalCoordinates {
    /// Distance from z-axis (ρ)
    pub radius: f64,
    /// Azimuthal angle in radians (θ)
    pub theta: f64,
    /// Height along z-axis
    pub z: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CartesianCoordinates {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CylindricalToCartesianResult {
    /// Original cylindrical coordinates
    pub original_cylindrical: CylindricalCoordinates,
    /// Converted Cartesian coordinates
    pub cartesian_coordinates: CartesianCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

/// Convert cylindrical coordinates (ρ, θ, z) to Cartesian coordinates (x, y, z)
/// 
/// Cylindrical coordinates use:
/// - ρ (radius): distance from the z-axis
/// - θ (theta): azimuthal angle in radians around the z-axis
/// - z: height along the z-axis
/// 
/// Conversion formulas:
/// - x = ρ * cos(θ)
/// - y = ρ * sin(θ)
/// - z = z (unchanged)
#[cfg_attr(not(test), tool)]
pub fn cylindrical_to_cartesian(input: CylindricalCoordinates) -> ToolResponse {
    let logic_input = LogicInput {
        radius: input.radius,
        theta: input.theta,
        z: input.z,
    };
    
    match cylindrical_to_cartesian_logic(logic_input) {
        Ok(logic_result) => {
            let result = CylindricalToCartesianResult {
                original_cylindrical: CylindricalCoordinates {
                    radius: logic_result.original_cylindrical.radius,
                    theta: logic_result.original_cylindrical.theta,
                    z: logic_result.original_cylindrical.z,
                },
                cartesian_coordinates: CartesianCoordinates {
                    x: logic_result.cartesian_coordinates.x,
                    y: logic_result.cartesian_coordinates.y,
                    z: logic_result.cartesian_coordinates.z,
                },
                conversion_notes: logic_result.conversion_notes,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
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
        
        let result = cylindrical_to_cartesian(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z - 2.0).abs() < 1e-15);
    }

    #[test]
    fn test_45_degree_conversion() {
        let input = CylindricalCoordinates {
            radius: 2.0_f64.sqrt(),
            theta: std::f64::consts::PI / 4.0,
            z: 0.0,
        };
        
        let result = cylindrical_to_cartesian(input).unwrap();
        assert!((result.cartesian_coordinates.x - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.y - 1.0).abs() < 1e-15);
        assert!((result.cartesian_coordinates.z).abs() < 1e-15);
    }
}