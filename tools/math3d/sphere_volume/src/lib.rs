use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct SphereVolumeInput {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct SphereVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub center: Vector3D,
    pub radius: f64,
}

#[cfg_attr(not(test), tool)]
pub fn sphere_volume(input: SphereVolumeInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::SphereVolumeInput {
        center: logic::Vector3D {
            x: input.center.x,
            y: input.center.y,
            z: input.center.z,
        },
        radius: input.radius,
    };

    // Call business logic
    match logic::compute_sphere_volume(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to API types
            let result = SphereVolumeResponse {
                volume: logic_result.volume,
                calculation_method: logic_result.calculation_method,
                center: Vector3D {
                    x: logic_result.center.x,
                    y: logic_result.center.y,
                    z: logic_result.center.z,
                },
                radius: logic_result.radius,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
