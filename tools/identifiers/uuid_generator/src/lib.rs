use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

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
pub fn uuid_generator(input: UuidGeneratorInput) -> Result<UuidGeneratorOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        count: input.count,
        format: input.format,
    };
    
    // Call logic implementation
    let result = logic::generate_uuids(logic_input)?;
    
    // Convert back to wrapper types
    Ok(UuidGeneratorOutput {
        uuids: result.uuids,
        version: result.version,
        format: result.format,
    })
}