use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

use ftl_sdk::{ToolResponse, tool};

// Re-export types from logic module
pub use logic::{CorrelationOutput as LogicOutput, TwoSeriesInput as LogicInput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoSeriesInput {
    /// X values (first data series)
    pub x: Vec<f64>,
    /// Y values (second data series)
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorrelationOutput {
    /// Spearman rank correlation coefficient (-1 to 1)
    pub correlation_coefficient: f64,
    /// Statistical significance p-value (if calculable)
    pub p_value: Option<f64>,
    /// Number of data points analyzed
    pub sample_size: usize,
    /// Human-readable interpretation of correlation strength
    pub interpretation: String,
}

#[cfg_attr(not(test), tool)]
pub fn spearman_correlation(input: TwoSeriesInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        x: input.x,
        y: input.y,
    };

    // Call logic implementation
    match logic::calculate_spearman_correlation(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = CorrelationOutput {
                correlation_coefficient: result.correlation_coefficient,
                p_value: result.p_value,
                sample_size: result.sample_size,
                interpretation: result.interpretation,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
