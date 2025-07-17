use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{TwoNumberInput as LogicInput, ArithmeticResult as LogicOutput};

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

#[cfg_attr(not(test), tool)]
pub fn divide(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };
    
    // Call logic implementation
    let result = logic::divide_numbers(logic_input)?;
    
    // Convert back to wrapper types
    Ok(ArithmeticResult {
        result: result.result,
        operation: result.operation,
        inputs: result.inputs,
    })
}