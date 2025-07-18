use serde::{Deserialize, Serialize};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexDecoderInput {
    /// Hex encoded string to decode
    pub encoded: String,
    /// Whether to ignore whitespace in the input (optional, default: true)
    pub ignore_whitespace: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexDecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Decoded data as UTF-8 string (if valid UTF-8)
    pub decoded_utf8: Option<String>,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length in bytes
    pub decoded_length: usize,
    /// Whether the decoded data is valid UTF-8
    pub is_valid_utf8: bool,
    /// Number of hex pairs decoded
    pub pairs_decoded: usize,
}

pub fn decode_hex(input: HexDecoderInput) -> Result<HexDecoderOutput, String> {
    if input.encoded.is_empty() {
        return Err("Encoded data cannot be empty".to_string());
    }
    
    let ignore_whitespace = input.ignore_whitespace.unwrap_or(true);
    
    // Clean the input if needed
    let cleaned_input = if ignore_whitespace {
        input.encoded.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    } else {
        input.encoded.clone()
    };
    
    // Validate that the string has even length (hex requires pairs)
    if cleaned_input.len() % 2 != 0 {
        return Err("Hex string must have even length (pairs of characters)".to_string());
    }
    
    // Count pairs before decoding
    let pairs_decoded = cleaned_input.len() / 2;
    
    // Decode the hex string
    match hex::decode(&cleaned_input) {
        Ok(bytes) => {
            // Try to convert to UTF-8 string
            let decoded_utf8 = String::from_utf8(bytes.clone()).ok();
            let is_valid_utf8 = decoded_utf8.is_some();
            
            // For the decoded field, use UTF-8 if valid, otherwise show binary representation
            let decoded = if let Some(utf8_str) = &decoded_utf8 {
                utf8_str.clone()
            } else {
                format!("[Binary data: {} bytes]", bytes.len())
            };
            
            Ok(HexDecoderOutput {
                decoded,
                decoded_utf8,
                encoded_length: input.encoded.len(),
                decoded_length: bytes.len(),
                is_valid_utf8,
                pairs_decoded,
            })
        },
        Err(e) => {
            Err(format!("Failed to decode hex: {}", e))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_decode_simple_string() {
        let input = HexDecoderInput {
            encoded: "48656c6c6f".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "Hello");
        assert_eq!(result.decoded_utf8, Some("Hello".to_string()));
        assert!(result.is_valid_utf8);
        assert_eq!(result.decoded_length, 5);
        assert_eq!(result.pairs_decoded, 5);
    }
    
    #[test]
    fn test_decode_uppercase() {
        let input = HexDecoderInput {
            encoded: "48656C6C6F".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "Hello");
        assert_eq!(result.pairs_decoded, 5);
    }
    
    #[test]
    fn test_decode_with_whitespace() {
        let input = HexDecoderInput {
            encoded: "48 65 6c 6c 6f".to_string(),
            ignore_whitespace: Some(true),
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "Hello");
        assert_eq!(result.encoded_length, 14); // includes spaces
        assert_eq!(result.decoded_length, 5);
    }
    
    #[test]
    fn test_decode_without_ignoring_whitespace() {
        let input = HexDecoderInput {
            encoded: "48 65 6c 6c 6f".to_string(),
            ignore_whitespace: Some(false),
        };
        
        let result = decode_hex(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode hex"));
    }
    
    #[test]
    fn test_decode_empty_error() {
        let input = HexDecoderInput {
            encoded: "".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Encoded data cannot be empty");
    }
    
    #[test]
    fn test_decode_odd_length_error() {
        let input = HexDecoderInput {
            encoded: "48656c6c6".to_string(), // Missing one character
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("even length"));
    }
    
    #[test]
    fn test_decode_invalid_hex() {
        let input = HexDecoderInput {
            encoded: "48656c6c6g".to_string(), // 'g' is not a hex digit
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Failed to decode hex"));
    }
    
    #[test]
    fn test_decode_unicode() {
        let input = HexDecoderInput {
            encoded: "48656c6c6f20e4b896e7958c".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "Hello 世界");
        assert!(result.is_valid_utf8);
        assert_eq!(result.pairs_decoded, 12);
    }
    
    #[test]
    fn test_decode_newlines() {
        let input = HexDecoderInput {
            encoded: "6c696e65310a6c696e6532".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "line1\nline2");
        assert_eq!(result.decoded_length, 11);
    }
    
    #[test]
    fn test_decode_null_bytes() {
        let input = HexDecoderInput {
            encoded: "610062".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "a\0b");
        assert_eq!(result.decoded_length, 3);
    }
    
    #[test]
    fn test_decode_binary_data() {
        let input = HexDecoderInput {
            encoded: "fffefd".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert!(!result.is_valid_utf8);
        assert!(result.decoded_utf8.is_none());
        assert_eq!(result.decoded, "[Binary data: 3 bytes]");
        assert_eq!(result.decoded_length, 3);
    }
    
    #[test]
    fn test_decode_all_zeros() {
        let input = HexDecoderInput {
            encoded: "000000".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "\0\0\0");
        assert_eq!(result.decoded_length, 3);
        assert_eq!(result.pairs_decoded, 3);
    }
    
    #[test]
    fn test_decode_mixed_case() {
        let input = HexDecoderInput {
            encoded: "48656C6c6F".to_string(),
            ignore_whitespace: None,
        };
        
        let result = decode_hex(input).unwrap();
        assert_eq!(result.decoded, "Hello");
    }
    
    #[test]
    fn test_length_relationship() {
        // Test that decoded length is always half of cleaned encoded length
        let test_hexes = vec!["48", "4865", "48656c", "48656c6c", "48656c6c6f"];
        
        for hex in test_hexes {
            let input = HexDecoderInput {
                encoded: hex.to_string(),
                ignore_whitespace: None,
            };
            
            let result = decode_hex(input).unwrap();
            assert_eq!(result.decoded_length * 2, hex.len());
        }
    }
}