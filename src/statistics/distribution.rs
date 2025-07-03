use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct DistributionInput {
    pub data: Vec<f64>,
    pub num_bins: Option<usize>,
}

#[derive(Debug, Serialize)]
pub struct HistogramOutput {
    pub bins: Vec<HistogramBin>,
    pub total_count: usize,
    pub bin_width: f64,
    pub range: (f64, f64),
}

#[derive(Debug, Serialize)]
pub struct HistogramBin {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub count: usize,
    pub frequency: f64,
    pub density: f64,
}

#[derive(Debug, Serialize)]
pub struct NormalityTestOutput {
    pub is_normal: bool,
    pub shapiro_wilk_statistic: Option<f64>,
    pub jarque_bera_statistic: f64,
    pub p_value: f64,
    pub confidence_level: f64,
    pub interpretation: String,
}

#[derive(Debug, Serialize)]
pub struct DistributionAnalysisOutput {
    pub histogram: HistogramOutput,
    pub normality_test: NormalityTestOutput,
    pub distribution_parameters: DistributionParameters,
}

#[derive(Debug, Serialize)]
pub struct DistributionParameters {
    pub mean: f64,
    pub std_dev: f64,
    pub skewness: f64,
    pub kurtosis: f64,
    pub suggested_distribution: String,
}

pub fn generate_histogram(input: DistributionInput) -> Result<HistogramOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }
    
    // Check for invalid values
    if input.data.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }
    
    let data = &input.data;
    let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
    
    if min_val == max_val {
        return Err("All data values are the same, cannot create histogram".to_string());
    }
    
    // Determine number of bins (use Sturges' rule if not specified)
    let num_bins = input.num_bins.unwrap_or_else(|| {
        let n = data.len() as f64;
        ((n.ln() / 2.0_f64.ln()).ceil() as usize).max(1).min(50)
    });
    
    let range = max_val - min_val;
    let bin_width = range / num_bins as f64;
    
    // Create bins
    let mut bins = Vec::new();
    let mut counts = vec![0; num_bins];
    
    // Count data points in each bin
    for &value in data {
        let bin_index = if value == max_val {
            num_bins - 1 // Put max value in last bin
        } else {
            ((value - min_val) / bin_width).floor() as usize
        };
        
        if bin_index < num_bins {
            counts[bin_index] += 1;
        }
    }
    
    // Create histogram bins
    for i in 0..num_bins {
        let lower_bound = min_val + i as f64 * bin_width;
        let upper_bound = if i == num_bins - 1 {
            max_val
        } else {
            min_val + (i + 1) as f64 * bin_width
        };
        
        let count = counts[i];
        let frequency = count as f64 / data.len() as f64;
        let density = frequency / bin_width;
        
        bins.push(HistogramBin {
            lower_bound,
            upper_bound,
            count,
            frequency,
            density,
        });
    }
    
    Ok(HistogramOutput {
        bins,
        total_count: data.len(),
        bin_width,
        range: (min_val, max_val),
    })
}

pub fn test_normality(input: DistributionInput) -> Result<NormalityTestOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }
    
    if input.data.len() < 3 {
        return Err("Need at least 3 data points for normality testing".to_string());
    }
    
    // Check for invalid values
    if input.data.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }
    
    let data = &input.data;
    let n = data.len() as f64;
    
    // Calculate basic statistics
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    
    if std_dev == 0.0 {
        return Err("Standard deviation is zero, cannot test normality".to_string());
    }
    
    // Calculate skewness and kurtosis
    let skewness = data.iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>() / n;
    
    let kurtosis = data.iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>() / n;
    
    // Jarque-Bera test
    let jb_statistic = (n / 6.0) * (skewness.powi(2) + (kurtosis - 3.0).powi(2) / 4.0);
    
    // Approximate p-value for Jarque-Bera test (chi-square with 2 df)
    let p_value = chi_square_p_value(jb_statistic, 2.0);
    
    let confidence_level = 0.05;
    let is_normal = p_value > confidence_level;
    
    let interpretation = if is_normal {
        format!("Data appears to be normally distributed (p-value: {:.4} > {:.2})", p_value, confidence_level)
    } else {
        format!("Data does not appear to be normally distributed (p-value: {:.4} <= {:.2})", p_value, confidence_level)
    };
    
    // Shapiro-Wilk test would be more accurate but is complex to implement
    let shapiro_wilk_statistic = if data.len() <= 50 {
        Some(calculate_shapiro_wilk_approximation(data, mean, std_dev))
    } else {
        None
    };
    
    Ok(NormalityTestOutput {
        is_normal,
        shapiro_wilk_statistic,
        jarque_bera_statistic: jb_statistic,
        p_value,
        confidence_level,
        interpretation,
    })
}

