use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{CartesianToSphericalInput, cartesian_to_spherical_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    coordinates: logic::Vector3D,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
    original_cartesian: logic::Vector3D,
    spherical_coordinates: logic::SphericalCoord,
    conversion_notes: String,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
pub fn cartesian_to_spherical_conversion(input: ToolInput) -> ToolResponse {
    let logic_input = CartesianToSphericalInput {
        coordinates: input.coordinates,
    };
    
    match cartesian_to_spherical_logic(logic_input) {
        Ok(output) => {
            let response = ToolResponse_ {
                original_cartesian: output.original_cartesian,
                spherical_coordinates: output.spherical_coordinates,
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