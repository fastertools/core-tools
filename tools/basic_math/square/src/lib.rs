use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

// Re-export logic module types
mod logic;
pub use logic::*;

// FTL-compatible input type (with JsonSchema for HTTP interface)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SingleNumberInput {
    /// Number to square
    pub value: f64,
}

// Core implementation - shared between both modes
fn square_impl(input: SingleNumberInput) -> Result<ArithmeticResult, String> {
    // Convert to logic types
    let logic_input = logic::SingleNumberInput {
        value: input.value,
    };
    
    // Call pure business logic
    logic::square_number(logic_input)
}

// Library mode: Primary export for pure function usage
#[cfg(feature = "library")]
pub fn square(input: SingleNumberInput) -> Result<ArithmeticResult, String> {
    square_impl(input)
}

// Individual mode: HTTP-based tool handler
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub fn square(input: SingleNumberInput) -> ToolResponse {
    match square_impl(input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}