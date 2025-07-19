use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

mod logic;

// Re-export types from logic module
pub use logic::{TwoNumberInput as LogicInput, ArithmeticResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoNumberInput {
    /// First number
    pub a: f64,
    /// Second number  
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArithmeticResult {
    /// The calculated result
    pub result: f64,
    /// The operation performed
    pub operation: String,
    /// The input values
    pub inputs: Vec<f64>,
}

/// Multiply two numbers together
#[cfg_attr(not(test), tool)]
pub fn multiply(input: TwoNumberInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };
    
    // Call logic implementation
    match logic::multiply_numbers(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = ArithmeticResult {
                result: result.result,
                operation: result.operation,
                inputs: result.inputs,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}