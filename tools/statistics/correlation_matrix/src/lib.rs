use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};

// Re-export types from logic module
pub use logic::{
    MultiSeriesInput as LogicMultiSeriesInput,
    CorrelationMatrixOutput as LogicCorrelationMatrixOutput,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct MultiSeriesInput {
    /// Matrix of data series (each inner vector is one variable)
    pub data: Vec<Vec<f64>>,
    /// Optional names for each variable (if not provided, will use Variable_1, Variable_2, etc.)
    pub variable_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CorrelationMatrixOutput {
    /// Names of the variables in order
    pub variables: Vec<String>,
    /// Correlation matrix (symmetric matrix where entry [i][j] is correlation between variable i and j)
    pub correlation_matrix: Vec<Vec<f64>>,
    /// Number of data points used for calculations
    pub sample_size: usize,
}

#[cfg_attr(not(test), tool)]
pub fn correlation_matrix(input: MultiSeriesInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicMultiSeriesInput {
        data: input.data,
        variable_names: input.variable_names,
    };
    
    // Call logic implementation
    match logic::calculate_correlation_matrix(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = CorrelationMatrixOutput {
                variables: result.variables,
                correlation_matrix: result.correlation_matrix,
                sample_size: result.sample_size,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}