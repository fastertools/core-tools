use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;


// Re-export types from logic module
pub use logic::{JsonValidatorInput as LogicInput, JsonValidatorResult as LogicOutput, ValidationDetails as LogicDetails};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonValidatorInput {
    /// JSON string to validate
    pub json_string: String,
    /// Optional JSON schema to validate against (as JSON string)
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct JsonValidatorResult {
    /// Whether the JSON is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Detailed validation information
    pub details: ValidationDetails,
    /// Whether schema validation was performed
    pub schema_validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationDetails {
    /// Type of the root JSON value
    pub root_type: String,
    /// Number of keys (if object)
    pub key_count: Option<usize>,
    /// Number of elements (if array)
    pub element_count: Option<usize>,
    /// Maximum nesting depth
    pub max_depth: usize,
    /// Total number of values
    pub total_values: usize,
    /// Line number where error occurred (if applicable)
    pub error_line: Option<usize>,
    /// Column number where error occurred (if applicable)
    pub error_column: Option<usize>,
}

#[cfg_attr(not(test), tool)]
pub fn json_validator(input: JsonValidatorInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        json_string: input.json_string,
        schema: input.schema,
    };
    
    // Call logic implementation
    let result = match logic::validate_json(logic_input) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error validating JSON: {}", e)),
    };
    
    // Convert back to wrapper types
    let response = JsonValidatorResult {
        is_valid: result.is_valid,
        error: result.error,
        details: ValidationDetails {
            root_type: result.details.root_type,
            key_count: result.details.key_count,
            element_count: result.details.element_count,
            max_depth: result.details.max_depth,
            total_values: result.details.total_values,
            error_line: result.details.error_line,
            error_column: result.details.error_column,
        },
        schema_validated: result.schema_validated,
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap_or_else(|e| format!("Serialization error: {}", e)))
}