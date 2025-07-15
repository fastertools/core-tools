use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, JsonSchema)]
struct QuaternionFromAxisAngleInput {
    axis: Vector3D,
    angle: f64,
}

#[derive(Serialize)]
struct QuaternionFromAxisAngleResponse {
    quaternion: Quaternion,
}

impl Quaternion {
    pub fn from_axis_angle(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        let magnitude = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        let half_angle = angle * 0.5;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();

        Ok(Quaternion {
            x: (axis.x / magnitude) * sin_half,
            y: (axis.y / magnitude) * sin_half,
            z: (axis.z / magnitude) * sin_half,
            w: cos_half,
        })
    }
}

#[tool]
fn quaternion_from_axis_angle(input: QuaternionFromAxisAngleInput) -> ToolResponse {
    match Quaternion::from_axis_angle(&input.axis, input.angle) {
        Ok(quaternion) => {
            let response = QuaternionFromAxisAngleResponse { quaternion };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}
