use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

// Re-export logic module types
mod logic;
pub use logic::*;

// FTL-compatible input type (with JsonSchema for HTTP interface)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoNumberInput {
    /// Base number
    pub a: f64,
    /// Exponent (power to raise the base to)
    pub b: f64,
}

// Core implementation - shared between both modes
fn power_impl(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    // Convert to logic types
    let logic_input = logic::TwoNumberInput {
        a: input.a,
        b: input.b,
    };
    
    // Call pure business logic
    logic::power_numbers(logic_input)
}

// Library mode: Primary export for pure function usage
#[cfg(feature = "library")]
pub fn power(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    power_impl(input)
}

// Individual mode: HTTP-based tool handler
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub fn power(input: TwoNumberInput) -> ToolResponse {
    match power_impl(input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}