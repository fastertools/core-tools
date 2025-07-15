use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StatisticsInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SummaryStatisticsOutput {
    /// Number of data points
    pub count: usize,
    /// Arithmetic mean of the data
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum value
    pub min: f64,
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Maximum value
    pub max: f64,
}

#[tool]
pub fn summary_statistics(input: StatisticsInput) -> Result<SummaryStatisticsOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }
    
    let data = &input.data;
    let count = data.len();
    
    // Check for invalid values
    if data.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }
    
    // Basic calculations
    let sum: f64 = data.iter().sum();
    let mean = sum / count as f64;
    
    // Variance and standard deviation
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / count as f64;
    let std_dev = variance.sqrt();
    
    // Sort data for percentiles
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let min = sorted_data[0];
    let max = sorted_data[count - 1];
    let q1 = calculate_percentile(&sorted_data, 25.0);
    let median = calculate_percentile(&sorted_data, 50.0);
    let q3 = calculate_percentile(&sorted_data, 75.0);
    
    Ok(SummaryStatisticsOutput {
        count,
        mean,
        std_dev,
        min,
        q1,
        median,
        q3,
        max,
    })
}

fn calculate_percentile(sorted_data: &[f64], percentile: f64) -> f64 {
    let n = sorted_data.len();
    let index = (percentile / 100.0) * (n - 1) as f64;
    let lower_index = index.floor() as usize;
    let upper_index = index.ceil() as usize;
    
    if lower_index == upper_index {
        sorted_data[lower_index]
    } else {
        let weight = index - lower_index as f64;
        sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
    }
}