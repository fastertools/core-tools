use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

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
pub fn sphere_volume(input: SphereVolumeInput) -> Result<SphereVolumeResponse, String> {
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
    let logic_result = logic::compute_sphere_volume(logic_input)?;
    
    // Convert logic types back to API types
    Ok(SphereVolumeResponse {
        volume: logic_result.volume,
        calculation_method: logic_result.calculation_method,
        center: Vector3D {
            x: logic_result.center.x,
            y: logic_result.center.y,
            z: logic_result.center.z,
        },
        radius: logic_result.radius,
    })
}