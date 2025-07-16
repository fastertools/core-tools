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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct QuaternionFromAxisAngleInput {
    pub axis: Vector3D,
    pub angle: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct QuaternionFromAxisAngleResponse {
    pub quaternion: Quaternion,
}

#[cfg_attr(not(test), tool)]
pub fn quaternion_from_axis_angle(input: QuaternionFromAxisAngleInput) -> Result<QuaternionFromAxisAngleResponse, String> {
    // Convert API types to logic types
    let logic_input = logic::QuaternionFromAxisAngleInput {
        axis: logic::Vector3D {
            x: input.axis.x,
            y: input.axis.y,
            z: input.axis.z,
        },
        angle: input.angle,
    };
    
    // Call business logic
    let logic_result = logic::compute_quaternion_from_axis_angle(logic_input)?;
    
    // Convert logic types back to API types
    Ok(QuaternionFromAxisAngleResponse {
        quaternion: Quaternion {
            x: logic_result.quaternion.x,
            y: logic_result.quaternion.y,
            z: logic_result.quaternion.z,
            w: logic_result.quaternion.w,
        },
    })
}
