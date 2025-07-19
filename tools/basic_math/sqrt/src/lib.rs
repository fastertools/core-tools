use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
use ftl_sdk::ToolResponse;

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

// Individual component mode - FTL tool
#[cfg_attr(not(test), tool)]
pub fn sqrt(input: SingleNumberInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        value: input.value,
    };
    
    // Call logic implementation
    match logic::calculate_sqrt(logic_input) {
        Ok(result) => {
            let response = SquareRootResult {
                result: result.result,
                input: result.input,
                is_valid: result.is_valid,
                error: result.error,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}
