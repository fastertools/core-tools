use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{QuaternionSlerpInput, quaternion_slerp_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    q1: logic::Quaternion,
    q2: logic::Quaternion,
    t: f64,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
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
            let response = ToolResponse_ {
                result: output.result,
            };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}
