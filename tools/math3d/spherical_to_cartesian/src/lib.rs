#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{SphericalToCartesianInput, spherical_to_cartesian_logic};

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct SphericalCoordinates {
    /// Radial distance (radius)
    pub radius: f64,
    /// Polar angle (theta) in radians
    pub theta: f64,
    /// Azimuthal angle (phi) in radians
    pub phi: f64,
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
pub struct SphericalToCartesianResult {
    /// Original spherical coordinates
    pub original_spherical: SphericalCoordinates,
    /// Converted cartesian coordinates
    pub cartesian_coordinates: CartesianCoordinates,
    /// Conversion notes
    pub conversion_notes: String,
}

/// Convert spherical coordinates (r, theta, phi) to Cartesian coordinates (x, y, z)
#[cfg_attr(not(test), tool)]
pub fn spherical_to_cartesian(input: SphericalCoordinates) -> ToolResponse {
    let logic_input = SphericalToCartesianInput {
        coordinates: logic::SphericalCoord {
            radius: input.radius,
            theta: input.theta,
            phi: input.phi,
        },
    };

    match spherical_to_cartesian_logic(logic_input) {
        Ok(output) => {
            let result = SphericalToCartesianResult {
                original_spherical: SphericalCoordinates {
                    radius: output.original_spherical.radius,
                    theta: output.original_spherical.theta,
                    phi: output.original_spherical.phi,
                },
                cartesian_coordinates: CartesianCoordinates {
                    x: output.cartesian_coordinates.x,
                    y: output.cartesian_coordinates.y,
                    z: output.cartesian_coordinates.z,
                },
                conversion_notes: output.conversion_notes,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
