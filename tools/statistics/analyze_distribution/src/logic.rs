use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct AnalyzeDistributionInput {
    pub data: Vec<f64>,
    pub num_bins: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeDistributionOutput {
    pub histogram: HistogramOutput,
    pub normality_test: NormalityTestOutput,
    pub distribution_parameters: DistributionParameters,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramOutput {
    pub bins: Vec<HistogramBin>,
    pub total_count: usize,
    pub bin_width: f64,
    pub range: (f64, f64),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistogramBin {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub count: usize,
    pub frequency: f64,
    pub density: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NormalityTestOutput {
    pub is_normal: bool,
    pub shapiro_wilk_statistic: Option<f64>,
    pub jarque_bera_statistic: f64,
    pub p_value: f64,
    pub confidence_level: f64,
    pub interpretation: String,
}

#[derive(Debug, Serialize)]
pub struct DistributionParameters {
    pub mean: f64,
    pub std_dev: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub suggested_distribution: String,
}

// Helper structs for calling other tools
#[derive(Serialize)]
struct HistogramInput {
    data: Vec<f64>,
    num_bins: Option<usize>,
}

#[derive(Serialize)]
struct TestNormalityInput {
    data: Vec<f64>,
}

#[derive(Deserialize)]
struct ToolResponseWrapper<T> {
    #[serde(rename = "Ok")]
    ok: T,
}

pub async fn calculate_analyze_distribution(
    input: AnalyzeDistributionInput,
) -> Result<AnalyzeDistributionOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }

    if input.data.len() < 3 {
        return Err("Need at least 3 data points for distribution analysis".to_string());
    }

    // Check for invalid values
    if input.data.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }

    // Step 1: Call histogram tool
    let histogram = call_histogram_tool(&input.data, input.num_bins).await?;

    // Step 2: Call test_normality tool
    let normality_test = call_test_normality_tool(&input.data).await?;

    // Step 3: Calculate distribution parameters locally
    let distribution_parameters =
        calculate_distribution_parameters(&input.data, normality_test.is_normal)?;

    Ok(AnalyzeDistributionOutput {
        histogram,
        normality_test,
        distribution_parameters,
    })
}

async fn call_histogram_tool(
    data: &[f64],
    num_bins: Option<usize>,
) -> Result<HistogramOutput, String> {
    use spin_sdk::http::{Method, Request};

    let histogram_input = HistogramInput {
        data: data.to_vec(),
        num_bins,
    };

    let request_body = serde_json::to_string(&histogram_input)
        .map_err(|e| format!("Failed to serialize histogram input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://histogram.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Error calling histogram tool: {e:?}"))?;

    let body_bytes = response.into_body();
    let body =
        String::from_utf8(body_bytes).map_err(|e| format!("Failed to parse response body: {e}"))?;

    let wrapper: ToolResponseWrapper<HistogramOutput> =
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse tool response: {e}"))?;

    let histogram_result = wrapper.ok;

    Ok(histogram_result)
}

async fn call_test_normality_tool(data: &[f64]) -> Result<NormalityTestOutput, String> {
    use spin_sdk::http::{Method, Request};

    let test_normality_input = TestNormalityInput {
        data: data.to_vec(),
    };

    let request_body = serde_json::to_string(&test_normality_input)
        .map_err(|e| format!("Failed to serialize test_normality input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://test-normality.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Error calling test_normality tool: {e:?}"))?;

    let body_bytes = response.into_body();
    let body =
        String::from_utf8(body_bytes).map_err(|e| format!("Failed to parse response body: {e}"))?;

    let wrapper: ToolResponseWrapper<NormalityTestOutput> =
        serde_json::from_str(&body).map_err(|e| format!("Failed to parse tool response: {e}"))?;

    let normality_result = wrapper.ok;

    Ok(normality_result)
}

fn calculate_distribution_parameters(
    data: &[f64],
    is_normal: bool,
) -> Result<DistributionParameters, String> {
    let n = data.len() as f64;

    // Calculate basic statistics
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();

    if std_dev == 0.0 {
        return Err(
            "Standard deviation is zero, cannot calculate distribution parameters".to_string(),
        );
    }

    // Calculate skewness and kurtosis
    let skewness = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>()
        / n;

    let kurtosis = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>()
        / n
        - 3.0; // Excess kurtosis

    // Suggest distribution type based on characteristics
    let suggested_distribution = suggest_distribution(skewness, kurtosis, is_normal);

    Ok(DistributionParameters {
        mean,
        std_dev,
        skewness,
        kurtosis,
        suggested_distribution,
    })
}

fn suggest_distribution(skewness: f64, kurtosis: f64, is_normal: bool) -> String {
    if is_normal {
        "Normal Distribution".to_string()
    } else if skewness.abs() < 0.5 && kurtosis.abs() < 0.5 {
        "Approximately Normal Distribution".to_string()
    } else if skewness > 1.0 {
        "Right-skewed Distribution (consider Log-normal, Exponential, or Gamma)".to_string()
    } else if skewness < -1.0 {
        "Left-skewed Distribution (consider Beta or transformed distributions)".to_string()
    } else if kurtosis > 3.0 {
        "Heavy-tailed Distribution (consider t-distribution or Laplace)".to_string()
    } else if kurtosis < -1.0 {
        "Light-tailed Distribution (consider Uniform or truncated distributions)".to_string()
    } else {
        "Non-normal Distribution (consider non-parametric approaches)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggest_distribution() {
        // Test normal distribution suggestion
        assert_eq!(suggest_distribution(0.0, 0.0, true), "Normal Distribution");

        // Test approximately normal
        assert_eq!(
            suggest_distribution(0.3, 0.2, false),
            "Approximately Normal Distribution"
        );

        // Test right-skewed
        assert_eq!(
            suggest_distribution(1.5, 0.0, false),
            "Right-skewed Distribution (consider Log-normal, Exponential, or Gamma)"
        );

        // Test left-skewed
        assert_eq!(
            suggest_distribution(-1.5, 0.0, false),
            "Left-skewed Distribution (consider Beta or transformed distributions)"
        );

        // Test heavy-tailed
        assert_eq!(
            suggest_distribution(0.0, 4.0, false),
            "Heavy-tailed Distribution (consider t-distribution or Laplace)"
        );

        // Test light-tailed
        assert_eq!(
            suggest_distribution(0.0, -1.5, false),
            "Light-tailed Distribution (consider Uniform or truncated distributions)"
        );

        // Test non-normal
        assert_eq!(
            suggest_distribution(0.8, 1.0, false),
            "Non-normal Distribution (consider non-parametric approaches)"
        );
    }

    #[test]
    fn test_calculate_distribution_parameters() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let result = calculate_distribution_parameters(&data, false).unwrap();

        assert_eq!(result.mean, 3.0);
        assert!((result.std_dev - std::f64::consts::SQRT_2).abs() < 1e-10);
        assert!(result.skewness.abs() < 1e-10); // Should be close to 0 for symmetric data
        assert!(!result.suggested_distribution.is_empty());
    }

    #[test]
    fn test_input_validation() {
        // Test that single element data is caught (std_dev will be 0)
        let result = calculate_distribution_parameters(&[1.0], false);
        assert!(result.is_err()); // Single element has std_dev = 0

        // Test that empty data would cause issues (though it's caught earlier)
        // Empty slice would have mean = 0/0 = NaN, but we catch it before this function
        // Let's just test a simple case with different values
        let result = calculate_distribution_parameters(&[1.0, 2.0, 3.0], false);
        assert!(result.is_ok()); // This should work fine
    }

    #[test]
    fn test_zero_variance() {
        let data = vec![5.0, 5.0, 5.0, 5.0, 5.0]; // All identical
        let result = calculate_distribution_parameters(&data, false);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Standard deviation is zero"));
    }

    #[test]
    fn test_skewed_data() {
        // Right-skewed data
        let data = vec![1.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0];
        let result = calculate_distribution_parameters(&data, false).unwrap();

        assert!(result.skewness > 0.0); // Should be positive for right-skewed
        assert!(
            result.suggested_distribution.contains("Right-skewed")
                || result.suggested_distribution.contains("Non-normal")
        );
    }

    #[test]
    fn test_heavy_tailed_data() {
        // Data with outliers (heavy tails)
        let data = vec![-10.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 10.0];
        let result = calculate_distribution_parameters(&data, false).unwrap();

        // Should detect high kurtosis
        assert!(result.kurtosis > 0.0);
    }
}
