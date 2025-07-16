use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegressionInput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinearRegressionOutput {
    pub slope: f64,
    pub intercept: f64,
    pub r_squared: f64,
    pub correlation_coefficient: f64,
    pub standard_error: f64,
    pub slope_std_error: f64,
    pub intercept_std_error: f64,
    pub t_statistic_slope: f64,
    pub t_statistic_intercept: f64,
    pub p_value_slope: f64,
    pub p_value_intercept: f64,
    pub equation: String,
    pub residuals: Vec<f64>,
    pub predicted_values: Vec<f64>,
    pub sample_size: usize,
}

pub fn calculate_linear_regression(input: RegressionInput) -> Result<LinearRegressionOutput, String> {
    if input.x.len() != input.y.len() {
        return Err("X and Y series must have the same length".to_string());
    }
    
    if input.x.len() < 2 {
        return Err("Need at least 2 data points for regression".to_string());
    }
    
    // Check for invalid values
    if input.x.iter().any(|&x| x.is_nan() || x.is_infinite()) ||
       input.y.iter().any(|&y| y.is_nan() || y.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }
    
    let n = input.x.len() as f64;
    let x_mean = input.x.iter().sum::<f64>() / n;
    let y_mean = input.y.iter().sum::<f64>() / n;
    
    // Calculate sums for regression
    let mut sum_xy = 0.0;
    let mut sum_x_squared = 0.0;
    let mut sum_y_squared = 0.0;
    
    for i in 0..input.x.len() {
        let x_dev = input.x[i] - x_mean;
        let y_dev = input.y[i] - y_mean;
        
        sum_xy += x_dev * y_dev;
        sum_x_squared += x_dev * x_dev;
        sum_y_squared += y_dev * y_dev;
    }
    
    // Check for zero variance in X
    if sum_x_squared == 0.0 {
        return Err("X values have zero variance - cannot perform regression".to_string());
    }
    
    // Calculate slope and intercept
    let slope = sum_xy / sum_x_squared;
    let intercept = y_mean - slope * x_mean;
    
    // Calculate predicted values and residuals
    let mut predicted_values = Vec::new();
    let mut residuals = Vec::new();
    let mut residual_sum_squares = 0.0;
    
    for i in 0..input.x.len() {
        let predicted = slope * input.x[i] + intercept;
        let residual = input.y[i] - predicted;
        
        predicted_values.push(predicted);
        residuals.push(residual);
        residual_sum_squares += residual * residual;
    }
    
    // Calculate R-squared
    let total_sum_squares = sum_y_squared;
    let r_squared = if total_sum_squares == 0.0 {
        1.0 // Perfect fit when y has no variance
    } else {
        1.0 - (residual_sum_squares / total_sum_squares)
    };
    
    // Calculate correlation coefficient
    let correlation_coefficient = if sum_y_squared == 0.0 {
        0.0
    } else {
        sum_xy / (sum_x_squared * sum_y_squared).sqrt()
    };
    
    // Calculate standard errors
    let degrees_of_freedom = n - 2.0;
    let standard_error = if degrees_of_freedom > 0.0 {
        (residual_sum_squares / degrees_of_freedom).sqrt()
    } else {
        0.0
    };
    
    let slope_std_error = if sum_x_squared > 0.0 {
        standard_error / sum_x_squared.sqrt()
    } else {
        0.0
    };
    
    let intercept_std_error = if sum_x_squared > 0.0 {
        standard_error * ((1.0 / n) + (x_mean * x_mean / sum_x_squared)).sqrt()
    } else {
        0.0
    };
    
    // Calculate t-statistics
    let t_statistic_slope = if slope_std_error > 0.0 {
        slope / slope_std_error
    } else {
        0.0
    };
    
    let t_statistic_intercept = if intercept_std_error > 0.0 {
        intercept / intercept_std_error
    } else {
        0.0
    };
    
    // Calculate p-values (approximate)
    let p_value_slope = if degrees_of_freedom > 0.0 {
        2.0 * (1.0 - t_distribution_cdf(t_statistic_slope.abs(), degrees_of_freedom))
    } else {
        1.0
    };
    
    let p_value_intercept = if degrees_of_freedom > 0.0 {
        2.0 * (1.0 - t_distribution_cdf(t_statistic_intercept.abs(), degrees_of_freedom))
    } else {
        1.0
    };
    
    // Create equation string
    let equation = if intercept >= 0.0 {
        format!("y = {:.6}x + {:.6}", slope, intercept)
    } else {
        format!("y = {:.6}x - {:.6}", slope, intercept.abs())
    };
    
    Ok(LinearRegressionOutput {
        slope,
        intercept,
        r_squared,
        correlation_coefficient,
        standard_error,
        slope_std_error,
        intercept_std_error,
        t_statistic_slope,
        t_statistic_intercept,
        p_value_slope,
        p_value_intercept,
        equation,
        residuals,
        predicted_values,
        sample_size: input.x.len(),
    })
}

