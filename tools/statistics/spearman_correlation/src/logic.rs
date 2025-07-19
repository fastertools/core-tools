use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoSeriesInput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationOutput {
    pub correlation_coefficient: f64,
    pub p_value: Option<f64>,
    pub sample_size: usize,
    pub interpretation: String,
}

pub fn calculate_spearman_correlation(input: TwoSeriesInput) -> Result<CorrelationOutput, String> {
    if input.x.len() != input.y.len() {
        return Err("X and Y series must have the same length".to_string());
    }

    if input.x.len() < 2 {
        return Err("Need at least 2 data points for correlation".to_string());
    }

    // Check for invalid values
    if input.x.iter().any(|&x| x.is_nan() || x.is_infinite())
        || input.y.iter().any(|&y| y.is_nan() || y.is_infinite())
    {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }

    // Convert to ranks
    let x_ranks = calculate_ranks(&input.x);
    let y_ranks = calculate_ranks(&input.y);

    // Calculate Pearson correlation on ranks
    let rank_input = TwoSeriesInput {
        x: x_ranks,
        y: y_ranks,
    };

    let mut result = calculate_pearson_correlation(rank_input)?;

    // Only override interpretation if it's not a zero variance case
    if !result.interpretation.contains("zero variance") {
        result.interpretation = format!(
            "Spearman rank correlation: {}",
            interpret_correlation(result.correlation_coefficient)
        );
    }

    Ok(result)
}

fn calculate_pearson_correlation(input: TwoSeriesInput) -> Result<CorrelationOutput, String> {
    let n = input.x.len() as f64;
    let x_mean = input.x.iter().sum::<f64>() / n;
    let y_mean = input.y.iter().sum::<f64>() / n;

    // Calculate covariance and standard deviations
    let mut covariance = 0.0;
    let mut x_variance = 0.0;
    let mut y_variance = 0.0;

    for i in 0..input.x.len() {
        let x_diff = input.x[i] - x_mean;
        let y_diff = input.y[i] - y_mean;

        covariance += x_diff * y_diff;
        x_variance += x_diff * x_diff;
        y_variance += y_diff * y_diff;
    }

    let x_std = (x_variance / n).sqrt();
    let y_std = (y_variance / n).sqrt();

    // Handle case where one variable has zero variance
    if x_std == 0.0 || y_std == 0.0 {
        return Ok(CorrelationOutput {
            correlation_coefficient: 0.0,
            p_value: None,
            sample_size: input.x.len(),
            interpretation: "No correlation (zero variance in one variable)".to_string(),
        });
    }

    let correlation = covariance / (n * x_std * y_std);

    // Calculate approximate p-value for testing H0: r = 0
    let p_value = if input.x.len() >= 3 {
        let t_stat = correlation * ((n - 2.0) / (1.0 - correlation * correlation)).sqrt();
        Some(calculate_t_test_p_value(t_stat.abs(), n - 2.0))
    } else {
        None
    };

    let interpretation = interpret_correlation(correlation);

    Ok(CorrelationOutput {
        correlation_coefficient: correlation,
        p_value,
        sample_size: input.x.len(),
        interpretation,
    })
}

fn calculate_ranks(data: &[f64]) -> Vec<f64> {
    let mut indexed_data: Vec<(f64, usize)> =
        data.iter().enumerate().map(|(i, &val)| (val, i)).collect();
    indexed_data.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut ranks = vec![0.0; data.len()];
    let mut i = 0;

    while i < indexed_data.len() {
        let mut j = i;
        // Find all tied values
        while j < indexed_data.len() && indexed_data[j].0 == indexed_data[i].0 {
            j += 1;
        }

        // Assign average rank to tied values
        let avg_rank = (i + j + 1) as f64 / 2.0;
        for k in i..j {
            ranks[indexed_data[k].1] = avg_rank;
        }

        i = j;
    }

    ranks
}

