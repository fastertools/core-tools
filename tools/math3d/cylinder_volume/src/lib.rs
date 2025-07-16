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
pub struct CylinderVolumeInput {
    pub base_center: Vector3D,
    pub axis: Vector3D,
    pub radius: f64,
    pub height: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CylinderVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub base_center: Vector3D,
    pub axis: Vector3D,
    pub radius: f64,
    pub height: f64,
}

#[cfg_attr(not(test), tool)]
pub fn cylinder_volume(input: CylinderVolumeInput) -> Result<CylinderVolumeResponse, String> {
    // Convert API types to logic types
    let logic_input = logic::CylinderVolumeInput {
        base_center: logic::Vector3D {
            x: input.base_center.x,
            y: input.base_center.y,
            z: input.base_center.z,
        },
        axis: logic::Vector3D {
            x: input.axis.x,
            y: input.axis.y,
            z: input.axis.z,
        },
        radius: input.radius,
        height: input.height,
    };
    
    // Call business logic
    let logic_result = logic::compute_cylinder_volume(logic_input)?;
    
    // Convert logic types back to API types
    Ok(CylinderVolumeResponse {
        volume: logic_result.volume,
        calculation_method: logic_result.calculation_method,
        base_center: Vector3D {
            x: logic_result.base_center.x,
            y: logic_result.base_center.y,
            z: logic_result.base_center.z,
        },
        axis: Vector3D {
            x: logic_result.axis.x,
            y: logic_result.axis.y,
            z: logic_result.axis.z,
        },
        radius: logic_result.radius,
        height: logic_result.height,
    })
}