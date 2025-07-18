use schemars::JsonSchema;
use ftl_sdk::{tool, ToolResponse};

mod logic;
use logic::{QuaternionSlerpInput, quaternion_slerp_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    q1: logic::Quaternion,
    q2: logic::Quaternion,
    t: f64,
}

#[derive(serde::Serialize, JsonSchema)]
struct ToolOutput {
    /// The interpolated quaternion result from spherical linear interpolation
    result: logic::Quaternion,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
fn quaternion_slerp(input: ToolInput) -> ToolResponse {
    let logic_input = QuaternionSlerpInput {
        q1: input.q1,
        q2: input.q2,
        t: input.t,
    };
    
    match quaternion_slerp_logic(logic_input) {
        Ok(output) => {
            let result = ToolOutput {
                result: output.result,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}
