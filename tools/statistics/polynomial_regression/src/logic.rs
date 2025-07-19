use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
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

pub fn calculate_polynomial_regression(
    input: PolynomialRegressionInput,
) -> Result<PolynomialRegressionOutput, String> {
    if input.x.len() != input.y.len() {
        return Err("X and Y series must have the same length".to_string());
    }

    if input.x.len() < input.degree + 1 {
        return Err(format!(
            "Need at least {} data points for degree {} polynomial",
            input.degree + 1,
            input.degree
        ));
    }

    if input.degree == 0 {
        return Err("Polynomial degree must be at least 1".to_string());
    }

    if input.degree > 10 {
        return Err("Polynomial degree cannot exceed 10 (numerical stability)".to_string());
    }

    // Check for invalid values
    if input.x.iter().any(|&x| x.is_nan() || x.is_infinite())
        || input.y.iter().any(|&y| y.is_nan() || y.is_infinite())
    {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }

    let n = input.x.len();
    let degree = input.degree;

    // Create design matrix (Vandermonde matrix)
    let mut design_matrix = vec![vec![0.0; degree + 1]; n];
    for (i, row) in design_matrix.iter_mut().enumerate().take(n) {
        for (j, item) in row.iter_mut().enumerate().take(degree + 1) {
            *item = input.x[i].powi(j as i32);
        }
    }

    // Solve normal equations: (X^T X) β = X^T y
    let mut xtx = vec![vec![0.0; degree + 1]; degree + 1];
    let mut xty = vec![0.0; degree + 1];

    // Calculate X^T X
    for (i, row) in xtx.iter_mut().enumerate().take(degree + 1) {
        for j in 0..=degree {
            for design_row in design_matrix.iter().take(n) {
                row[j] += design_row[i] * design_row[j];
            }
        }
    }

    // Calculate X^T y
    for (i, xty_item) in xty.iter_mut().enumerate().take(degree + 1) {
        for (design_row, &y_val) in design_matrix.iter().zip(input.y.iter()).take(n) {
            *xty_item += design_row[i] * y_val;
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
        for (j, &coeff) in coefficients.iter().enumerate().take(degree + 1) {
            predicted += coeff * input.x[i].powi(j as i32);
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
            equation.push_str(&format!("{coeff:.6}"));
        } else {
            let sign = if coeff >= 0.0 { " + " } else { " - " };
            equation.push_str(sign);
            if i == 1 {
                equation.push_str(&format!("{:.6}x", coeff.abs()));
            } else {
                equation.push_str(&format!("{coeff:.6}x^{i}", coeff = coeff.abs()));
            }
        }
    }
    equation = format!("y = {equation}");

    Ok(PolynomialRegressionOutput {
        coefficients,
        r_squared,
        equation,
        predicted_values,
        residuals,
        degree,
    })
}

fn solve_linear_system(
    mut matrix: Vec<Vec<f64>>,
    mut vector: Vec<f64>,
) -> Result<Vec<f64>, String> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_polynomial() {
        // Test degree 1 (should match linear regression)
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![2.0, 4.0, 6.0, 8.0, 10.0], // y = 2x
            degree: 1,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert_eq!(result.degree, 1);
        assert_eq!(result.coefficients.len(), 2);
        assert!((result.coefficients[1] - 2.0).abs() < 1e-10); // slope = 2
        assert!(result.coefficients[0].abs() < 1e-10); // intercept ≈ 0
        assert!((result.r_squared - 1.0).abs() < 1e-10); // Perfect fit
    }

    #[test]
    fn test_quadratic_polynomial() {
        // Test degree 2: y = x^2
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![1.0, 4.0, 9.0, 16.0, 25.0], // y = x^2
            degree: 2,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert_eq!(result.degree, 2);
        assert_eq!(result.coefficients.len(), 3);
        assert!(result.coefficients[0].abs() < 1e-10); // constant term ≈ 0
        assert!(result.coefficients[1].abs() < 1e-10); // linear term ≈ 0
        assert!((result.coefficients[2] - 1.0).abs() < 1e-10); // quadratic term = 1
        assert!((result.r_squared - 1.0).abs() < 1e-10); // Perfect fit
    }

    #[test]
    fn test_cubic_polynomial() {
        // Test degree 3: y = x^3 + 2x^2 + 3x + 4
        let x_vals = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let y_vals: Vec<f64> = x_vals
            .iter()
            .map(|&x: &f64| x.powi(3) + 2.0 * x.powi(2) + 3.0 * x + 4.0)
            .collect();

        let input = PolynomialRegressionInput {
            x: x_vals,
            y: y_vals,
            degree: 3,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert_eq!(result.degree, 3);
        assert_eq!(result.coefficients.len(), 4);
        assert!((result.coefficients[0] - 4.0).abs() < 1e-8); // constant = 4
        assert!((result.coefficients[1] - 3.0).abs() < 1e-8); // linear = 3
        assert!((result.coefficients[2] - 2.0).abs() < 1e-8); // quadratic = 2
        assert!((result.coefficients[3] - 1.0).abs() < 1e-8); // cubic = 1
        assert!(result.r_squared > 0.99); // Near perfect fit
    }

    #[test]
    fn test_insufficient_data_points() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0],
            y: vec![1.0, 4.0],
            degree: 3, // Need 4 points for degree 3
        };

        let result = calculate_polynomial_regression(input);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .contains("Need at least 4 data points")
        );
    }

    #[test]
    fn test_zero_degree() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 4.0, 9.0],
            degree: 0,
        };

        let result = calculate_polynomial_regression(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("degree must be at least 1"));
    }

    #[test]
    fn test_high_degree() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 4.0, 9.0],
            degree: 11, // Too high
        };

        let result = calculate_polynomial_regression(input);
        assert!(result.is_err());
        let error = result.err().unwrap();
        assert!(error.contains("cannot exceed 10") || error.contains("Need at least"));
    }

    #[test]
    fn test_mismatched_lengths() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 4.0], // Different length
            degree: 2,
        };

        let result = calculate_polynomial_regression(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("same length"));
    }

    #[test]
    fn test_nan_values() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, f64::NAN, 3.0],
            y: vec![1.0, 4.0, 9.0],
            degree: 2,
        };

        let result = calculate_polynomial_regression(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid values"));
    }

    #[test]
    fn test_equation_string_formation() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: vec![1.0, 3.0, 6.0], // y = 0.5x^2 + 0.5x
            degree: 2,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert!(result.equation.contains("y = "));
        assert!(result.equation.contains("x"));
    }

    #[test]
    fn test_residuals_calculation() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![1.0, 4.0, 9.0, 16.0, 25.0], // Perfect y = x^2
            degree: 2,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert_eq!(result.residuals.len(), 5);
        // Residuals should be near zero for perfect fit
        for residual in &result.residuals {
            assert!(residual.abs() < 1e-10);
        }
    }

    #[test]
    fn test_predicted_values() {
        let y_values = vec![2.0, 6.0, 12.0]; // y = 2x^2
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0],
            y: y_values.clone(),
            degree: 2,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert_eq!(result.predicted_values.len(), 3);
        // Check predicted values match input y values (perfect fit)
        for (predicted, &actual) in result.predicted_values.iter().zip(&y_values) {
            assert!((predicted - actual).abs() < 1e-8);
        }
    }

    #[test]
    fn test_r_squared_calculation() {
        let input = PolynomialRegressionInput {
            x: vec![1.0, 2.0, 3.0, 4.0, 5.0],
            y: vec![1.0, 4.0, 9.0, 16.0, 25.0], // Perfect y = x^2
            degree: 2,
        };

        let result = calculate_polynomial_regression(input).unwrap();
        assert!((result.r_squared - 1.0).abs() < 1e-10); // Perfect fit
    }
}
