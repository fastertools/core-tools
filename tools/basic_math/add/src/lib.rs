use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct TwoNumberInput {
    /// First number to add
    a: f64,
    /// Second number to add
    b: f64,
}

#[derive(Serialize)]
struct ArithmeticResult {
    result: f64,
    operation: String,
    inputs: Vec<f64>,
}

/// Add two numbers together
#[tool]
fn add(input: TwoNumberInput) -> ToolResponse {
    let result = input.a + input.b;
    
    let response = ArithmeticResult {
        result,
        operation: "addition".to_string(),
        inputs: vec![input.a, input.b],
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}