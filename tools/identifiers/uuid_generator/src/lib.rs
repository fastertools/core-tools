use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::ToolResponse;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{UuidGeneratorInput as LogicInput, UuidGeneratorOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UuidGeneratorInput {
    /// Number of UUIDs to generate (default: 1, max: 100)
    pub count: Option<u32>,
    /// Format for the UUIDs (default: "hyphenated")
    /// Options: "hyphenated", "simple", "urn", "braced"
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UuidGeneratorOutput {
    /// Generated UUID(s)
    pub uuids: Vec<String>,
    /// Version of UUID generated
    pub version: String,
    /// Format used
    pub format: String,
}

#[cfg_attr(not(test), tool)]
pub fn uuid_generator(input: UuidGeneratorInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        count: input.count,
        format: input.format,
    };

    // Call logic implementation
    let result = match logic::generate_uuids(logic_input) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Error: {}", e)),
    };

    // Convert back to wrapper types
    let output = UuidGeneratorOutput {
        uuids: result.uuids,
        version: result.version,
        format: result.format,
    };

    ToolResponse::text(
        serde_json::to_string_pretty(&output)
            .unwrap_or_else(|_| "Error serializing output".to_string()),
    )
}
