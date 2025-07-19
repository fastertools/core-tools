use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
mod logic;
use ftl_sdk::ToolResponse;
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{StringTrimInput as LogicInput, StringTrimResult as LogicResult};

// Define wrapper types with JsonSchema for FTL-SDK (duplicating logic types but with JsonSchema)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringTrimInput {
    /// The text to process
    pub text: String,

    /// Operation type: trim, trim_start, trim_end, trim_char, trim_char_start,
    /// trim_char_end, pad, pad_left, pad_right, pad_center
    #[serde(default = "default_operation")]
    pub operation: String,

    /// Character to trim (for trim_char operations)
    #[serde(default)]
    pub char_to_trim: Option<String>,

    /// Target length for padding operations
    #[serde(default)]
    pub pad_length: Option<usize>,

    /// Character to use for padding (defaults to space)
    #[serde(default = "default_pad_char")]
    pub pad_char: String,

    /// Side to pad (for pad operation): left, right (default)
    #[serde(default = "default_pad_side")]
    pub pad_side: String,
}

fn default_operation() -> String {
    "trim".to_string()
}

fn default_pad_char() -> String {
    " ".to_string()
}

fn default_pad_side() -> String {
    "right".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringTrimResult {
    /// Original text
    pub original: String,
    /// Processed text
    pub processed: String,
    /// Operation performed
    pub operation: String,
    /// Length before processing
    pub length_before: usize,
    /// Length after processing
    pub length_after: usize,
}

#[cfg_attr(not(test), tool)]
pub fn string_trimmer(input: StringTrimInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        text: input.text,
        operation: input.operation,
        char_to_trim: input.char_to_trim,
        pad_length: input.pad_length,
        pad_char: input.pad_char,
        pad_side: input.pad_side,
    };

    // Call logic implementation
    let result = match logic::process_string(logic_input) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Error: {}", e)),
    };

    // Convert back to wrapper types
    let output = StringTrimResult {
        original: result.original,
        processed: result.processed,
        operation: result.operation,
        length_before: result.length_before,
        length_after: result.length_after,
    };

    ToolResponse::text(
        serde_json::to_string_pretty(&output)
            .unwrap_or_else(|_| "Error serializing output".to_string()),
    )
}
