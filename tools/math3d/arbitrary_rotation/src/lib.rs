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
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Deserialize, JsonSchema)]
struct ArbitraryRotationInput {
    axis: Vector3D,
    angle: f64,
}

#[derive(Serialize)]
struct ArbitraryRotationResponse {
    matrix: Matrix3x3,
}

impl Matrix3x3 {
    pub fn rotation_axis(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        let magnitude = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        let ux = axis.x / magnitude;
        let uy = axis.y / magnitude;
        let uz = axis.z / magnitude;

        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let one_minus_cos = 1.0 - cos_a;

        Ok(Matrix3x3 {
            m00: cos_a + ux * ux * one_minus_cos,
            m01: ux * uy * one_minus_cos - uz * sin_a,
            m02: ux * uz * one_minus_cos + uy * sin_a,
            m10: uy * ux * one_minus_cos + uz * sin_a,
            m11: cos_a + uy * uy * one_minus_cos,
            m12: uy * uz * one_minus_cos - ux * sin_a,
            m20: uz * ux * one_minus_cos - uy * sin_a,
            m21: uz * uy * one_minus_cos + ux * sin_a,
            m22: cos_a + uz * uz * one_minus_cos,
        })
    }
}

#[tool]
fn arbitrary_rotation(input: ArbitraryRotationInput) -> ToolResponse {
    match Matrix3x3::rotation_axis(&input.axis, input.angle) {
        Ok(matrix) => {
            let response = ArbitraryRotationResponse { matrix };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}
