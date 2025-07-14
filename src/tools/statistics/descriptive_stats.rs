use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct DescriptiveStatsInput {
    /// Array of numerical data
    data: Vec<f64>,
}

/// Calculate comprehensive descriptive statistics
#[tool]
fn descriptive_stats(input: DescriptiveStatsInput) -> ToolResponse {
    use crate::statistics::descriptive::{StatisticsInput, calculate_descriptive_statistics};
    
    let internal_input = StatisticsInput {
        data: input.data,
    };
    
    match calculate_descriptive_statistics(internal_input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}