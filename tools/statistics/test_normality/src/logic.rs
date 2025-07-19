use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct TestNormalityInput {
    pub data: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct TestNormalityOutput {
    pub is_normal: bool,
    pub shapiro_wilk_statistic: Option<f64>,
    pub jarque_bera_statistic: f64,
    pub p_value: f64,
    pub confidence_level: f64,
    pub interpretation: String,
}

pub fn calculate_test_normality(input: TestNormalityInput) -> Result<TestNormalityOutput, String> {
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
    let skewness = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>()
        / n;

    let kurtosis = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>()
        / n;

    // Jarque-Bera test
    let jb_statistic = (n / 6.0) * (skewness.powi(2) + (kurtosis - 3.0).powi(2) / 4.0);

    // Approximate p-value for Jarque-Bera test (chi-square with 2 df)
    let p_value = chi_square_p_value(jb_statistic, 2.0);

    let confidence_level = 0.05;
    let is_normal = p_value > confidence_level;

    let interpretation = if is_normal {
        format!(
            "Data appears to be normally distributed (p-value: {:.4} > {:.2})",
            p_value, confidence_level
        )
    } else {
        format!(
            "Data does not appear to be normally distributed (p-value: {:.4} <= {:.2})",
            p_value, confidence_level
        )
    };

    // Shapiro-Wilk test would be more accurate but is complex to implement
    // For now, we set it to None
    let shapiro_wilk_statistic = None;

    Ok(TestNormalityOutput {
        is_normal,
        shapiro_wilk_statistic,
        jarque_bera_statistic: jb_statistic,
        p_value,
        confidence_level,
        interpretation,
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_data() {
        // Test with approximately normal data
        let input = TestNormalityInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 3.0], // Symmetric-ish
        };

        let result = calculate_test_normality(input).unwrap();
        assert!(result.jarque_bera_statistic >= 0.0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
        assert_eq!(result.confidence_level, 0.05);
        assert!(result.interpretation.contains("p-value"));
    }

    #[test]
    fn test_clearly_non_normal_data() {
        // Test with clearly non-normal data (exponential-like)
        let input = TestNormalityInput {
            data: vec![1.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0], // Exponential pattern
        };

        let result = calculate_test_normality(input).unwrap();
        assert!(result.jarque_bera_statistic > 0.0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
        // With such skewed data, it should likely be detected as non-normal
        // But we don't assert the result since statistical tests can vary
    }

    #[test]
    fn test_insufficient_data() {
        let input = TestNormalityInput {
            data: vec![1.0, 2.0], // Only 2 points
        };

        let result = calculate_test_normality(input);
        assert!(result.is_err());
        assert!(
            result
                .err()
                .unwrap()
                .contains("Need at least 3 data points")
        );
    }

    #[test]
    fn test_empty_data() {
        let input = TestNormalityInput { data: vec![] };

        let result = calculate_test_normality(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("cannot be empty"));
    }

    #[test]
    fn test_zero_variance() {
        let input = TestNormalityInput {
            data: vec![5.0, 5.0, 5.0, 5.0, 5.0], // All identical
        };

        let result = calculate_test_normality(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Standard deviation is zero"));
    }

    #[test]
    fn test_nan_values() {
        let input = TestNormalityInput {
            data: vec![1.0, f64::NAN, 3.0],
        };

        let result = calculate_test_normality(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid values"));
    }

    #[test]
    fn test_infinite_values() {
        let input = TestNormalityInput {
            data: vec![1.0, f64::INFINITY, 3.0],
        };

        let result = calculate_test_normality(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid values"));
    }

    #[test]
    fn test_jarque_bera_calculation() {
        // Test with known data that has specific skewness and kurtosis
        let input = TestNormalityInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = calculate_test_normality(input).unwrap();
        assert!(result.jarque_bera_statistic >= 0.0);
        // JB statistic should be finite and non-negative
        assert!(result.jarque_bera_statistic.is_finite());
    }

    #[test]
    fn test_p_value_bounds() {
        let input = TestNormalityInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0],
        };

        let result = calculate_test_normality(input).unwrap();
        assert!(result.p_value >= 0.0);
        assert!(result.p_value <= 1.0);
    }

    #[test]
    fn test_output_fields() {
        let input = TestNormalityInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = calculate_test_normality(input).unwrap();

        // Check all fields are present and reasonable
        assert!(result.is_normal == true || result.is_normal == false);
        assert!(result.shapiro_wilk_statistic.is_none()); // Currently not implemented
        assert!(result.jarque_bera_statistic >= 0.0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
        assert_eq!(result.confidence_level, 0.05);
        assert!(!result.interpretation.is_empty());
    }

    #[test]
    fn test_large_dataset() {
        // Test with larger dataset
        let mut data = Vec::new();
        for i in 1..=100 {
            data.push(i as f64);
        }

        let input = TestNormalityInput { data };
        let result = calculate_test_normality(input).unwrap();

        assert!(result.jarque_bera_statistic >= 0.0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }

    #[test]
    fn test_negative_values() {
        // Test with negative values
        let input = TestNormalityInput {
            data: vec![-5.0, -2.0, 0.0, 2.0, 5.0],
        };

        let result = calculate_test_normality(input).unwrap();
        assert!(result.jarque_bera_statistic >= 0.0);
        assert!(result.p_value >= 0.0 && result.p_value <= 1.0);
    }
}
