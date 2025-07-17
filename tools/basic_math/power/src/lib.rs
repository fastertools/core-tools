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
    /// Base number
    pub a: f64,
    /// Exponent (power to raise the base to)
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArithmeticResult {
    pub result: f64,
    pub operation: String,
    pub inputs: Vec<f64>,
}

#[cfg_attr(not(test), tool)]
pub fn power(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };
    
    // Call logic implementation
    let result = logic::power_numbers(logic_input)?;
    
    // Convert back to wrapper types
    Ok(ArithmeticResult {
        result: result.result,
        operation: result.operation,
        inputs: result.inputs,
    })
}