use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_prediction() {
        let input = PredictionInput {
            slope: 2.0,
            intercept: 1.0,
            x_values: vec![1.0, 2.0, 3.0],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 3);
        assert_eq!(result.predictions[0].y_predicted, 3.0); // 2*1 + 1
        assert_eq!(result.predictions[1].y_predicted, 5.0); // 2*2 + 1
        assert_eq!(result.predictions[2].y_predicted, 7.0); // 2*3 + 1
    }

    #[test]
    fn test_zero_slope() {
        let input = PredictionInput {
            slope: 0.0,
            intercept: 5.0,
            x_values: vec![1.0, 2.0, 3.0],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 3);
        assert_eq!(result.predictions[0].y_predicted, 5.0);
        assert_eq!(result.predictions[1].y_predicted, 5.0);
        assert_eq!(result.predictions[2].y_predicted, 5.0);
    }

    #[test]
    fn test_negative_slope() {
        let input = PredictionInput {
            slope: -1.5,
            intercept: 10.0,
            x_values: vec![2.0, 4.0],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 2);
        assert_eq!(result.predictions[0].y_predicted, 7.0);  // -1.5*2 + 10
        assert_eq!(result.predictions[1].y_predicted, 4.0);  // -1.5*4 + 10
    }

    #[test]
    fn test_single_value() {
        let input = PredictionInput {
            slope: 3.0,
            intercept: -2.0,
            x_values: vec![5.0],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 1);
        assert_eq!(result.predictions[0].y_predicted, 13.0); // 3*5 - 2
    }

    #[test]
    fn test_empty_x_values() {
        let input = PredictionInput {
            slope: 1.0,
            intercept: 0.0,
            x_values: vec![],
        };
        
        let result = predict_values(input);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "X values for prediction cannot be empty");
    }

    #[test]
    fn test_nan_x_values() {
        let input = PredictionInput {
            slope: 1.0,
            intercept: 0.0,
            x_values: vec![1.0, f64::NAN, 3.0],
        };
        
        let result = predict_values(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid values"));
    }

    #[test]
    fn test_infinite_x_values() {
        let input = PredictionInput {
            slope: 1.0,
            intercept: 0.0,
            x_values: vec![1.0, f64::INFINITY, 3.0],
        };
        
        let result = predict_values(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("invalid values"));
    }

    #[test]
    fn test_nan_slope() {
        let input = PredictionInput {
            slope: f64::NAN,
            intercept: 0.0,
            x_values: vec![1.0, 2.0],
        };
        
        let result = predict_values(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Slope or intercept contains invalid values"));
    }

    #[test]
    fn test_infinite_intercept() {
        let input = PredictionInput {
            slope: 1.0,
            intercept: f64::INFINITY,
            x_values: vec![1.0, 2.0],
        };
        
        let result = predict_values(input);
        assert!(result.is_err());
        assert!(result.err().unwrap().contains("Slope or intercept contains invalid values"));
    }

    #[test]
    fn test_large_values() {
        let input = PredictionInput {
            slope: 1000.0,
            intercept: -500.0,
            x_values: vec![0.1, 0.2, 0.3],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 3);
        assert_eq!(result.predictions[0].y_predicted, -400.0); // 1000*0.1 - 500
        assert_eq!(result.predictions[1].y_predicted, -300.0); // 1000*0.2 - 500
        assert_eq!(result.predictions[2].y_predicted, -200.0); // 1000*0.3 - 500
    }

    #[test]
    fn test_fractional_values() {
        let input = PredictionInput {
            slope: 0.5,
            intercept: 0.25,
            x_values: vec![1.5, 2.5, 3.5],
        };
        
        let result = predict_values(input).unwrap();
        assert_eq!(result.predictions.len(), 3);
        assert_eq!(result.predictions[0].y_predicted, 1.0);   // 0.5*1.5 + 0.25
        assert_eq!(result.predictions[1].y_predicted, 1.5);   // 0.5*2.5 + 0.25
        assert_eq!(result.predictions[2].y_predicted, 2.0);   // 0.5*3.5 + 0.25
    }

    #[test]
    fn test_confidence_interval_none() {
        let input = PredictionInput {
            slope: 1.0,
            intercept: 0.0,
            x_values: vec![1.0],
        };
        
        let result = predict_values(input).unwrap();
        assert!(result.predictions[0].confidence_interval.is_none());
    }
}