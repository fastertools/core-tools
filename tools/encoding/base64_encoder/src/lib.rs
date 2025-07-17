use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{Base64EncoderInput as LogicInput, Base64EncoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Base64EncoderInput {
    /// The data to encode (as string)
    pub data: String,
    /// Encoding variant (optional, default: "standard")
    /// Options: "standard", "standard_no_pad", "url_safe", "url_safe_no_pad"
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Base64EncoderOutput {
    /// Base64 encoded string
    pub encoded: String,
    /// Original data length in bytes
    pub original_length: usize,
    /// Encoded length
    pub encoded_length: usize,
    /// Encoding variant used
    pub variant: String,
}

#[cfg_attr(not(test), tool)]
pub fn base64_encoder(input: Base64EncoderInput) -> Result<Base64EncoderOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
        variant: input.variant,
    };
    
    // Call logic implementation
    let result = logic::encode_base64(logic_input)?;
    
    // Convert back to wrapper types
    Ok(Base64EncoderOutput {
        encoded: result.encoded,
        original_length: result.original_length,
        encoded_length: result.encoded_length,
        variant: result.variant,
    })
}