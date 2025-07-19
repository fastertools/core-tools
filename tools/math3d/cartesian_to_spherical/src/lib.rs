use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{CartesianToSphericalInput, cartesian_to_spherical_logic};

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
pub struct SphericalCoordinates {
    /// Radial distance (radius)
    pub radius: f64,
    /// Polar angle (theta) in radians
    pub theta: f64,
    /// Azimuthal angle (phi) in radians
    pub phi: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CartesianToSphericalResult {
    /// Original cartesian coordinates
    pub original_cartesian: CartesianCoordinates,
    /// Converted spherical coordinates
    pub spherical_coordinates: SphericalCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

/// Convert Cartesian coordinates (x, y, z) to spherical coordinates (r, theta, phi)
#[cfg_attr(not(test), tool)]
pub fn cartesian_to_spherical(input: CartesianCoordinates) -> ToolResponse {
    let logic_input = CartesianToSphericalInput {
        coordinates: logic::Vector3D {
            x: input.x,
            y: input.y,
            z: input.z,
        },
    };

    match cartesian_to_spherical_logic(logic_input) {
        Ok(output) => {
            let result = CartesianToSphericalResult {
                original_cartesian: CartesianCoordinates {
                    x: output.original_cartesian.x,
                    y: output.original_cartesian.y,
                    z: output.original_cartesian.z,
                },
                spherical_coordinates: SphericalCoordinates {
                    radius: output.spherical_coordinates.radius,
                    theta: output.spherical_coordinates.theta,
                    phi: output.spherical_coordinates.phi,
                },
                conversion_notes: output.conversion_notes,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
