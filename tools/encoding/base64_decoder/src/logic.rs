use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::{general_purpose, GeneralPurpose}, alphabet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64DecoderInput {
    /// Base64 encoded string to decode
    pub encoded: String,
    /// Decoding variant (optional, default: "standard")
    /// Options: "standard", "standard_no_pad", "url_safe", "url_safe_no_pad"
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64DecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Decoded data as UTF-8 string (if valid UTF-8)
    pub decoded_utf8: Option<String>,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length in bytes
    pub decoded_length: usize,
    /// Decoding variant used
    pub variant: String,
    /// Whether the decoded data is valid UTF-8
    pub is_valid_utf8: bool,
}

pub fn decode_base64(input: Base64DecoderInput) -> Result<Base64DecoderOutput, String> {
    if input.encoded.is_empty() {
        return Err("Encoded data cannot be empty".to_string());
    }
    
    let variant = input.variant.unwrap_or_else(|| "standard".to_string());
    
    // Remove whitespace from input (common in base64 strings)
    let cleaned_input: String = input.encoded.chars()
        .filter(|c| !c.is_whitespace())
        .collect();
    
    // Get the appropriate engine based on variant
    let decoded_bytes = match variant.as_str() {
        "standard" => {
            general_purpose::STANDARD.decode(&cleaned_input)
        },
        "standard_no_pad" => {
            general_purpose::STANDARD_NO_PAD.decode(&cleaned_input)
        },
        "url_safe" => {
            general_purpose::URL_SAFE.decode(&cleaned_input)
        },
        "url_safe_no_pad" => {
            general_purpose::URL_SAFE_NO_PAD.decode(&cleaned_input)
        },
        _ => {
            return Err(format!(
                "Invalid variant '{}'. Valid variants are: standard, standard_no_pad, url_safe, url_safe_no_pad",
                variant
            ));
        }
    }.map_err(|e| format!("Failed to decode base64: {}", e))?;
    
    // Try to convert to UTF-8 string
    let decoded_utf8 = String::from_utf8(decoded_bytes.clone()).ok();
    let is_valid_utf8 = decoded_utf8.is_some();
    
    // For the decoded field, if it's valid UTF-8, use that, otherwise convert bytes to string representation
    let decoded = if let Some(utf8_str) = &decoded_utf8 {
        utf8_str.clone()
    } else {
        // Convert bytes to a readable format (e.g., hex or escaped)
        format!("[Binary data: {} bytes]", decoded_bytes.len())
    };
    
    Ok(Base64DecoderOutput {
        decoded,
        decoded_utf8,
        encoded_length: input.encoded.len(),
        decoded_length: decoded_bytes.len(),
        variant,
        is_valid_utf8,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_simple_string() {
        let input = Base64DecoderInput {
            encoded: "SGVsbG8sIFdvcmxkIQ==".to_string(),
            variant: None,
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, "Hello, World!");
        assert_eq!(result.decoded_utf8, Some("Hello, World!".to_string()));
        assert_eq!(result.encoded_length, 20);
        assert_eq!(result.decoded_length, 13);
        assert_eq!(result.variant, "standard");
        assert!(result.is_valid_utf8);
    }
    
    #[test]
    fn test_decode_empty_error() {
        let input = Base64DecoderInput {
            encoded: "".to_string(),
            variant: None,
        };
        
        let result = decode_base64(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Encoded data cannot be empty");
    }
    
    #[test]
    fn test_decode_with_whitespace() {
        let input = Base64DecoderInput {
            encoded: "SGVs bG8s\nIFdv\tcmxk IQ==".to_string(),
            variant: None,
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, "Hello, World!");
        assert!(result.is_valid_utf8);
    }
    
    #[test]
    fn test_decode_no_padding() {
        let input = Base64DecoderInput {
            encoded: "SGVsbG8sIFdvcmxkIQ".to_string(),
            variant: Some("standard_no_pad".to_string()),
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, "Hello, World!");
        assert_eq!(result.variant, "standard_no_pad");
    }
    
    #[test]
    fn test_decode_url_safe() {
        // This would have + and / in standard encoding
        let input = Base64DecoderInput {
            encoded: "Pz8-Pg".to_string(), // URL safe encoding of "??>>""
            variant: Some("url_safe_no_pad".to_string()),
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, "??>>");
        assert_eq!(result.variant, "url_safe_no_pad");
    }
    
    #[test]
    fn test_decode_invalid_base64() {
        let input = Base64DecoderInput {
            encoded: "This is not valid base64!@#$".to_string(),
            variant: None,
        };
        
        let result = decode_base64(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode base64"));
    }
    
    #[test]
    fn test_decode_unicode() {
        let input = Base64DecoderInput {
            encoded: "SGVsbG8g5LiW55WMIPCfjI0=".to_string(),
            variant: None,
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, "Hello 世界 🌍");
        assert!(result.is_valid_utf8);
    }
    
    #[test]
    fn test_decode_binary_data() {
        // Base64 encoding of binary data that's not valid UTF-8
        let input = Base64DecoderInput {
            encoded: "/v8=".to_string(), // Binary: 0xFF 0xFF
            variant: None,
        };
        
        let result = decode_base64(input).unwrap();
        assert!(!result.is_valid_utf8);
        assert!(result.decoded_utf8.is_none());
        assert!(result.decoded.contains("[Binary data:"));
        assert_eq!(result.decoded_length, 2);
    }
    
    #[test]
    fn test_invalid_variant() {
        let input = Base64DecoderInput {
            encoded: "SGVsbG8=".to_string(),
            variant: Some("invalid".to_string()),
        };
        
        let result = decode_base64(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid variant"));
    }
    
    #[test]
    fn test_decode_with_wrong_padding() {
        // Try to decode with wrong variant (has padding but using no_pad variant)
        let input = Base64DecoderInput {
            encoded: "SGVsbG8sIFdvcmxkIQ==".to_string(),
            variant: Some("standard_no_pad".to_string()),
        };
        
        // This should fail because we're using no_pad variant with padded data
        let result = decode_base64(input);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_round_trip() {
        // Test that encoding then decoding gives back original
        let original = "The quick brown fox jumps over the lazy dog.";
        let encoded = general_purpose::STANDARD.encode(original);
        
        let input = Base64DecoderInput {
            encoded,
            variant: None,
        };
        
        let result = decode_base64(input).unwrap();
        assert_eq!(result.decoded, original);
        assert!(result.is_valid_utf8);
    }
}