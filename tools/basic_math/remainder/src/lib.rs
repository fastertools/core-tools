use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{TwoNumberInput as LogicInput, ArithmeticResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoNumberInput {
    /// Number to find remainder of (dividend)
    pub a: f64,
    /// Number to divide by (divisor)
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
pub fn remainder(input: TwoNumberInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };
    
    // Call logic implementation
    match logic::remainder_numbers(logic_input) {
        Ok(result) => {
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

#[cfg(feature = "library")]
pub fn remainder_pure(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot calculate remainder with zero divisor".to_string())
    } else {
        Ok(a % b)
    }
}