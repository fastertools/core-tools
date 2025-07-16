use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{descriptive_statistics_logic, StatisticsInput as LogicInput, DescriptiveStatisticsOutput};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatisticsInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
}

impl From<StatisticsInput> for LogicInput {
    fn from(input: StatisticsInput) -> Self {
        LogicInput {
            data: input.data,
        }
    }
}

#[cfg_attr(not(test), tool)]
pub fn descriptive_statistics(input: StatisticsInput) -> Result<DescriptiveStatisticsOutput, String> {
    descriptive_statistics_logic(input.into())
}