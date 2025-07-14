use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct TwoNumberInput {
    /// First number to multiply
    a: f64,
    /// Second number to multiply
    b: f64,
}

#[derive(Serialize)]
struct ArithmeticResult {
    result: f64,
    operation: String,
    inputs: Vec<f64>,
}

/// Multiply two numbers
#[tool]
fn multiply(input: TwoNumberInput) -> ToolResponse {
    let result = input.a * input.b;
    
    let response = ArithmeticResult {
        result,
        operation: "multiplication".to_string(),
        inputs: vec![input.a, input.b],
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}