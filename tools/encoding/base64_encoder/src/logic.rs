use base64::{
    Engine as _,
    engine::general_purpose,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64EncoderInput {
    /// The data to encode (as string)
    pub data: String,
    /// Encoding variant (optional, default: "standard")
    /// Options: "standard", "standard_no_pad", "url_safe", "url_safe_no_pad"
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64EncoderOutput {
    /// Base64 encoded string
    pub encoded: String,
    /// Original data length in bytes
    pub original_length: usize,
    /// Encoded length
    pub encoded_length: usize,
    /// Encoding variant used
    pub variant: String,
}

pub fn encode_base64(input: Base64EncoderInput) -> Result<Base64EncoderOutput, String> {
    if input.data.is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let variant = input.variant.unwrap_or_else(|| "standard".to_string());

    // Get the appropriate engine based on variant
    let encoded = match variant.as_str() {
        "standard" => general_purpose::STANDARD.encode(&input.data),
        "standard_no_pad" => general_purpose::STANDARD_NO_PAD.encode(&input.data),
        "url_safe" => general_purpose::URL_SAFE.encode(&input.data),
        "url_safe_no_pad" => general_purpose::URL_SAFE_NO_PAD.encode(&input.data),
        _ => {
            return Err(format!(
                "Invalid variant '{variant}'. Valid variants are: standard, standard_no_pad, url_safe, url_safe_no_pad"
            ));
        }
    };

    let original_length = input.data.len();
    let encoded_length = encoded.len();

    Ok(Base64EncoderOutput {
        encoded,
        original_length,
        encoded_length,
        variant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string() {
        let input = Base64EncoderInput {
            data: "Hello, World!".to_string(),
            variant: None,
        };

        let result = encode_base64(input).unwrap();
        assert_eq!(result.encoded, "SGVsbG8sIFdvcmxkIQ==");
        assert_eq!(result.original_length, 13);
        assert_eq!(result.encoded_length, 20);
        assert_eq!(result.variant, "standard");
    }

    #[test]
    fn test_encode_empty_error() {
        let input = Base64EncoderInput {
            data: "".to_string(),
            variant: None,
        };

        let result = encode_base64(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Data cannot be empty");
    }

    #[test]
    fn test_encode_no_padding() {
        let input = Base64EncoderInput {
            data: "Hello, World!".to_string(),
            variant: Some("standard_no_pad".to_string()),
        };

        let result = encode_base64(input).unwrap();
        assert_eq!(result.encoded, "SGVsbG8sIFdvcmxkIQ");
        assert_eq!(result.variant, "standard_no_pad");
    }

    #[test]
    fn test_encode_url_safe() {
        // Use data that would contain + and / in standard encoding
        let input = Base64EncoderInput {
            data: "??>>".to_string(),
            variant: Some("url_safe".to_string()),
        };

        let result = encode_base64(input).unwrap();
        assert_eq!(result.variant, "url_safe");
        // URL safe encoding uses - and _ instead of + and /
        assert!(!result.encoded.contains('+'));
        assert!(!result.encoded.contains('/'));
    }

    #[test]
    fn test_encode_special_characters() {
        let input = Base64EncoderInput {
            data: "!@#$%^&*(){}[]|\\:;\"'<>,.?/~`".to_string(),
            variant: None,
        };

        let result = encode_base64(input).unwrap();
        assert!(!result.encoded.is_empty());
        assert!(result.encoded_length > result.original_length);
    }

    #[test]
    fn test_encode_unicode() {
        let input = Base64EncoderInput {
            data: "Hello 世界 🌍".to_string(),
            variant: None,
        };

        let result = encode_base64(input).unwrap();
        assert_eq!(result.encoded, "SGVsbG8g5LiW55WMIPCfjI0=");
        assert_eq!(result.variant, "standard");
    }

    #[test]
    fn test_encode_newlines() {
        let input = Base64EncoderInput {
            data: "Line 1\nLine 2\rLine 3\r\nLine 4".to_string(),
            variant: None,
        };

        let result = encode_base64(input).unwrap();
        assert!(!result.encoded.is_empty());
        assert_eq!(result.variant, "standard");
    }

    #[test]
    fn test_invalid_variant() {
        let input = Base64EncoderInput {
            data: "test".to_string(),
            variant: Some("invalid".to_string()),
        };

        let result = encode_base64(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid variant"));
    }

    #[test]
    fn test_length_calculations() {
        // Test various lengths to ensure proper calculation
        let test_cases = vec![
            "a",      // 1 byte
            "ab",     // 2 bytes
            "abc",    // 3 bytes
            "abcd",   // 4 bytes
            "abcde",  // 5 bytes
            "abcdef", // 6 bytes
        ];

        for data in test_cases {
            let input = Base64EncoderInput {
                data: data.to_string(),
                variant: None,
            };

            let result = encode_base64(input).unwrap();
            assert_eq!(result.original_length, data.len());

            // Base64 encoding increases size by approximately 4/3
            let expected_len = (data.len() * 4).div_ceil(3);
            let expected_len = expected_len.div_ceil(4) * 4; // Padding to multiple of 4
            assert_eq!(result.encoded_length, expected_len);
        }
    }

    #[test]
    fn test_binary_data_simulation() {
        // Simulate binary data with all byte values
        let mut data = String::new();
        for i in 0..256 {
            data.push(char::from(i as u8));
        }

        let input = Base64EncoderInput {
            data,
            variant: None,
        };

        let result = encode_base64(input).unwrap();
        assert_eq!(result.original_length, 384); // UTF-8 encoding makes some chars multi-byte
        assert!(result.encoded_length > 384); // Base64 encoding increases size
        assert_eq!(result.variant, "standard");
    }
}
