use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryStatisticsOutput {
    /// Number of data points
    pub count: usize,
    /// Arithmetic mean of the data
    pub mean: f64,
    /// Standard deviation
    pub std_dev: f64,
    /// Minimum value
    pub min: f64,
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Median (50th percentile)
    pub median: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Maximum value
    pub max: f64,
}

pub fn summary_statistics_logic(input: StatisticsInput) -> Result<SummaryStatisticsOutput, String> {
    if input.data.is_empty() {
        return Err("Input data cannot be empty".to_string());
    }

    let data = &input.data;
    let count = data.len();

    // Check for invalid values
    if data.iter().any(|&x| x.is_nan() || x.is_infinite()) {
        return Err("Input data contains invalid values (NaN or Infinite)".to_string());
    }

    // Basic calculations
    let sum: f64 = data.iter().sum();
    let mean = sum / count as f64;

    // Variance and standard deviation
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
    let std_dev = variance.sqrt();

    // Sort data for percentiles
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let min = sorted_data[0];
    let max = sorted_data[count - 1];
    let q1 = calculate_percentile(&sorted_data, 25.0);
    let median = calculate_percentile(&sorted_data, 50.0);
    let q3 = calculate_percentile(&sorted_data, 75.0);

    Ok(SummaryStatisticsOutput {
        count,
        mean,
        std_dev,
        min,
        q1,
        median,
        q3,
        max,
    })
}

fn calculate_percentile(sorted_data: &[f64], percentile: f64) -> f64 {
    let n = sorted_data.len();
    let index = (percentile / 100.0) * (n - 1) as f64;
    let lower_index = index.floor() as usize;
    let upper_index = index.ceil() as usize;

    if lower_index == upper_index {
        sorted_data[lower_index]
    } else {
        let weight = index - lower_index as f64;
        sorted_data[lower_index] * (1.0 - weight) + sorted_data[upper_index] * weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_statistics() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 3.0);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 5.0);
        assert_eq!(result.median, 3.0);
        assert!((result.std_dev - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn test_single_value() {
        let input = StatisticsInput { data: vec![42.0] };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.mean, 42.0);
        assert_eq!(result.std_dev, 0.0);
        assert_eq!(result.min, 42.0);
        assert_eq!(result.max, 42.0);
        assert_eq!(result.median, 42.0);
        assert_eq!(result.q1, 42.0);
        assert_eq!(result.q3, 42.0);
    }

    #[test]
    fn test_even_number_of_values() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 4);
        assert_eq!(result.mean, 2.5);
        assert_eq!(result.median, 2.5); // Average of 2.0 and 3.0
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 4.0);
    }

    #[test]
    fn test_percentile_calculations() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 10);
        assert_eq!(result.mean, 5.5);
        assert_eq!(result.median, 5.5);
        assert_eq!(result.q1, 3.25); // 25th percentile
        assert_eq!(result.q3, 7.75); // 75th percentile
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 10.0);
    }

    #[test]
    fn test_negative_and_decimal_values() {
        let input = StatisticsInput {
            data: vec![-2.5, -1.0, 0.0, 1.5, 3.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 0.2);
        assert_eq!(result.median, 0.0);
        assert_eq!(result.min, -2.5);
        assert_eq!(result.max, 3.0);
    }

    #[test]
    fn test_duplicate_values() {
        let input = StatisticsInput {
            data: vec![5.0, 5.0, 5.0, 5.0, 5.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.std_dev, 0.0);
        assert_eq!(result.min, 5.0);
        assert_eq!(result.max, 5.0);
        assert_eq!(result.median, 5.0);
        assert_eq!(result.q1, 5.0);
        assert_eq!(result.q3, 5.0);
    }

    #[test]
    fn test_large_dataset() {
        let data: Vec<f64> = (1..=1000).map(|x| x as f64).collect();
        let input = StatisticsInput { data };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 1000);
        assert_eq!(result.mean, 500.5);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 1000.0);
        assert_eq!(result.median, 500.5);
    }

    #[test]
    fn test_unsorted_data() {
        let input = StatisticsInput {
            data: vec![5.0, 1.0, 9.0, 3.0, 7.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 9.0);
        assert_eq!(result.median, 5.0);
    }

    #[test]
    fn test_empty_data_error() {
        let input = StatisticsInput { data: vec![] };

        let result = summary_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_nan_data_error() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, f64::NAN, 4.0],
        };

        let result = summary_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_infinite_data_error() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, f64::INFINITY, 4.0],
        };

        let result = summary_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_negative_infinite_data_error() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, f64::NEG_INFINITY, 4.0],
        };

        let result = summary_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_very_small_values() {
        let input = StatisticsInput {
            data: vec![1e-10, 2e-10, 3e-10, 4e-10, 5e-10],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert!((result.mean - 3e-10).abs() < 1e-20);
        assert_eq!(result.min, 1e-10);
        assert_eq!(result.max, 5e-10);
    }

    #[test]
    fn test_very_large_values() {
        let input = StatisticsInput {
            data: vec![1e10, 2e10, 3e10, 4e10, 5e10],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert!((result.mean - 3e10).abs() < 1e5);
        assert_eq!(result.min, 1e10);
        assert_eq!(result.max, 5e10);
    }

    #[test]
    fn test_standard_deviation_calculation() {
        // Known dataset with known std dev
        let input = StatisticsInput {
            data: vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        // Manual calculation: mean = 5.0, variance = 4.0, std_dev = 2.0
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.std_dev, 2.0);
    }

    #[test]
    fn test_percentile_edge_cases() {
        // Test with 2 values
        let input = StatisticsInput {
            data: vec![1.0, 10.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.q1, 3.25); // 25% between 1 and 10
        assert_eq!(result.median, 5.5); // 50% between 1 and 10
        assert_eq!(result.q3, 7.75); // 75% between 1 and 10
    }

    #[test]
    fn test_mixed_positive_negative() {
        let input = StatisticsInput {
            data: vec![-10.0, -5.0, 0.0, 5.0, 10.0],
        };

        let result = summary_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 0.0);
        assert_eq!(result.median, 0.0);
        assert_eq!(result.min, -10.0);
        assert_eq!(result.max, 10.0);
    }
}
