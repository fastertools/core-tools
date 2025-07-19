#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Matrix3x3 {
    pub m00: f64,
    pub m01: f64,
    pub m02: f64,
    pub m10: f64,
    pub m11: f64,
    pub m12: f64,
    pub m20: f64,
    pub m21: f64,
    pub m22: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct RotationMatrixInput {
    pub axis: String,
    pub angle: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct RotationMatrixResponse {
    pub matrix: Matrix3x3,
}

#[cfg_attr(not(test), tool)]
pub fn rotation_matrix(input: RotationMatrixInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::RotationMatrixInput {
        axis: input.axis,
        angle: input.angle,
    };

    // Call business logic
    match logic::compute_rotation_matrix(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to API types
            let result = RotationMatrixResponse {
                matrix: Matrix3x3 {
                    m00: logic_result.matrix.m00,
                    m01: logic_result.matrix.m01,
                    m02: logic_result.matrix.m02,
                    m10: logic_result.matrix.m10,
                    m11: logic_result.matrix.m11,
                    m12: logic_result.matrix.m12,
                    m20: logic_result.matrix.m20,
                    m21: logic_result.matrix.m21,
                    m22: logic_result.matrix.m22,
                },
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
