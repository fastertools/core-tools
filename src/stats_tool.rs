use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct StatsInput {
    /// Array of numerical data for statistical analysis
    data: Vec<f64>,
}

/// Calculate comprehensive descriptive statistics for a dataset
#[tool]
fn descriptive_stats(input: StatsInput) -> ToolResponse {
    use crate::statistics::descriptive::{StatisticsInput, calculate_descriptive_statistics};
    
    let internal_input = StatisticsInput {
        data: input.data,
    };
    
    match calculate_descriptive_statistics(internal_input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("{{\"error\": \"{}\"}}", e)),
    }
}