fn t_distribution_cdf(t: f64, df: f64) -> f64 {
    // Approximate t-distribution CDF
    if df <= 0.0 {
        return 0.5;
    }
    
    // For large df, t-distribution approaches normal distribution
    if df > 30.0 {
        return standard_normal_cdf(t);
    }
    
    // Simple approximation for small df
    let x = t / (df + t * t).sqrt();
    0.5 + x * (0.5 - x * x / 12.0)
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
    fn test_perfect_linear_relationship() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![2.0, 4.0, 6.0, 8.0, 10.0], // y = 2x
        };
        let result = calculate_linear_regression(input).unwrap();
        assert!((result.slope - 2.0).abs() < 0.0001);
        assert!(result.intercept.abs() < 0.0001);
        assert!((result.r_squared - 1.0).abs() < 0.0001);
        assert_eq!(result.sample_size, 5);
    }

    #[test]
    fn test_linear_with_intercept() {
        let input = RegressionInput {
            x: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            y: vec![1.0, 3.0, 5.0, 7.0, 9.0], // y = 2x + 1
        };
        let result = calculate_linear_regression(input).unwrap();
        assert!((result.slope - 2.0).abs() < 0.0001);
        assert!((result.intercept - 1.0).abs() < 0.0001);
        assert!((result.r_squared - 1.0).abs() < 0.0001);
        assert!(result.equation.contains("y = 2.000000x + 1.000000"));
    }

    #[test]
    fn test_negative_slope() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![10.0, 8.0, 6.0, 4.0, 2.0], // y = -2x + 12
        };
        let result = calculate_linear_regression(input).unwrap();
        assert!((result.slope + 2.0).abs() < 0.0001);
        assert!((result.intercept - 12.0).abs() < 0.0001);
        assert!((result.r_squared - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_real_world_data() {
        // Height vs weight data (simplified)
        let input = RegressionInput {
            x: vec![160.0, 165.0, 170.0, 175.0, 180.0],
            y: vec![60.0, 63.0, 66.0, 70.0, 74.0],
        };
        let result = calculate_linear_regression(input).unwrap();
        assert!(result.slope > 0.0); // Positive correlation
        assert!(result.r_squared > 0.95); // Strong linear relationship
        assert!(result.t_statistic_slope > 2.0); // Large t-statistic
        assert_eq!(result.sample_size, 5);
    }

    #[test]
    fn test_residuals_sum_to_zero() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![2.1, 3.9, 6.1, 7.8, 10.2],
        };
        let result = calculate_linear_regression(input).unwrap();
        let residual_sum: f64 = result.residuals.iter().sum();
        assert!(residual_sum.abs() < 0.0001);
    }

    #[test]
    fn test_minimum_data_points() {
        let input = RegressionInput {
            x: vec![1.0, 2.0],
            y: vec![2.0, 4.0],
        };
        let result = calculate_linear_regression(input).unwrap();
        assert_eq!(result.sample_size, 2);
        assert!((result.slope - 2.0).abs() < 0.0001);
    }

    #[test]
    fn test_horizontal_line() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![5.0, 5.0, 5.0, 5.0, 5.0], // Constant y
        };
        let result = calculate_linear_regression(input).unwrap();
        assert!(result.slope.abs() < 0.0001);
        assert!((result.intercept - 5.0).abs() < 0.0001);
        assert_eq!(result.correlation_coefficient, 0.0);
    }

    #[test]
    fn test_zero_variance_x_error() {
        let input = RegressionInput {
            x: vec![1.0, 1.0, 1.0, 1.0],
            y: vec![1.0, 2.0, 3.0, 4.0],
        };
        let result = calculate_linear_regression(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "X values have zero variance - cannot perform regression");
    }

    #[test]
    fn test_different_lengths_error() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 2.0],
        };
        let result = calculate_linear_regression(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "X and Y series must have the same length");
    }

    #[test]
    fn test_insufficient_data_error() {
        let input = RegressionInput {
            x: vec![1.0],
            y: vec![2.0],
        };
        let result = calculate_linear_regression(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Need at least 2 data points for regression");
    }

    #[test]
    fn test_nan_values_error() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, f64::NAN],
            y: vec![2.0, 4.0, 6.0],
        };
        let result = calculate_linear_regression(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input data contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_predicted_values_match_equation() {
        let input = RegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![3.0, 5.0, 7.0], // y = 2x + 1
        };
        let result = calculate_linear_regression(input.clone()).unwrap();
        
        for i in 0..result.predicted_values.len() {
            let expected = result.slope * input.x[i] + result.intercept;
            assert!((result.predicted_values[i] - expected).abs() < 0.0001);
        }
    }
}