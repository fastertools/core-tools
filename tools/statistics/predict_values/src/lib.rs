use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};

// Re-export types from logic module
pub use logic::{PredictionInput as LogicInput, PredictionOutput as LogicOutput, RegressionPrediction as LogicPrediction};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PredictionInput {
    /// Slope of the regression line
    pub slope: f64,
    /// Y-intercept of the regression line
    pub intercept: f64,
    /// X values to predict Y values for
    pub x_values: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PredictionOutput {
    /// Predictions for each X value
    pub predictions: Vec<RegressionPrediction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RegressionPrediction {
    /// X value
    pub x: f64,
    /// Predicted Y value
    pub y_predicted: f64,
    /// Confidence interval (currently not implemented)
    pub confidence_interval: Option<(f64, f64)>,
}

#[cfg_attr(not(test), tool)]
pub fn predict_values(input: PredictionInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        slope: input.slope,
        intercept: input.intercept,
        x_values: input.x_values,
    };
    
    // Call logic implementation
    match logic::predict_values(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = PredictionOutput {
                predictions: result.predictions.into_iter().map(|p| RegressionPrediction {
                    x: p.x,
                    y_predicted: p.y_predicted,
                    confidence_interval: p.confidence_interval,
                }).collect(),
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}