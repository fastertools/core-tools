use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{HexEncoderInput as LogicInput, HexEncoderOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HexEncoderInput {
    /// The data to encode (as string)
    pub data: String,
    /// Output case (optional, default: "lowercase")
    /// Options: "lowercase", "uppercase"
    pub case: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HexEncoderOutput {
    /// Hex encoded string
    pub encoded: String,
    /// Original data length in bytes
    pub original_length: usize,
    /// Encoded length (always 2x original for hex)
    pub encoded_length: usize,
    /// Output case used
    pub case: String,
}

#[cfg_attr(not(test), tool)]
pub fn hex_encoder(input: HexEncoderInput) -> Result<HexEncoderOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
        case: input.case,
    };
    
    // Call logic implementation
    let result = logic::encode_hex(logic_input)?;
    
    // Convert back to wrapper types
    Ok(HexEncoderOutput {
        encoded: result.encoded,
        original_length: result.original_length,
        encoded_length: result.encoded_length,
        case: result.case,
    })
}