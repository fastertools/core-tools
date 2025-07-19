use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{
    HistogramBin as LogicBin, HistogramInput as LogicInput, HistogramOutput as LogicOutput,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
    /// Number of bins for the histogram (optional, uses Sturges' rule if not specified)
    pub num_bins: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramOutput {
    /// Array of histogram bins with counts and statistics
    pub bins: Vec<HistogramBin>,
    /// Total number of data points
    pub total_count: usize,
    /// Width of each bin
    pub bin_width: f64,
    /// Range of the data (min, max)
    pub range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct HistogramBin {
    /// Lower bound of the bin (inclusive)
    pub lower_bound: f64,
    /// Upper bound of the bin (exclusive, except for last bin)
    pub upper_bound: f64,
    /// Number of values in this bin
    pub count: usize,
    /// Relative frequency (count / total_count)
    pub frequency: f64,
    /// Probability density (frequency / bin_width)
    pub density: f64,
}

#[cfg_attr(not(test), tool)]
pub fn histogram(input: HistogramInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        data: input.data,
        num_bins: input.num_bins,
    };

    // Call logic implementation
    match logic::generate_histogram(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = HistogramOutput {
                bins: result
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
                total_count: result.total_count,
                bin_width: result.bin_width,
                range: result.range,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
