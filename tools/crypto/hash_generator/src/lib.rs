use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{HashGeneratorInput as LogicInput, HashGeneratorResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HashGeneratorInput {
    /// Text to hash
    pub text: String,
    /// Hash algorithm (md5, sha256, sha512)
    pub algorithm: String,
    /// Output format (hex, base64) - defaults to hex
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HashGeneratorResult {
    /// The computed hash
    pub hash: String,
    /// Algorithm used
    pub algorithm: String,
    /// Output format used
    pub format: String,
    /// Length of the hash in bytes
    pub byte_length: usize,
    /// Length of the hash string
    pub string_length: usize,
    /// Input text length
    pub input_length: usize,
}

#[cfg_attr(not(test), tool)]
pub fn hash_generator(input: HashGeneratorInput) -> Result<HashGeneratorResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        text: input.text,
        algorithm: input.algorithm,
        format: input.format,
    };
    
    // Call logic implementation
    let result = logic::generate_hash(logic_input)?;
    
    // Convert back to wrapper types
    Ok(HashGeneratorResult {
        hash: result.hash,
        algorithm: result.algorithm,
        format: result.format,
        byte_length: result.byte_length,
        string_length: result.string_length,
        input_length: result.input_length,
    })
}