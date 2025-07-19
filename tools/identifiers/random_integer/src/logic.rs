use rand::{Rng, thread_rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomIntegerInput {
    /// Minimum value (inclusive)
    pub min: Option<i64>,
    /// Maximum value (inclusive)
    pub max: Option<i64>,
    /// Number of random integers to generate (default: 1, max: 100)
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomIntegerOutput {
    /// Generated random integer(s)
    pub values: Vec<i64>,
    /// Range used
    pub range: RandomRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomRange {
    pub min: i64,
    pub max: i64,
}

pub fn generate_random_integers(input: RandomIntegerInput) -> Result<RandomIntegerOutput, String> {
    // Set defaults
    let min = input.min.unwrap_or(0);
    let max = input.max.unwrap_or(100);
    let count = input.count.unwrap_or(1);

    // Validate inputs
    if min > max {
        return Err("Minimum value must be less than or equal to maximum value".to_string());
    }

    if count == 0 {
        return Err("Count must be at least 1".to_string());
    }

    if count > 100 {
        return Err("Count cannot exceed 100".to_string());
    }

    // Check for overflow
    if max as i128 - min as i128 > i64::MAX as i128 {
        return Err("Range is too large".to_string());
    }

    // Generate random integers
    let mut rng = thread_rng();
    let mut values = Vec::with_capacity(count as usize);

    for _ in 0..count {
        let value = rng.gen_range(min..=max);
        values.push(value);
    }

    Ok(RandomIntegerOutput {
        values,
        range: RandomRange { min, max },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_range() {
        let input = RandomIntegerInput {
            min: None,
            max: None,
            count: None,
        };

        let result = generate_random_integers(input).unwrap();
        assert_eq!(result.values.len(), 1);
        assert!(result.values[0] >= 0);
        assert!(result.values[0] <= 100);
        assert_eq!(result.range.min, 0);
        assert_eq!(result.range.max, 100);
    }

    #[test]
    fn test_custom_range() {
        let input = RandomIntegerInput {
            min: Some(10),
            max: Some(20),
            count: Some(5),
        };

        let result = generate_random_integers(input).unwrap();
        assert_eq!(result.values.len(), 5);

        for value in &result.values {
            assert!(*value >= 10);
            assert!(*value <= 20);
        }

        assert_eq!(result.range.min, 10);
        assert_eq!(result.range.max, 20);
    }

    #[test]
    fn test_negative_range() {
        let input = RandomIntegerInput {
            min: Some(-50),
            max: Some(-10),
            count: Some(3),
        };

        let result = generate_random_integers(input).unwrap();
        assert_eq!(result.values.len(), 3);

        for value in &result.values {
            assert!(*value >= -50);
            assert!(*value <= -10);
        }
    }

    #[test]
    fn test_single_value_range() {
        let input = RandomIntegerInput {
            min: Some(42),
            max: Some(42),
            count: Some(5),
        };

        let result = generate_random_integers(input).unwrap();
        assert_eq!(result.values.len(), 5);

        for value in &result.values {
            assert_eq!(*value, 42);
        }
    }

    #[test]
    fn test_large_range() {
        let input = RandomIntegerInput {
            min: Some(i64::MIN / 2),
            max: Some(i64::MAX / 2),
            count: Some(10),
        };

        let result = generate_random_integers(input).unwrap();
        assert_eq!(result.values.len(), 10);

        for value in &result.values {
            assert!(*value >= i64::MIN / 2);
            assert!(*value <= i64::MAX / 2);
        }
    }

    #[test]
    fn test_invalid_range() {
        let input = RandomIntegerInput {
            min: Some(100),
            max: Some(10),
            count: Some(1),
        };

        let result = generate_random_integers(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Minimum value must be less than or equal to maximum value"
        );
    }

    #[test]
    fn test_zero_count() {
        let input = RandomIntegerInput {
            min: Some(0),
            max: Some(10),
            count: Some(0),
        };

        let result = generate_random_integers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Count must be at least 1");
    }

    #[test]
    fn test_exceeds_max_count() {
        let input = RandomIntegerInput {
            min: Some(0),
            max: Some(10),
            count: Some(101),
        };

        let result = generate_random_integers(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Count cannot exceed 100");
    }

    #[test]
    fn test_randomness() {
        let input = RandomIntegerInput {
            min: Some(0),
            max: Some(1000),
            count: Some(100),
        };

        let result1 = generate_random_integers(input.clone()).unwrap();
        let result2 = generate_random_integers(input).unwrap();

        // With high probability, two sets of 100 random numbers won't be identical
        assert_ne!(result1.values, result2.values);
    }

    #[test]
    fn test_distribution() {
        // Test that values are reasonably distributed
        let input = RandomIntegerInput {
            min: Some(0),
            max: Some(9),
            count: Some(100),
        };

        let result = generate_random_integers(input).unwrap();

        // Count occurrences of each digit
        let mut counts = vec![0; 10];
        for value in &result.values {
            counts[*value as usize] += 1;
        }

        // Each digit should appear at least once (very high probability)
        for count in &counts {
            assert!(*count > 0);
        }
    }
}
