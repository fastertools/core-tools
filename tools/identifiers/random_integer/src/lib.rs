use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{
    RandomIntegerInput as LogicInput, RandomIntegerOutput as LogicOutput, RandomRange as LogicRange,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RandomIntegerInput {
    /// Minimum value (inclusive)
    pub min: Option<i64>,
    /// Maximum value (inclusive)
    pub max: Option<i64>,
    /// Number of random integers to generate (default: 1, max: 100)
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RandomIntegerOutput {
    /// Generated random integer(s)
    pub values: Vec<i64>,
    /// Range used
    pub range: RandomRange,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RandomRange {
    pub min: i64,
    pub max: i64,
}

#[cfg_attr(not(test), tool)]
pub fn random_integer(input: RandomIntegerInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        min: input.min,
        max: input.max,
        count: input.count,
    };

    // Call logic implementation
    let result = match logic::generate_random_integers(logic_input) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Error: {e}")),
    };

    // Convert back to wrapper types
    let output = RandomIntegerOutput {
        values: result.values,
        range: RandomRange {
            min: result.range.min,
            max: result.range.max,
        },
    };

    ToolResponse::text(
        serde_json::to_string_pretty(&output)
            .unwrap_or_else(|_| "Error serializing output".to_string()),
    )
}
