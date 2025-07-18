use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

use ftl_sdk::ToolResponse;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{StringSplitInput as LogicInput, StringSplitResult as LogicResult};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringSplitInput {
    /// The text to split
    pub text: String,
    
    /// Delimiter for splitting (ignored for split_type: whitespace, lines, chars, words)
    #[serde(default = "default_delimiter")]
    pub delimiter: String,
    
    /// Split type: string, regex, whitespace, lines, chars, words
    #[serde(default = "default_split_type")]
    pub split_type: String,
    
    /// Maximum number of splits (None for unlimited)
    #[serde(default)]
    pub limit: Option<usize>,
    
    /// Whether to trim whitespace from each part
    #[serde(default)]
    pub trim_parts: bool,
    
    /// Whether to remove empty parts from result
    #[serde(default)]
    pub remove_empty: bool,
    
    /// Case sensitivity (for string split_type)
    #[serde(default)]
    pub case_sensitive: Option<bool>,
}

fn default_delimiter() -> String {
    " ".to_string()
}

fn default_split_type() -> String {
    "string".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringSplitResult {
    /// Array of split parts
    pub parts: Vec<String>,
    /// Number of parts
    pub count: usize,
    /// Original text
    pub original: String,
    /// Delimiter used (or description for special split types)
    pub delimiter_used: String,
    /// Split type used
    pub split_type: String,
}

#[cfg_attr(not(test), tool)]
pub fn string_splitter(input: StringSplitInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        text: input.text,
        delimiter: input.delimiter,
        split_type: input.split_type,
        limit: input.limit,
        trim_parts: input.trim_parts,
        remove_empty: input.remove_empty,
        case_sensitive: input.case_sensitive,
    };
    
    // Call logic implementation
    let result = match logic::split_string(logic_input) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Error: {}", e))
    };
    
    // Convert back to wrapper types
    let output = StringSplitResult {
        parts: result.parts,
        count: result.count,
        original: result.original,
        delimiter_used: result.delimiter_used,
        split_type: result.split_type,
    };
    
    ToolResponse::text(serde_json::to_string_pretty(&output).unwrap_or_else(|_| "Error serializing output".to_string()))
}