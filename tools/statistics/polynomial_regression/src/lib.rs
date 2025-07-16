use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{PolynomialRegressionInput as LogicInput, PolynomialRegressionOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PolynomialRegressionInput {
    /// X values (independent variable)
    pub x: Vec<f64>,
    /// Y values (dependent variable)
    pub y: Vec<f64>,
    /// Degree of the polynomial (1-10)
    pub degree: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PolynomialRegressionOutput {
    /// Polynomial coefficients (constant, linear, quadratic, etc.)
    pub coefficients: Vec<f64>,
    /// Coefficient of determination (0 to 1)
    pub r_squared: f64,
    /// Polynomial equation in readable format
    pub equation: String,
    /// Predicted Y values for each input X
    pub predicted_values: Vec<f64>,
    /// Residuals (observed - predicted) for each data point
    pub residuals: Vec<f64>,
    /// Degree of the polynomial
    pub degree: usize,
}

#[cfg_attr(not(test), tool)]
pub fn polynomial_regression(input: PolynomialRegressionInput) -> Result<PolynomialRegressionOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        x: input.x,
        y: input.y,
        degree: input.degree,
    };
    
    // Call logic implementation
    let result = logic::calculate_polynomial_regression(logic_input)?;
    
    // Convert back to wrapper types
    Ok(PolynomialRegressionOutput {
        coefficients: result.coefficients,
        r_squared: result.r_squared,
        equation: result.equation,
        predicted_values: result.predicted_values,
        residuals: result.residuals,
        degree: result.degree,
    })
}