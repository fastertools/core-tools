use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StatisticsInput {
    pub data: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct DescriptiveStatisticsOutput {
    pub count: usize,
    pub mean: f64,
    pub median: f64,
    pub mode: Option<f64>,
    pub standard_deviation: f64,
    pub variance: f64,
    pub min: f64,
    pub max: f64,
    pub range: f64,
    pub sum: f64,
    pub quartiles: Quartiles,
    pub skewness: f64,
    pub kurtosis: f64,
}

#[derive(Debug, Serialize)]
pub struct Quartiles {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub iqr: f64,
}

pub fn calculate_descriptive_statistics(input: StatisticsInput) -> Result<DescriptiveStatisticsOutput, String> {
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
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / count as f64;
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
        let key = format!("{:.10}", value);
        *frequency.entry(key).or_insert(0) += 1;
    }
    
    let max_count = frequency.values().max().unwrap_or(&0);
    
    // Only return mode if there's a clear winner (appears more than once)
    if *max_count > 1 {
        let modes: Vec<f64> = frequency
            .iter()
            .filter(|(_, &count)| count == *max_count)
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
    let n = sorted_data.len();
    
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
    let skewness = data.iter()
        .map(|x| ((x - mean) / std_dev).powi(3))
        .sum::<f64>() / n;
    
    skewness
}

fn calculate_kurtosis(data: &[f64], mean: f64, std_dev: f64) -> f64 {
    if std_dev == 0.0 {
        return 0.0;
    }
    
    let n = data.len() as f64;
    let kurtosis = data.iter()
        .map(|x| ((x - mean) / std_dev).powi(4))
        .sum::<f64>() / n;
    
    // Excess kurtosis (subtract 3 for normal distribution)
    kurtosis - 3.0
}

#[derive(Debug, Serialize)]
pub struct SummaryStatisticsOutput {
    pub count: usize,
    pub mean: f64,
    pub std_dev: f64,
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
}

pub fn calculate_summary_statistics(input: StatisticsInput) -> Result<SummaryStatisticsOutput, String> {
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
    let variance = data.iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>() / count as f64;
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