use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{JsonFormatterInput as LogicInput, JsonFormatterResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonFormatterInput {
    /// JSON string to format
    pub json_string: String,
    /// Number of spaces for indentation (0 for compact, default is 2)
    pub indent: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonFormatterResult {
    /// Formatted JSON string
    pub formatted: String,
    /// Whether the input was valid JSON
    pub is_valid: bool,
    /// Error message if parsing failed
    pub error: Option<String>,
    /// Number of characters in input
    pub input_length: usize,
    /// Number of characters in output
    pub output_length: usize,
}

#[cfg_attr(not(test), tool)]
pub fn json_formatter(input: JsonFormatterInput) -> Result<JsonFormatterResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        json_string: input.json_string,
        indent: input.indent,
    };
    
    // Call logic implementation
    let result = logic::format_json(logic_input)?;
    
    // Convert back to wrapper types
    Ok(JsonFormatterResult {
        formatted: result.formatted,
        is_valid: result.is_valid,
        error: result.error,
        input_length: result.input_length,
        output_length: result.output_length,
    })
}