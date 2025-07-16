use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{ArbitraryRotationInput, arbitrary_rotation_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    axis: logic::Vector3D,
    angle: f64,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
    matrix: logic::Matrix3x3,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
fn arbitrary_rotation(input: ToolInput) -> ToolResponse {
    let logic_input = ArbitraryRotationInput {
        axis: input.axis,
        angle: input.angle,
    };
    
    match arbitrary_rotation_logic(logic_input) {
        Ok(output) => {
            let response = ToolResponse_ {
                matrix: output.matrix,
            };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}
