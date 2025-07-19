use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{StatisticsInput as LogicInput, summary_statistics_logic};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatisticsInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
}

impl From<StatisticsInput> for LogicInput {
    fn from(input: StatisticsInput) -> Self {
        LogicInput { data: input.data }
    }
}

#[cfg_attr(not(test), tool)]
pub fn summary_statistics(input: StatisticsInput) -> ToolResponse {
    match summary_statistics_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
