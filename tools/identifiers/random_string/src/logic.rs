use serde::{Deserialize, Serialize};
use rand::{thread_rng, Rng};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomStringInput {
    /// Length of the string to generate (default: 16, max: 1000)
    pub length: Option<u32>,
    /// Character set to use (default: "alphanumeric")
    /// Options: "alphanumeric", "alphabetic", "numeric", "lowercase", "uppercase", "hex"
    pub charset: Option<String>,
    /// Number of random strings to generate (default: 1, max: 100)
    pub count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RandomStringOutput {
    /// Generated random string(s)
    pub values: Vec<String>,
    /// Configuration used
    pub config: StringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringConfig {
    pub length: u32,
    pub charset: String,
    pub charset_size: usize,
}

pub fn generate_random_strings(input: RandomStringInput) -> Result<RandomStringOutput, String> {
    // Set defaults
    let length = input.length.unwrap_or(16);
    let charset = input.charset.unwrap_or_else(|| "alphanumeric".to_string());
    let count = input.count.unwrap_or(1);
    
    // Validate inputs
    if length == 0 {
        return Err("Length must be at least 1".to_string());
    }
    
    if length > 1000 {
        return Err("Length cannot exceed 1000".to_string());
    }
    
    if count == 0 {
        return Err("Count must be at least 1".to_string());
    }
    
    if count > 100 {
        return Err("Count cannot exceed 100".to_string());
    }
    
    // Define character sets
    let chars: Vec<char> = match charset.as_str() {
        "alphanumeric" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
            .chars().collect(),
        "alphabetic" => "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars().collect(),
        "numeric" => "0123456789"
            .chars().collect(),
        "lowercase" => "abcdefghijklmnopqrstuvwxyz"
            .chars().collect(),
        "uppercase" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars().collect(),
        "hex" => "0123456789abcdef"
            .chars().collect(),
        _ => {
            return Err(format!(
                "Invalid charset '{}'. Valid options are: alphanumeric, alphabetic, numeric, lowercase, uppercase, hex",
                charset
            ));
        }
    };
    
    let charset_size = chars.len();
    
    // Generate random strings
    let mut rng = thread_rng();
    let mut values = Vec::with_capacity(count as usize);
    
    for _ in 0..count {
        let mut random_string = String::with_capacity(length as usize);
        
        for _ in 0..length {
            let idx = rng.gen_range(0..charset_size);
            random_string.push(chars[idx]);
        }
        
        values.push(random_string);
    }
    
    Ok(RandomStringOutput {
        values,
        config: StringConfig {
            length,
            charset,
            charset_size,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_default_generation() {
        let input = RandomStringInput {
            length: None,
            charset: None,
            count: None,
        };
        
        let result = generate_random_strings(input).unwrap();
        assert_eq!(result.values.len(), 1);
        assert_eq!(result.values[0].len(), 16);
        assert_eq!(result.config.length, 16);
        assert_eq!(result.config.charset, "alphanumeric");
        assert_eq!(result.config.charset_size, 62);
        
        // Check all characters are alphanumeric
        for ch in result.values[0].chars() {
            assert!(ch.is_alphanumeric());
        }
    }
    
    #[test]
    fn test_custom_length() {
        let input = RandomStringInput {
            length: Some(32),
            charset: None,
            count: None,
        };
        
        let result = generate_random_strings(input).unwrap();
        assert_eq!(result.values[0].len(), 32);
        assert_eq!(result.config.length, 32);
    }
    
    #[test]
    fn test_numeric_charset() {
        let input = RandomStringInput {
            length: Some(10),
            charset: Some("numeric".to_string()),
            count: Some(5),
        };
        
        let result = generate_random_strings(input).unwrap();
        assert_eq!(result.values.len(), 5);
        
        for value in &result.values {
            assert_eq!(value.len(), 10);
            for ch in value.chars() {
                assert!(ch.is_numeric());
            }
        }
        
        assert_eq!(result.config.charset, "numeric");
        assert_eq!(result.config.charset_size, 10);
    }
    
    #[test]
    fn test_lowercase_charset() {
        let input = RandomStringInput {
            length: Some(8),
            charset: Some("lowercase".to_string()),
            count: Some(3),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        for value in &result.values {
            assert_eq!(value.len(), 8);
            for ch in value.chars() {
                assert!(ch.is_lowercase());
                assert!(ch.is_alphabetic());
            }
        }
        
        assert_eq!(result.config.charset_size, 26);
    }
    
    #[test]
    fn test_uppercase_charset() {
        let input = RandomStringInput {
            length: Some(8),
            charset: Some("uppercase".to_string()),
            count: Some(3),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        for value in &result.values {
            assert_eq!(value.len(), 8);
            for ch in value.chars() {
                assert!(ch.is_uppercase());
                assert!(ch.is_alphabetic());
            }
        }
        
        assert_eq!(result.config.charset_size, 26);
    }
    
    #[test]
    fn test_hex_charset() {
        let input = RandomStringInput {
            length: Some(16),
            charset: Some("hex".to_string()),
            count: Some(3),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        for value in &result.values {
            assert_eq!(value.len(), 16);
            for ch in value.chars() {
                assert!(ch.is_ascii_hexdigit());
                assert!(ch.is_lowercase() || ch.is_numeric());
            }
        }
        
        assert_eq!(result.config.charset_size, 16);
    }
    
    #[test]
    fn test_alphabetic_charset() {
        let input = RandomStringInput {
            length: Some(12),
            charset: Some("alphabetic".to_string()),
            count: Some(2),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        for value in &result.values {
            assert_eq!(value.len(), 12);
            for ch in value.chars() {
                assert!(ch.is_alphabetic());
            }
        }
        
        assert_eq!(result.config.charset_size, 52);
    }
    
    #[test]
    fn test_zero_length_error() {
        let input = RandomStringInput {
            length: Some(0),
            charset: None,
            count: None,
        };
        
        let result = generate_random_strings(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Length must be at least 1");
    }
    
    #[test]
    fn test_exceeds_max_length_error() {
        let input = RandomStringInput {
            length: Some(1001),
            charset: None,
            count: None,
        };
        
        let result = generate_random_strings(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Length cannot exceed 1000");
    }
    
    #[test]
    fn test_invalid_charset_error() {
        let input = RandomStringInput {
            length: Some(10),
            charset: Some("invalid".to_string()),
            count: None,
        };
        
        let result = generate_random_strings(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid charset"));
    }
    
    #[test]
    fn test_randomness() {
        let input = RandomStringInput {
            length: Some(20),
            charset: Some("alphanumeric".to_string()),
            count: Some(10),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        // Check that all strings are different (very high probability)
        let unique_count = result.values.iter()
            .collect::<std::collections::HashSet<_>>()
            .len();
        assert_eq!(unique_count, 10);
    }
    
    #[test]
    fn test_single_character_strings() {
        let input = RandomStringInput {
            length: Some(1),
            charset: Some("numeric".to_string()),
            count: Some(100),
        };
        
        let result = generate_random_strings(input).unwrap();
        
        // Should see most digits represented
        let unique_chars: std::collections::HashSet<char> = result.values
            .iter()
            .map(|s| s.chars().next().unwrap())
            .collect();
        
        assert!(unique_chars.len() >= 5); // Very high probability of at least 5 different digits
    }
}