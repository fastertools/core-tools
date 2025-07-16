use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{MatrixVectorInput, matrix_vector_multiply_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    matrix: logic::Matrix3x3,
    vector: logic::Vector3D,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
    result: logic::Vector3D,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
fn matrix_vector_multiply(input: ToolInput) -> ToolResponse {
    let logic_input = MatrixVectorInput {
        matrix: input.matrix,
        vector: input.vector,
    };
    
    match matrix_vector_multiply_logic(logic_input) {
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