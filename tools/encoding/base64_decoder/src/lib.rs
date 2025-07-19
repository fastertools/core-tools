use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{Base64DecoderInput as LogicInput, Base64DecoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Base64DecoderInput {
    /// Base64 encoded string to decode
    pub encoded: String,
    /// Decoding variant (optional, default: "standard")
    /// Options: "standard", "standard_no_pad", "url_safe", "url_safe_no_pad"
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Base64DecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Decoded data as UTF-8 string (if valid UTF-8)
    pub decoded_utf8: Option<String>,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length in bytes
    pub decoded_length: usize,
    /// Decoding variant used
    pub variant: String,
    /// Whether the decoded data is valid UTF-8
    pub is_valid_utf8: bool,
}

#[cfg_attr(not(test), tool)]
pub fn base64_decoder(input: Base64DecoderInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        encoded: input.encoded,
        variant: input.variant,
    };

    // Call logic implementation
    match logic::decode_base64(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let output = Base64DecoderOutput {
                decoded: result.decoded,
                decoded_utf8: result.decoded_utf8,
                encoded_length: result.encoded_length,
                decoded_length: result.decoded_length,
                variant: result.variant,
                is_valid_utf8: result.is_valid_utf8,
            };
            ToolResponse::text(serde_json::to_string(&output).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
