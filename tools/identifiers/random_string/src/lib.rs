use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

use ftl_sdk::ToolResponse;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{
    RandomStringInput as LogicInput, 
    RandomStringOutput as LogicOutput,
    StringConfig as LogicConfig
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RandomStringInput {
    /// Length of the string to generate (default: 16, max: 1000)
    pub length: Option<u32>,
    /// Character set to use (default: "alphanumeric")
    /// Options: "alphanumeric", "alphabetic", "numeric", "lowercase", "uppercase", "hex"
    pub charset: Option<String>,
    /// Number of random strings to generate (default: 1, max: 100)
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RandomStringOutput {
    /// Generated random string(s)
    pub values: Vec<String>,
    /// Configuration used
    pub config: StringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringConfig {
    pub length: u32,
    pub charset: String,
    pub charset_size: usize,
}

#[cfg_attr(not(test), tool)]
pub fn random_string(input: RandomStringInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        length: input.length,
        charset: input.charset,
        count: input.count,
    };
    
    // Call logic implementation
    let result = match logic::generate_random_strings(logic_input) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Error: {}", e))
    };
    
    // Convert back to wrapper types
    let output = RandomStringOutput {
        values: result.values,
        config: StringConfig {
            length: result.config.length,
            charset: result.config.charset,
            charset_size: result.config.charset_size,
        },
    };
    
    ToolResponse::text(serde_json::to_string_pretty(&output).unwrap_or_else(|_| "Error serializing output".to_string()))
}