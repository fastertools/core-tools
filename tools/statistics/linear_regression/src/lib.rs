use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

// Re-export types from logic module
pub use logic::{RegressionInput as LogicInput, LinearRegressionOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RegressionInput {
    /// X values (independent variable)
    pub x: Vec<f64>,
    /// Y values (dependent variable)
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct LinearRegressionOutput {
    /// Slope of the regression line
    pub slope: f64,
    /// Y-intercept of the regression line
    pub intercept: f64,
    /// Coefficient of determination (0 to 1)
    pub r_squared: f64,
    /// Pearson correlation coefficient (-1 to 1)
    pub correlation_coefficient: f64,
    /// Standard error of the regression
    pub standard_error: f64,
    /// Standard error of the slope
    pub slope_std_error: f64,
    /// Standard error of the intercept
    pub intercept_std_error: f64,
    /// T-statistic for the slope
    pub t_statistic_slope: f64,
    /// T-statistic for the intercept
    pub t_statistic_intercept: f64,
    /// P-value for the slope (testing if slope = 0)
    pub p_value_slope: f64,
    /// P-value for the intercept (testing if intercept = 0)
    pub p_value_intercept: f64,
    /// Regression equation in readable format
    pub equation: String,
    /// Residuals (observed - predicted) for each data point
    pub residuals: Vec<f64>,
    /// Predicted Y values for each X
    pub predicted_values: Vec<f64>,
    /// Number of data points used
    pub sample_size: usize,
}

#[cfg_attr(not(test), tool)]
pub fn linear_regression(input: RegressionInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        x: input.x,
        y: input.y,
    };
    
    // Call logic implementation
    match logic::calculate_linear_regression(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = LinearRegressionOutput {
                slope: result.slope,
                intercept: result.intercept,
                r_squared: result.r_squared,
                correlation_coefficient: result.correlation_coefficient,
                standard_error: result.standard_error,
                slope_std_error: result.slope_std_error,
                intercept_std_error: result.intercept_std_error,
                t_statistic_slope: result.t_statistic_slope,
                t_statistic_intercept: result.t_statistic_intercept,
                p_value_slope: result.p_value_slope,
                p_value_intercept: result.p_value_intercept,
                equation: result.equation,
                residuals: result.residuals,
                predicted_values: result.predicted_values,
                sample_size: result.sample_size,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}