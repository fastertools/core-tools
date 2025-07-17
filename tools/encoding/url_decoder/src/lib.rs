use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{UrlDecoderInput as LogicInput, UrlDecoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlDecoderInput {
    /// The URL encoded string to decode
    pub encoded: String,
    /// Whether to decode plus signs as spaces (optional, default: false)
    /// This is common in query strings where spaces are encoded as +
    pub decode_plus: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct UrlDecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length
    pub decoded_length: usize,
    /// Number of percent-encoded sequences decoded
    pub sequences_decoded: usize,
    /// Whether the decoded result is valid UTF-8
    pub is_valid_utf8: bool,
    /// Error message if decoding failed
    pub error: Option<String>,
}

#[cfg_attr(not(test), tool)]
pub fn url_decoder(input: UrlDecoderInput) -> Result<UrlDecoderOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        encoded: input.encoded,
        decode_plus: input.decode_plus,
    };
    
    // Call logic implementation
    let result = logic::decode_url(logic_input)?;
    
    // Convert back to wrapper types
    Ok(UrlDecoderOutput {
        decoded: result.decoded,
        encoded_length: result.encoded_length,
        decoded_length: result.decoded_length,
        sequences_decoded: result.sequences_decoded,
        is_valid_utf8: result.is_valid_utf8,
        error: result.error,
    })
}