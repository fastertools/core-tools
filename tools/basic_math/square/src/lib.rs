use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct SingleNumberInput {
    /// Number to square
    value: f64,
}

#[derive(Serialize)]
struct ArithmeticResult {
    result: f64,
    operation: String,
    inputs: Vec<f64>,
}

/// Calculate the square of a number
#[tool]
fn square(input: SingleNumberInput) -> ToolResponse {
    let result = input.value * input.value;
    
    let response = ArithmeticResult {
        result,
        operation: "square".to_string(),
        inputs: vec![input.value],
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}