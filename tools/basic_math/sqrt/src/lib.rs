use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct SingleNumberInput {
    /// Number to calculate square root of
    value: f64,
}

#[derive(Serialize)]
struct SquareRootResult {
    result: f64,
    input: f64,
    is_valid: bool,
    error: Option<String>,
}

/// Calculate the square root of a number with error handling for negative inputs
#[tool]
fn sqrt(input: SingleNumberInput) -> ToolResponse {
    let response = if input.value < 0.0 {
        SquareRootResult {
            result: f64::NAN,
            input: input.value,
            is_valid: false,
            error: Some("Cannot compute square root of negative number".to_string()),
        }
    } else {
        SquareRootResult {
            result: input.value.sqrt(),
            input: input.value,
            is_valid: true,
            error: None,
        }
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}