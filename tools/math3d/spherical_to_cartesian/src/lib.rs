use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{SphericalToCartesianInput, spherical_to_cartesian_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    coordinates: logic::SphericalCoord,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
    original_spherical: logic::SphericalCoord,
    cartesian_coordinates: logic::Vector3D,
    conversion_notes: String,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
pub fn spherical_to_cartesian_conversion(input: ToolInput) -> ToolResponse {
    let logic_input = SphericalToCartesianInput {
        coordinates: input.coordinates,
    };
    
    match spherical_to_cartesian_logic(logic_input) {
        Ok(output) => {
            let response = ToolResponse_ {
                original_spherical: output.original_spherical,
                cartesian_coordinates: output.cartesian_coordinates,
                conversion_notes: output.conversion_notes,
            };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}