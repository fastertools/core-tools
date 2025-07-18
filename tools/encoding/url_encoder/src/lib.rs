use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{UrlEncoderInput as LogicInput, UrlEncoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlEncoderInput {
    /// The string to encode
    pub data: String,
    /// Encoding mode (optional, default: "component")
    /// Options: "component", "path", "query", "full"
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlEncoderOutput {
    /// URL encoded string
    pub encoded: String,
    /// Original data length
    pub original_length: usize,
    /// Encoded length
    pub encoded_length: usize,
    /// Encoding mode used
    pub mode: String,
    /// Number of characters encoded
    pub chars_encoded: usize,
}

#[cfg_attr(not(test), tool)]
pub fn url_encoder(input: UrlEncoderInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
        mode: input.mode,
    };
    
    // Call logic implementation
    match logic::encode_url(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let output = UrlEncoderOutput {
                encoded: result.encoded,
                original_length: result.original_length,
                encoded_length: result.encoded_length,
                mode: result.mode,
                chars_encoded: result.chars_encoded,
            };
            ToolResponse::text(serde_json::to_string(&output).unwrap())
        },
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}