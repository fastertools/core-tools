use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;

mod logic;
use logic::{MatrixVectorInput, matrix_vector_multiply_logic};

#[derive(serde::Deserialize, JsonSchema)]
pub struct ToolInput {
    matrix: logic::Matrix3x3,
    vector: logic::Vector3D,
}

#[derive(serde::Serialize, JsonSchema)]
struct ToolOutput {
    /// The resulting vector from matrix-vector multiplication
    result: logic::Vector3D,
}

#[cfg_attr(not(test), tool)]
pub fn matrix_vector_multiply(input: ToolInput) -> ToolResponse {
    let logic_input = MatrixVectorInput {
        matrix: input.matrix,
        vector: input.vector,
    };

    match matrix_vector_multiply_logic(logic_input) {
        Ok(output) => {
            let result = ToolOutput {
                result: output.result,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
