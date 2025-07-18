use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};

// Re-export types from logic module
pub use logic::{YamlFormatterInput as LogicInput, YamlFormatterResult as LogicOutput, YamlStats as LogicStats};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct YamlFormatterInput {
    /// YAML content to format
    pub content: String,
    /// Whether to validate YAML syntax
    pub validate_only: Option<bool>,
    /// Indentation spaces (default: 2)
    pub indent_spaces: Option<usize>,
    /// Whether to quote all string values
    pub quote_all_strings: Option<bool>,
    /// Whether to sort keys alphabetically
    pub sort_keys: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct YamlFormatterResult {
    /// Formatted YAML (if not validate_only)
    pub formatted: Option<String>,
    /// Whether the YAML is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Document statistics
    pub stats: YamlStats,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct YamlStats {
    /// Number of documents in the YAML
    pub document_count: usize,
    /// Total number of keys
    pub key_count: usize,
    /// Maximum nesting depth
    pub max_depth: usize,
    /// Types of values found
    pub value_types: Vec<String>,
}

#[cfg_attr(not(test), tool)]
pub fn yaml_formatter(input: YamlFormatterInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        content: input.content,
        validate_only: input.validate_only,
        indent_spaces: input.indent_spaces,
        quote_all_strings: input.quote_all_strings,
        sort_keys: input.sort_keys,
    };
    
    // Call logic implementation
    let result = match logic::format_yaml(logic_input) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error formatting YAML: {}", e)),
    };
    
    // Convert back to wrapper types
    let response = YamlFormatterResult {
        formatted: result.formatted,
        is_valid: result.is_valid,
        error: result.error,
        stats: YamlStats {
            document_count: result.stats.document_count,
            key_count: result.stats.key_count,
            max_depth: result.stats.max_depth,
            value_types: result.stats.value_types,
        },
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap_or_else(|e| format!("Serialization error: {}", e)))
}