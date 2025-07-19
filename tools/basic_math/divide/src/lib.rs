use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{ArithmeticResult as LogicOutput, TwoNumberInput as LogicInput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoNumberInput {
    /// Dividend (number to be divided)
    pub a: f64,
    /// Divisor (number to divide by)
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArithmeticResult {
    pub result: f64,
    pub operation: String,
    pub inputs: Vec<f64>,
}

#[cfg(feature = "individual")]
#[cfg_attr(not(test), tool)]
pub fn divide(input: TwoNumberInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };

    // Call logic implementation
    match logic::divide_numbers(logic_input) {
        Ok(result) => {
            let response = ArithmeticResult {
                result: result.result,
                operation: result.operation,
                inputs: result.inputs,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
