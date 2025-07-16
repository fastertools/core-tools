use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{TwoSeriesInput as LogicInput, CorrelationOutput as LogicOutput};

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
pub fn spearman_correlation(input: TwoSeriesInput) -> Result<CorrelationOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        x: input.x,
        y: input.y,
    };
    
    // Call logic implementation
    let result = logic::calculate_spearman_correlation(logic_input)?;
    
    // Convert back to wrapper types
    Ok(CorrelationOutput {
        correlation_coefficient: result.correlation_coefficient,
        p_value: result.p_value,
        sample_size: result.sample_size,
        interpretation: result.interpretation,
    })
}