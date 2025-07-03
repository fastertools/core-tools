use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegressionInput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

#[derive(Debug, Serialize)]
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

#[derive(Debug, Deserialize)]
pub struct PredictionInput {
    pub slope: f64,
    pub intercept: f64,
    pub x_values: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct PredictionOutput {
    pub predictions: Vec<RegressionPrediction>,
}

#[derive(Debug, Serialize)]
pub struct RegressionPrediction {
    pub x: f64,
    pub y_predicted: f64,
    pub confidence_interval: Option<(f64, f64)>,
}

#[derive(Debug, Deserialize)]
pub struct PolynomialRegressionInput {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub degree: usize,
}

#[derive(Debug, Serialize)]
pub struct PolynomialRegressionOutput {
    pub coefficients: Vec<f64>,
    pub r_squared: f64,
    pub equation: String,
    pub predicted_values: Vec<f64>,
    pub residuals: Vec<f64>,
    pub degree: usize,
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

pub fn predict_values(input: PredictionInput) -> Result<PredictionOutput, String> {
    if input.x_values.is_empty() {
        return Err("X values for prediction cannot be empty".to_string());
    }
    
    // Check for invalid values
    if input.x_values.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("X values contain invalid values (NaN or Infinite)".to_string());
    }
    
    if input.slope.is_nan() || input.slope.is_infinite() ||
       input.intercept.is_nan() || input.intercept.is_infinite() {
        return Err("Slope or intercept contains invalid values".to_string());
    }
    
    let predictions = input.x_values.iter().map(|&x| {
        let y_predicted = input.slope * x + input.intercept;
        RegressionPrediction {
            x,
            y_predicted,
            confidence_interval: None, // Would require additional statistics to calculate
        }
    }).collect();
    
    Ok(PredictionOutput { predictions })
}

pub fn calculate_polynomial_regression(input: PolynomialRegressionInput) -> Result<PolynomialRegressionOutput, String> {
    if input.x.len() != input.y.len() {
        return Err("X and Y series must have the same length".to_string());
    }
    
    if input.x.len() < input.degree + 1 {
        return Err(format!("Need at least {} data points for degree {} polynomial", input.degree + 1, input.degree));
    }
    
    if input.degree == 0 {
        return Err("Polynomial degree must be at least 1".to_string());
    }
    
    if input.degree > 10 {
        return Err("Polynomial degree cannot exceed 10 (numerical stability)".to_string());
    }
    
    // Check for invalid values
    if input.x.iter().any(|&x| x.is_nan() || x.is_infinite()) ||
       input.y.iter().any(|&y| y.is_nan() || y.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }
    
    let n = input.x.len();
    let degree = input.degree;
    
    // Create design matrix (Vandermonde matrix)
    let mut design_matrix = vec![vec![0.0; degree + 1]; n];
    for i in 0..n {
        for j in 0..=degree {
            design_matrix[i][j] = input.x[i].powi(j as i32);
        }
    }
    
    // Solve normal equations: (X^T X) β = X^T y
    let mut xtx = vec![vec![0.0; degree + 1]; degree + 1];
    let mut xty = vec![0.0; degree + 1];
    
    // Calculate X^T X
    for i in 0..=degree {
        for j in 0..=degree {
            for k in 0..n {
                xtx[i][j] += design_matrix[k][i] * design_matrix[k][j];
            }
        }
    }
    
    // Calculate X^T y
    for i in 0..=degree {
        for k in 0..n {
            xty[i] += design_matrix[k][i] * input.y[k];
        }
    }
    
    // Solve linear system using Gaussian elimination
    let coefficients = solve_linear_system(xtx, xty)?;
    
    // Calculate predicted values and residuals
    let mut predicted_values = Vec::new();
    let mut residuals = Vec::new();
    let mut residual_sum_squares = 0.0;
    
    for i in 0..n {
        let mut predicted = 0.0;
        for j in 0..=degree {
            predicted += coefficients[j] * input.x[i].powi(j as i32);
        }
        
        let residual = input.y[i] - predicted;
        predicted_values.push(predicted);
        residuals.push(residual);
        residual_sum_squares += residual * residual;
    }
    
    // Calculate R-squared
    let y_mean = input.y.iter().sum::<f64>() / n as f64;
    let total_sum_squares = input.y.iter().map(|&y| (y - y_mean).powi(2)).sum::<f64>();
    
    let r_squared = if total_sum_squares == 0.0 {
        1.0
    } else {
        1.0 - (residual_sum_squares / total_sum_squares)
    };
    
    // Create equation string
    let mut equation = String::new();
    for (i, &coeff) in coefficients.iter().enumerate() {
        if i == 0 {
            equation.push_str(&format!("{:.6}", coeff));
        } else {
            let sign = if coeff >= 0.0 { " + " } else { " - " };
            equation.push_str(sign);
            if i == 1 {
                equation.push_str(&format!("{:.6}x", coeff.abs()));
            } else {
                equation.push_str(&format!("{:.6}x^{}", coeff.abs(), i));
            }
        }
    }
    equation = format!("y = {}", equation);
    
    Ok(PolynomialRegressionOutput {
        coefficients,
        r_squared,
        equation,
        predicted_values,
        residuals,
        degree,
    })
}

fn solve_linear_system(mut matrix: Vec<Vec<f64>>, mut vector: Vec<f64>) -> Result<Vec<f64>, String> {
    let n = matrix.len();
    
    // Forward elimination
    for i in 0..n {
        // Find pivot
        let mut max_row = i;
        for k in i + 1..n {
            if matrix[k][i].abs() > matrix[max_row][i].abs() {
                max_row = k;
            }
        }
        
        // Swap rows
        if max_row != i {
            matrix.swap(i, max_row);
            vector.swap(i, max_row);
        }
        
        // Check for singular matrix
        if matrix[i][i].abs() < 1e-10 {
            return Err("Matrix is singular - cannot solve linear system".to_string());
        }
        
        // Eliminate column
        for k in i + 1..n {
            let factor = matrix[k][i] / matrix[i][i];
            for j in i..n {
                matrix[k][j] -= factor * matrix[i][j];
            }
            vector[k] -= factor * vector[i];
        }
    }
    
    // Back substitution
    let mut solution = vec![0.0; n];
    for i in (0..n).rev() {
        solution[i] = vector[i];
        for j in i + 1..n {
            solution[i] -= matrix[i][j] * solution[j];
        }
        solution[i] /= matrix[i][i];
    }
    
    Ok(solution)
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