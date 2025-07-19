use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

// Re-export types from logic module
pub use logic::{
    CsvParserInput as LogicInput, CsvParserResult as LogicOutput, ParsingStats as LogicStats,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CsvParserInput {
    /// CSV content to parse
    pub content: String,
    /// Whether first row contains headers
    pub has_headers: Option<bool>,
    /// Custom delimiter (default: comma)
    pub delimiter: Option<String>,
    /// Whether to skip empty lines
    pub skip_empty_lines: Option<bool>,
    /// Whether to trim whitespace from fields
    pub trim_fields: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CsvParserResult {
    /// Parsed headers (if any)
    pub headers: Option<Vec<String>>,
    /// Parsed data rows
    pub rows: Vec<Vec<String>>,
    /// Number of rows parsed
    pub row_count: usize,
    /// Number of columns detected
    pub column_count: usize,
    /// Parsing statistics
    pub stats: ParsingStats,
    /// Error if parsing failed
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ParsingStats {
    /// Total lines processed
    pub lines_processed: usize,
    /// Lines skipped (empty or malformed)
    pub lines_skipped: usize,
    /// Whether all rows have same column count
    pub uniform_columns: bool,
    /// Delimiter used
    pub delimiter_used: String,
}

#[cfg_attr(not(test), tool)]
pub fn csv_parser(input: CsvParserInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        content: input.content,
        has_headers: input.has_headers,
        delimiter: input.delimiter,
        skip_empty_lines: input.skip_empty_lines,
        trim_fields: input.trim_fields,
    };

    // Call logic implementation
    let result = match logic::parse_csv(logic_input) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error parsing CSV: {}", e)),
    };

    // Convert back to wrapper types
    let response = CsvParserResult {
        headers: result.headers,
        rows: result.rows,
        row_count: result.row_count,
        column_count: result.column_count,
        stats: ParsingStats {
            lines_processed: result.stats.lines_processed,
            lines_skipped: result.stats.lines_skipped,
            uniform_columns: result.stats.uniform_columns,
            delimiter_used: result.stats.delimiter_used,
        },
        error: result.error,
    };

    ToolResponse::text(
        serde_json::to_string(&response).unwrap_or_else(|e| format!("Serialization error: {}", e)),
    )
}
