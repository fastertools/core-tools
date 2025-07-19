#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct QuaternionMultiplyInput {
    pub q1: Quaternion,
    pub q2: Quaternion,
}

#[derive(Serialize, JsonSchema)]
pub struct QuaternionMultiplyResponse {
    pub result: Quaternion,
}

#[cfg_attr(not(test), tool)]
pub fn quaternion_multiply(input: QuaternionMultiplyInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::QuaternionMultiplyInput {
        q1: logic::Quaternion {
            x: input.q1.x,
            y: input.q1.y,
            z: input.q1.z,
            w: input.q1.w,
        },
        q2: logic::Quaternion {
            x: input.q2.x,
            y: input.q2.y,
            z: input.q2.z,
            w: input.q2.w,
        },
    };

    // Call business logic
    match logic::compute_quaternion_multiply(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to API types
            let result = QuaternionMultiplyResponse {
                result: Quaternion {
                    x: logic_result.result.x,
                    y: logic_result.result.y,
                    z: logic_result.result.z,
                    w: logic_result.result.w,
                },
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
