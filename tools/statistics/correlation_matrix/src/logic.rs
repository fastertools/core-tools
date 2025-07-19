use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSeriesInput {
    pub data: Vec<Vec<f64>>,
    pub variable_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoSeriesInput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationMatrixOutput {
    pub variables: Vec<String>,
    pub correlation_matrix: Vec<Vec<f64>>,
    pub sample_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrelationOutput {
    pub correlation_coefficient: f64,
    pub p_value: Option<f64>,
    pub sample_size: usize,
    pub interpretation: String,
}

pub fn calculate_correlation_matrix(
    input: MultiSeriesInput,
) -> Result<CorrelationMatrixOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }

    let num_variables = input.data.len();
    let sample_size = input.data[0].len();

    // Check all series have same length
    for (i, series) in input.data.iter().enumerate() {
        if series.len() != sample_size {
            return Err(format!(
                "All data series must have the same length. Series {} has length {}, expected {}",
                i,
                series.len(),
                sample_size
            ));
        }

        // Check for invalid values
        if series.iter().any(|&x| x.is_nan() || x.is_infinite()) {
            return Err(format!(
                "Series {} contains invalid values (NaN or Infinite)",
                i
            ));
        }
    }

    if sample_size < 2 {
        return Err("Need at least 2 data points for correlation".to_string());
    }

    // Create correlation matrix
    let mut correlation_matrix = vec![vec![0.0; num_variables]; num_variables];

    for i in 0..num_variables {
        for j in 0..num_variables {
            if i == j {
                correlation_matrix[i][j] = 1.0;
            } else {
                let correlation_input = TwoSeriesInput {
                    x: input.data[i].clone(),
                    y: input.data[j].clone(),
                };

                match calculate_pearson_correlation(correlation_input) {
                    Ok(result) => {
                        correlation_matrix[i][j] = result.correlation_coefficient;
                    }
                    Err(_) => {
                        correlation_matrix[i][j] = 0.0;
                    }
                }
            }
        }
    }

    // Generate variable names if not provided
    let variables = if let Some(names) = input.variable_names {
        if names.len() != num_variables {
            return Err("Number of variable names must match number of data series".to_string());
        }
        names
    } else {
        (0..num_variables)
            .map(|i| format!("Variable_{}", i + 1))
            .collect()
    };

    Ok(CorrelationMatrixOutput {
        variables,
        correlation_matrix,
        sample_size,
    })
}

fn calculate_pearson_correlation(input: TwoSeriesInput) -> Result<CorrelationOutput, String> {
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

    format!("{} {} correlation", strength, direction)
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
        p.min(1.0).max(0.0)
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
    fn test_identity_matrix_single_variable() {
        let input = MultiSeriesInput {
            data: vec![vec![1.0, 2.0, 3.0, 4.0, 5.0]],
            variable_names: Some(vec!["X".to_string()]),
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert_eq!(result.correlation_matrix.len(), 1);
        assert_eq!(result.correlation_matrix[0][0], 1.0);
        assert_eq!(result.variables, vec!["X"]);
        assert_eq!(result.sample_size, 5);
    }

    #[test]
    fn test_perfect_correlation_two_variables() {
        let input = MultiSeriesInput {
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![2.0, 4.0, 6.0, 8.0, 10.0], // Perfect linear relationship
            ],
            variable_names: Some(vec!["X".to_string(), "Y".to_string()]),
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert_eq!(result.correlation_matrix.len(), 2);
        assert!((result.correlation_matrix[0][1] - 1.0).abs() < 0.0001);
        assert!((result.correlation_matrix[1][0] - 1.0).abs() < 0.0001);
        assert_eq!(result.correlation_matrix[0][0], 1.0);
        assert_eq!(result.correlation_matrix[1][1], 1.0);
    }

    #[test]
    fn test_negative_correlation() {
        let input = MultiSeriesInput {
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![5.0, 4.0, 3.0, 2.0, 1.0], // Perfect negative correlation
            ],
            variable_names: None, // Test automatic naming
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert!((result.correlation_matrix[0][1] + 1.0).abs() < 0.0001);
        assert!((result.correlation_matrix[1][0] + 1.0).abs() < 0.0001);
        assert_eq!(result.variables, vec!["Variable_1", "Variable_2"]);
    }

    #[test]
    fn test_three_variable_matrix() {
        let input = MultiSeriesInput {
            data: vec![
                vec![1.0, 2.0, 3.0, 4.0],
                vec![2.0, 4.0, 6.0, 8.0],
                vec![1.0, 1.0, 2.0, 2.0],
            ],
            variable_names: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert_eq!(result.correlation_matrix.len(), 3);
        assert_eq!(result.correlation_matrix[0].len(), 3);

        // Check diagonal is all 1s
        for i in 0..3 {
            assert_eq!(result.correlation_matrix[i][i], 1.0);
        }

        // Check symmetry
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (result.correlation_matrix[i][j] - result.correlation_matrix[j][i]).abs()
                        < 0.0001
                );
            }
        }
    }

    #[test]
    fn test_zero_variance_handling() {
        let input = MultiSeriesInput {
            data: vec![
                vec![1.0, 1.0, 1.0, 1.0], // Constant
                vec![1.0, 2.0, 3.0, 4.0],
            ],
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert_eq!(result.correlation_matrix[0][1], 0.0);
        assert_eq!(result.correlation_matrix[1][0], 0.0);
    }

    #[test]
    fn test_empty_data_error() {
        let input = MultiSeriesInput {
            data: vec![],
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input data cannot be empty");
    }

    #[test]
    fn test_mismatched_lengths_error() {
        let input = MultiSeriesInput {
            data: vec![
                vec![1.0, 2.0, 3.0],
                vec![1.0, 2.0], // Different length
            ],
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("All data series must have the same length")
        );
    }

    #[test]
    fn test_insufficient_data_error() {
        let input = MultiSeriesInput {
            data: vec![vec![1.0]], // Only 1 data point
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Need at least 2 data points for correlation"
        );
    }

    #[test]
    fn test_nan_values_error() {
        let input = MultiSeriesInput {
            data: vec![vec![1.0, 2.0, f64::NAN], vec![1.0, 2.0, 3.0]],
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("contains invalid values"));
    }

    #[test]
    fn test_mismatched_variable_names_error() {
        let input = MultiSeriesInput {
            data: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
            variable_names: Some(vec!["X".to_string()]), // Only 1 name for 2 variables
        };
        let result = calculate_correlation_matrix(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Number of variable names must match number of data series"
        );
    }

    #[test]
    fn test_minimum_data_points() {
        let input = MultiSeriesInput {
            data: vec![vec![1.0, 2.0], vec![3.0, 5.0]],
            variable_names: None,
        };
        let result = calculate_correlation_matrix(input).unwrap();
        assert_eq!(result.sample_size, 2);
        assert!((result.correlation_matrix[0][1] - 1.0).abs() < 0.0001);
    }
}
