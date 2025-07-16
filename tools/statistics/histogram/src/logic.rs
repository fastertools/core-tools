use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramInput {
    pub data: Vec<f64>,
    pub num_bins: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramOutput {
    pub bins: Vec<HistogramBin>,
    pub total_count: usize,
    pub bin_width: f64,
    pub range: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistogramBin {
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub count: usize,
    pub frequency: f64,
    pub density: f64,
}

pub fn generate_histogram(input: HistogramInput) -> Result<HistogramOutput, String> {
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
        ((n.ln() / 2.0_f64.ln()).ceil() as usize + 1).max(1).min(50)
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
            let exact_index = (value - min_val) / bin_width;
            let index = exact_index.floor() as usize;
            
            if index >= num_bins {
                num_bins - 1
            } else {
                // Check if value is exactly on a bin boundary
                if (exact_index.fract()).abs() < 1e-10 && index > 0 {
                    // Value is exactly on boundary, put in previous bin
                    index - 1
                } else {
                    index
                }
            }
        };
        
        counts[bin_index] += 1;
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_histogram() {
        let input = HistogramInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0],
            num_bins: Some(5),
        };
        let result = generate_histogram(input).unwrap();
        assert_eq!(result.bins.len(), 5);
        assert_eq!(result.total_count, 10);
        assert_eq!(result.bin_width, 1.8);
        assert_eq!(result.range, (1.0, 10.0));
    }
    
    #[test]
    fn test_automatic_bins() {
        let input = HistogramInput {
            data: vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            num_bins: None,
        };
        let result = generate_histogram(input).unwrap();
        // Sturges' rule: ceil(log2(8)) + 1 = 4
        assert_eq!(result.bins.len(), 4);
    }
    
    #[test]
    fn test_empty_data_error() {
        let input = HistogramInput {
            data: vec![],
            num_bins: Some(5),
        };
        let result = generate_histogram(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input data cannot be empty");
    }
    
    #[test]
    fn test_uniform_data_error() {
        let input = HistogramInput {
            data: vec![5.0, 5.0, 5.0, 5.0],
            num_bins: Some(5),
        };
        let result = generate_histogram(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "All data values are the same, cannot create histogram");
    }
    
    #[test]
    fn test_nan_values_error() {
        let input = HistogramInput {
            data: vec![1.0, 2.0, f64::NAN, 4.0],
            num_bins: Some(5),
        };
        let result = generate_histogram(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input data contains invalid values (NaN or Infinite)");
    }
    
    #[test]
    fn test_frequency_calculation() {
        let input = HistogramInput {
            data: vec![1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0],
            num_bins: Some(2),
        };
        let result = generate_histogram(input).unwrap();
        
        // First bin should have values 1.0-3.0 (5 values)
        assert_eq!(result.bins[0].count, 5);
        assert_eq!(result.bins[0].frequency, 5.0 / 9.0);
        
        // Second bin should have values 3.0-5.0 (4 values)
        assert_eq!(result.bins[1].count, 4);
        assert_eq!(result.bins[1].frequency, 4.0 / 9.0);
    }
}