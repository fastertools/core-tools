use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

// Re-export types from logic module
pub use logic::{AnalyzeDistributionInput as LogicInput, AnalyzeDistributionOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeDistributionInput {
    /// Data values to analyze
    pub data: Vec<f64>,
    /// Number of histogram bins (optional, auto-calculated if not provided)
    pub num_bins: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct AnalyzeDistributionOutput {
    /// Histogram analysis of the data
    pub histogram: HistogramOutput,
    /// Normality test results
    pub normality_test: NormalityTestOutput,
    /// Distribution parameters and suggestions
    pub distribution_parameters: DistributionParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramOutput {
    /// Histogram bins with counts and frequencies
    pub bins: Vec<HistogramBin>,
    /// Total number of data points
    pub total_count: usize,
    /// Width of each bin
    pub bin_width: f64,
    /// Data range (min, max)
    pub range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramBin {
    /// Lower bound of the bin
    pub lower_bound: f64,
    /// Upper bound of the bin
    pub upper_bound: f64,
    /// Number of values in this bin
    pub count: usize,
    /// Relative frequency (count/total)
    pub frequency: f64,
    /// Density (frequency/bin_width)
    pub density: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct NormalityTestOutput {
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistributionParameters {
    /// Mean of the data
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Skewness measure
    pub skewness: f64,
    /// Kurtosis measure
    pub kurtosis: f64,
    /// Suggested distribution type based on analysis
    pub suggested_distribution: String,
}

/// Analyze distribution characteristics including histogram, normality tests, and parameter estimation
/// This tool combines histogram generation and normality testing to provide comprehensive distribution analysis
#[cfg_attr(not(test), tool)]
pub async fn analyze_distribution(input: AnalyzeDistributionInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
        num_bins: input.num_bins,
    };

    // Call logic implementation
    match logic::calculate_analyze_distribution(logic_input).await {
        Ok(result) => {
            let response = AnalyzeDistributionOutput {
                histogram: HistogramOutput {
                    bins: result
                        .histogram
                        .bins
                        .into_iter()
                        .map(|bin| HistogramBin {
                            lower_bound: bin.lower_bound,
                            upper_bound: bin.upper_bound,
                            count: bin.count,
                            frequency: bin.frequency,
                            density: bin.density,
                        })
                        .collect(),
                    total_count: result.histogram.total_count,
                    bin_width: result.histogram.bin_width,
                    range: result.histogram.range,
                },
                normality_test: NormalityTestOutput {
                    is_normal: result.normality_test.is_normal,
                    shapiro_wilk_statistic: result.normality_test.shapiro_wilk_statistic,
                    jarque_bera_statistic: result.normality_test.jarque_bera_statistic,
                    p_value: result.normality_test.p_value,
                    confidence_level: result.normality_test.confidence_level,
                    interpretation: result.normality_test.interpretation,
                },
                distribution_parameters: DistributionParameters {
                    mean: result.distribution_parameters.mean,
                    std_dev: result.distribution_parameters.std_dev,
                    skewness: result.distribution_parameters.skewness,
                    kurtosis: result.distribution_parameters.kurtosis,
                    suggested_distribution: result.distribution_parameters.suggested_distribution,
                },
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
