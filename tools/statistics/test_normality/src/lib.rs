use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{TestNormalityInput as LogicInput, TestNormalityOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestNormalityInput {
    /// Data values to test for normality
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TestNormalityOutput {
    /// Whether the data appears to be normally distributed
    pub is_normal: bool,
    /// Shapiro-Wilk test statistic (if implemented)
    pub shapiro_wilk_statistic: Option<f64>,
    /// Jarque-Bera test statistic
    pub jarque_bera_statistic: f64,
    /// P-value for the normality test
    pub p_value: f64,
    /// Confidence level used (typically 0.05)
    pub confidence_level: f64,
    /// Human-readable interpretation of the test result
    pub interpretation: String,
}

#[cfg_attr(not(test), tool)]
pub fn test_normality(input: TestNormalityInput) -> Result<TestNormalityOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
    };
    
    // Call logic implementation
    let result = logic::calculate_test_normality(logic_input)?;
    
    // Convert back to wrapper types
    Ok(TestNormalityOutput {
        is_normal: result.is_normal,
        shapiro_wilk_statistic: result.shapiro_wilk_statistic,
        jarque_bera_statistic: result.jarque_bera_statistic,
        p_value: result.p_value,
        confidence_level: result.confidence_level,
        interpretation: result.interpretation,
    })
}