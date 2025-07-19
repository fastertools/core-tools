#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{ArbitraryRotationInput, arbitrary_rotation_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    axis: logic::Vector3D,
    angle: f64,
}

#[derive(serde::Serialize, JsonSchema)]
struct ToolOutput {
    /// The 3x3 rotation matrix representing the rotation
    matrix: logic::Matrix3x3,
}

#[cfg_attr(not(test), tool)]
pub fn arbitrary_rotation(input: ToolInput) -> ToolResponse {
    let logic_input = ArbitraryRotationInput {
        axis: input.axis,
        angle: input.angle,
    };

    match arbitrary_rotation_logic(logic_input) {
        Ok(output) => {
            let result = ToolOutput {
                matrix: output.matrix,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
