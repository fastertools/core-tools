use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{StringCaseConverterInput as LogicInput, StringCaseConverterOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringCaseConverterInput {
    /// The text to convert
    pub text: String,
    /// Target case format
    /// Options: "lower", "upper", "title", "sentence", "camelCase", "PascalCase", 
    /// "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
    pub target_case: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StringCaseConverterOutput {
    /// Converted text
    pub converted: String,
    /// Original text
    pub original: String,
    /// Target case used
    pub target_case: String,
    /// Whether conversion was applied (false if already in target case)
    pub changed: bool,
}

#[cfg_attr(not(test), tool)]
pub fn string_case_converter(input: StringCaseConverterInput) -> Result<StringCaseConverterOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        text: input.text,
        target_case: input.target_case,
    };
    
    // Call logic implementation
    let result = logic::convert_case(logic_input)?;
    
    // Convert back to wrapper types
    Ok(StringCaseConverterOutput {
        converted: result.converted,
        original: result.original,
        target_case: result.target_case,
        changed: result.changed,
    })
}