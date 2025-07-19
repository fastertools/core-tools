use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{HexDecoderInput as LogicInput, HexDecoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HexDecoderInput {
    /// Hex encoded string to decode
    pub encoded: String,
    /// Whether to ignore whitespace in the input (optional, default: true)
    pub ignore_whitespace: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HexDecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Decoded data as UTF-8 string (if valid UTF-8)
    pub decoded_utf8: Option<String>,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length in bytes
    pub decoded_length: usize,
    /// Whether the decoded data is valid UTF-8
    pub is_valid_utf8: bool,
    /// Number of hex pairs decoded
    pub pairs_decoded: usize,
}

#[cfg_attr(not(test), tool)]
pub fn hex_decoder(input: HexDecoderInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        encoded: input.encoded,
        ignore_whitespace: input.ignore_whitespace,
    };

    // Call logic implementation
    match logic::decode_hex(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let output = HexDecoderOutput {
                decoded: result.decoded,
                decoded_utf8: result.decoded_utf8,
                encoded_length: result.encoded_length,
                decoded_length: result.decoded_length,
                is_valid_utf8: result.is_valid_utf8,
                pairs_decoded: result.pairs_decoded,
            };
            ToolResponse::text(serde_json::to_string(&output).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
