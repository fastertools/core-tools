use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticsInput {
    /// Array of numerical values to analyze
    pub data: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescriptiveStatisticsOutput {
    /// Number of data points
    pub count: usize,
    /// Arithmetic mean of the data
    pub mean: f64,
    /// Middle value when sorted
    pub median: f64,
    /// Most frequently occurring value
    pub mode: Option<f64>,
    /// Standard deviation (population)
    pub standard_deviation: f64,
    /// Variance (population)
    pub variance: f64,
    /// Minimum value
    pub min: f64,
    /// Maximum value
    pub max: f64,
    /// Range (max - min)
    pub range: f64,
    /// Sum of all values
    pub sum: f64,
    /// Quartile information
    pub quartiles: Quartiles,
    /// Measure of asymmetry
    pub skewness: f64,
    /// Measure of tail heaviness (excess kurtosis)
    pub kurtosis: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quartiles {
    /// First quartile (25th percentile)
    pub q1: f64,
    /// Second quartile (50th percentile, median)
    pub q2: f64,
    /// Third quartile (75th percentile)
    pub q3: f64,
    /// Interquartile range (q3 - q1)
    pub iqr: f64,
}

pub fn descriptive_statistics_logic(
    input: StatisticsInput,
) -> Result<DescriptiveStatisticsOutput, String> {
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

    // Sort data for median and quartiles
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let median = calculate_median(&sorted_data);
    let mode = calculate_mode(data);

    // Variance and standard deviation
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
    let standard_deviation = variance.sqrt();

    // Min, max, range
    let min = sorted_data[0];
    let max = sorted_data[count - 1];
    let range = max - min;

    // Quartiles
    let quartiles = calculate_quartiles(&sorted_data);

    // Skewness and kurtosis
    let skewness = calculate_skewness(data, mean, standard_deviation);
    let kurtosis = calculate_kurtosis(data, mean, standard_deviation);

    Ok(DescriptiveStatisticsOutput {
        count,
        mean,
        median,
        mode,
        standard_deviation,
        variance,
        min,
        max,
        range,
        sum,
        quartiles,
        skewness,
        kurtosis,
    })
}

fn calculate_median(sorted_data: &[f64]) -> f64 {
    let n = sorted_data.len();
    if n % 2 == 0 {
        (sorted_data[n / 2 - 1] + sorted_data[n / 2]) / 2.0
    } else {
        sorted_data[n / 2]
    }
}

fn calculate_mode(data: &[f64]) -> Option<f64> {
    let mut frequency: HashMap<String, usize> = HashMap::new();

    // Use string representation to handle floating point precision
    for &value in data {
        let key = format!("{value:.10}");
        *frequency.entry(key).or_insert(0) += 1;
    }

    let max_count = frequency.values().max().unwrap_or(&0);

    // Only return mode if there's a clear winner (appears more than once)
    if *max_count > 1 {
        let modes: Vec<f64> = frequency
            .iter()
            .filter(|&(_, &count)| count == *max_count)
            .map(|(key, _)| key.parse::<f64>().unwrap())
            .collect();

        // If there's only one mode, return it
        if modes.len() == 1 {
            Some(modes[0])
        } else {
            // Multiple modes - return the first one found
            Some(modes[0])
        }
    } else {
        None
    }
}

fn calculate_quartiles(sorted_data: &[f64]) -> Quartiles {
    let q1 = calculate_percentile(sorted_data, 25.0);
    let q2 = calculate_percentile(sorted_data, 50.0); // median
    let q3 = calculate_percentile(sorted_data, 75.0);
    let iqr = q3 - q1;

    Quartiles { q1, q2, q3, iqr }
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

fn calculate_skewness(data: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }

    let n = data.len() as f64;
    let skewness = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>()
        / n;

    skewness
}

fn calculate_kurtosis(data: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }

    let n = data.len() as f64;
    let kurtosis = data
        .iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>()
        / n;

    // Excess kurtosis (subtract 3 for normal distribution)
    kurtosis - 3.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_statistics() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.count, 5);
        assert_eq!(result.mean, 3.0);
        assert_eq!(result.median, 3.0);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 5.0);
        assert_eq!(result.range, 4.0);
        assert_eq!(result.sum, 15.0);
    }

    #[test]
    fn test_even_length_median() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.median, 2.5); // (2 + 3) / 2
    }

    #[test]
    fn test_variance_and_std_dev() {
        let input = StatisticsInput {
            data: vec![2.0, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.variance, 4.0);
        assert_eq!(result.standard_deviation, 2.0);
    }

    #[test]
    fn test_quartiles() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.quartiles.q1, 3.0);
        assert_eq!(result.quartiles.q2, 5.0); // median
        assert_eq!(result.quartiles.q3, 7.0);
        assert_eq!(result.quartiles.iqr, 4.0); // 7 - 3
    }

    #[test]
    fn test_mode_single() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 2.0, 3.0, 4.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.mode, Some(2.0));
    }

    #[test]
    fn test_mode_none() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.mode, None);
    }

    #[test]
    fn test_skewness_symmetric() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert!((result.skewness).abs() < 1e-10); // Should be close to 0 for symmetric data
    }

    #[test]
    fn test_kurtosis_normal() {
        let input = StatisticsInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        // Uniform distribution has negative excess kurtosis
        assert!(result.kurtosis < 0.0);
    }

    #[test]
    fn test_empty_data() {
        let input = StatisticsInput { data: vec![] };

        let result = descriptive_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_nan_data() {
        let input = StatisticsInput {
            data: vec![1.0, f64::NAN, 3.0],
        };

        let result = descriptive_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_infinite_data() {
        let input = StatisticsInput {
            data: vec![1.0, f64::INFINITY, 3.0],
        };

        let result = descriptive_statistics_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_single_value() {
        let input = StatisticsInput { data: vec![42.0] };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.count, 1);
        assert_eq!(result.mean, 42.0);
        assert_eq!(result.median, 42.0);
        assert_eq!(result.min, 42.0);
        assert_eq!(result.max, 42.0);
        assert_eq!(result.range, 0.0);
        assert_eq!(result.variance, 0.0);
        assert_eq!(result.standard_deviation, 0.0);
        assert_eq!(result.skewness, 0.0);
        assert_eq!(result.kurtosis, 0.0);
    }

    #[test]
    fn test_all_same_values() {
        let input = StatisticsInput {
            data: vec![5.0, 5.0, 5.0, 5.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.mean, 5.0);
        assert_eq!(result.median, 5.0);
        assert_eq!(result.mode, Some(5.0));
        assert_eq!(result.variance, 0.0);
        assert_eq!(result.standard_deviation, 0.0);
        assert_eq!(result.skewness, 0.0);
        assert_eq!(result.kurtosis, 0.0);
    }

    #[test]
    fn test_large_dataset() {
        let data: Vec<f64> = (1..=1000).map(|i| i as f64).collect();
        let input = StatisticsInput { data };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.count, 1000);
        assert_eq!(result.mean, 500.5);
        assert_eq!(result.median, 500.5);
        assert_eq!(result.min, 1.0);
        assert_eq!(result.max, 1000.0);
    }

    #[test]
    fn test_negative_values() {
        let input = StatisticsInput {
            data: vec![-5.0, -2.0, 0.0, 2.0, 5.0],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert_eq!(result.mean, 0.0);
        assert_eq!(result.median, 0.0);
        assert_eq!(result.min, -5.0);
        assert_eq!(result.max, 5.0);
        assert_eq!(result.range, 10.0);
    }

    #[test]
    fn test_floating_point_precision() {
        let input = StatisticsInput {
            data: vec![1.1, 2.2, 3.3, 4.4, 5.5],
        };

        let result = descriptive_statistics_logic(input).unwrap();
        assert!((result.mean - 3.3).abs() < 1e-10);
        assert!((result.sum - 16.5).abs() < 1e-10);
    }
}