fn interpret_correlation(r: f64) -> String {
    let abs_r = r.abs();
    let strength = if abs_r >= 0.9 {
        "very strong"
    } else if abs_r >= 0.7 {
        "strong"
    } else if abs_r >= 0.5 {
        "moderate"
    } else if abs_r >= 0.3 {
        "weak"
    } else if abs_r >= 0.1 {
        "very weak"
    } else {
        "negligible"
    };

    let direction = if r > 0.0 {
        "positive"
    } else if r < 0.0 {
        "negative"
    } else {
        "no"
    };

    format!("{strength} {direction} correlation")
}

fn calculate_t_test_p_value(t_stat: f64, df: f64) -> f64 {
    // Approximate p-value calculation using t-distribution
    // This is a simplified approximation
    if df <= 0.0 {
        return 1.0;
    }

    // For large df, t-distribution approaches normal distribution
    if df > 30.0 {
        // Use normal approximation
        let z = t_stat;
        2.0 * (1.0 - standard_normal_cdf(z))
    } else {
        // Simple approximation for small df
        let p = 2.0 * (1.0 - (1.0 / (1.0 + (t_stat * t_stat) / df)).powf(df / 2.0));
        p.clamp(0.0, 1.0)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_monotonic_correlation() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![1.0, 4.0, 9.0, 16.0, 25.0], // Perfect quadratic relationship
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert!((result.correlation_coefficient - 1.0).abs() < 0.0001);
        assert_eq!(result.sample_size, 5);
    }

    #[test]
    fn test_perfect_negative_monotonic() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![25.0, 16.0, 9.0, 4.0, 1.0], // Perfect negative monotonic
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert!((result.correlation_coefficient + 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_tied_ranks() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 2.0, 4.0, 5.0],
            y: vec![1.0, 3.0, 3.0, 5.0, 6.0],
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert!(result.correlation_coefficient > 0.9);
        assert_eq!(result.sample_size, 5);
    }

    #[test]
    fn test_rank_calculation() {
        let data = vec![3.0, 1.0, 4.0, 1.0, 5.0];
        let ranks = calculate_ranks(&data);
        // Expected ranks: [3.0, 1.5, 4.0, 1.5, 5.0]
        assert_eq!(ranks[0], 3.0);
        assert_eq!(ranks[1], 1.5); // Tied for rank 1
        assert_eq!(ranks[2], 4.0);
        assert_eq!(ranks[3], 1.5); // Tied for rank 1
        assert_eq!(ranks[4], 5.0);
    }

    #[test]
    fn test_non_linear_relationship() {
        // Spearman should detect monotonic relationship even if not linear
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0],
            y: vec![1.0, 8.0, 27.0, 64.0, 125.0, 216.0], // Cubic relationship
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert!((result.correlation_coefficient - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_no_correlation() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![2.0, 2.0, 2.0, 2.0, 2.0], // Constant values
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert_eq!(result.correlation_coefficient, 0.0);
        assert_eq!(
            result.interpretation,
            "No correlation (zero variance in one variable)"
        );
    }

    #[test]
    fn test_different_lengths_error() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 2.0],
        };
        let result = calculate_spearman_correlation(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "X and Y series must have the same length"
        );
    }

    #[test]
    fn test_insufficient_data_error() {
        let input = TwoSeriesInput {
            x: vec![1.0],
            y: vec![2.0],
        };
        let result = calculate_spearman_correlation(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Need at least 2 data points for correlation"
        );
    }

    #[test]
    fn test_nan_values_error() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0, f64::NAN],
            y: vec![2.0, 4.0, 6.0],
        };
        let result = calculate_spearman_correlation(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input data contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_minimum_data_points() {
        let input = TwoSeriesInput {
            x: vec![1.0, 2.0],
            y: vec![3.0, 4.0],
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert!((result.correlation_coefficient - 1.0).abs() < 0.0001);
        assert_eq!(result.sample_size, 2);
        assert!(result.p_value.is_none()); // Not enough data for p-value
    }

    #[test]
    fn test_all_same_rank_ties() {
        let input = TwoSeriesInput {
            x: vec![1.0, 1.0, 1.0, 1.0],
            y: vec![2.0, 3.0, 4.0, 5.0],
        };
        let result = calculate_spearman_correlation(input).unwrap();
        assert_eq!(result.correlation_coefficient, 0.0);
    }
}