pub fn analyze_distribution(input: DistributionInput) -> Result<DistributionAnalysisOutput, String> {
    let histogram = generate_histogram(input.clone())?;
    let normality_test = test_normality(input.clone())?;
    
    let data = &input.data;
    let n = data.len() as f64;
    
    // Calculate distribution parameters
    let mean = data.iter().sum::<f64>() / n;
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let std_dev = variance.sqrt();
    
    let skewness = if std_dev > 0.0 {
        data.iter()
            .map(|x| ((x - mean) / std_dev).powi(3))
            .sum::<f64>() / n
    } else {
        0.0
    };
    
    let kurtosis = if std_dev > 0.0 {
        data.iter()
            .map(|x| ((x - mean) / std_dev).powi(4))
            .sum::<f64>() / n - 3.0
    } else {
        0.0
    };
    
    // Suggest distribution type based on characteristics
    let suggested_distribution = suggest_distribution(skewness, kurtosis, normality_test.is_normal);
    
    let distribution_parameters = DistributionParameters {
        mean,
        std_dev,
        skewness,
        kurtosis,
        suggested_distribution,
    };
    
    Ok(DistributionAnalysisOutput {
        histogram,
        normality_test,
        distribution_parameters,
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

fn calculate_shapiro_wilk_approximation(data: &[f64], mean: f64, std_dev: f64) -> f64 {
    // This is a simplified approximation of the Shapiro-Wilk test
    // The actual test requires complex coefficients and is more involved
    
    if std_dev == 0.0 {
        return 1.0;
    }
    
    let n = data.len();
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    // Calculate sum of squared deviations
    let ss = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>();
    
    // Approximate calculation based on ordered statistics
    let mut numerator = 0.0;
    for i in 0..n {
        let expected_normal = inverse_normal_cdf((i as f64 + 0.375) / (n as f64 + 0.25));
        numerator += expected_normal * sorted_data[i];
    }
    
    numerator = numerator.powi(2);
    let denominator = ss;
    
    if denominator > 0.0 {
        (numerator / denominator).min(1.0)
    } else {
        1.0
    }
}

fn chi_square_p_value(chi_square: f64, df: f64) -> f64 {
    // Approximate p-value for chi-square distribution
    // This is a simplified approximation
    
    if chi_square <= 0.0 {
        return 1.0;
    }
    
    if df == 2.0 {
        // For df=2, chi-square follows exponential distribution
        (-chi_square / 2.0).exp()
    } else {
        // Simple approximation using normal distribution for large df
        let mean = df;
        let variance = 2.0 * df;
        let z = (chi_square - mean) / variance.sqrt();
        
        if z > 0.0 {
            2.0 * (1.0 - standard_normal_cdf(z))
        } else {
            2.0 * standard_normal_cdf(z)
        }
    }
}

fn standard_normal_cdf(x: f64) -> f64 {
    // Abramowitz and Stegun approximation
    let a1 = 0.254829592;
    let a2 = -0.284496736;
    let a3 = 1.421413741;
    let a4 = -1.453152027;
    let a5 = 1.061405429;
    let p = 0.3275911;
    
    let sign = if x >= 0.0 { 1.0 } else { -1.0 };
    let x = x.abs();
    
    let t = 1.0 / (1.0 + p * x);
    let y = 1.0 - (((((a5 * t + a4) * t) + a3) * t + a2) * t + a1) * t * (-x * x / 2.0).exp();
    
    0.5 * (1.0 + sign * y)
}

fn inverse_normal_cdf(p: f64) -> f64 {
    // Beasley-Springer-Moro approximation
    let p = p.max(1e-10).min(1.0 - 1e-10);
    
    if p == 0.5 {
        return 0.0;
    }
    
    let sign = if p > 0.5 { 1.0 } else { -1.0 };
    let p = if p > 0.5 { 1.0 - p } else { p };
    
    let t = (-2.0 * p.ln()).sqrt();
    
    let c0 = 2.515517;
    let c1 = 0.802853;
    let c2 = 0.010328;
    let d1 = 1.432788;
    let d2 = 0.189269;
    let d3 = 0.001308;
    
    let numerator = c0 + c1 * t + c2 * t * t;
    let denominator = 1.0 + d1 * t + d2 * t * t + d3 * t * t * t;
    
    sign * (t - numerator / denominator)
}