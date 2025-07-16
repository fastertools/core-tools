use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{SingleNumberInput as LogicInput, SquareRootResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SingleNumberInput {
    /// Number to calculate square root of
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SquareRootResult {
    pub result: f64,
    pub input: f64,
    pub is_valid: bool,
    pub error: Option<String>,
}

#[cfg_attr(not(test), tool)]
pub fn sqrt(input: SingleNumberInput) -> Result<SquareRootResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        value: input.value,
    };
    
    // Call logic implementation
    let result = logic::calculate_sqrt(logic_input)?;
    
    // Convert back to wrapper types
    Ok(SquareRootResult {
        result: result.result,
        input: result.input,
        is_valid: result.is_valid,
        error: result.error,
    })
}