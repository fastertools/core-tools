use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Deserialize, JsonSchema)]
struct RotationMatrixInput {
    axis: String,
    angle: f64,
}

#[derive(Serialize)]
struct RotationMatrixResponse {
    matrix: Matrix3x3,
}

impl Matrix3x3 {
    pub fn rotation_x(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: cos_a, m12: -sin_a,
            m20: 0.0, m21: sin_a, m22: cos_a,
        }
    }

    pub fn rotation_y(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: 0.0, m02: sin_a,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: -sin_a, m21: 0.0, m22: cos_a,
        }
    }

    pub fn rotation_z(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: -sin_a, m02: 0.0,
            m10: sin_a, m11: cos_a, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    }
}

#[tool]
fn rotation_matrix(input: RotationMatrixInput) -> ToolResponse {
    let matrix = match input.axis.to_lowercase().as_str() {
        "x" => Matrix3x3::rotation_x(input.angle),
        "y" => Matrix3x3::rotation_y(input.angle),
        "z" => Matrix3x3::rotation_z(input.angle),
        _ => {
            return ToolResponse::error("Invalid axis. Use 'x', 'y', or 'z'");
        }
    };

    let response = RotationMatrixResponse { matrix };
    
    match serde_json::to_string(&response) {
        Ok(json) => ToolResponse::text(json),
        Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
    }
}
