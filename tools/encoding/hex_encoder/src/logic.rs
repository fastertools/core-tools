use hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexEncoderInput {
    /// The data to encode (as string)
    pub data: String,
    /// Output case (optional, default: "lowercase")
    /// Options: "lowercase", "uppercase"
    pub case: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HexEncoderOutput {
    /// Hex encoded string
    pub encoded: String,
    /// Original data length in bytes
    pub original_length: usize,
    /// Encoded length (always 2x original for hex)
    pub encoded_length: usize,
    /// Output case used
    pub case: String,
}

pub fn encode_hex(input: HexEncoderInput) -> Result<HexEncoderOutput, String> {
    if input.data.is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let case = input.case.unwrap_or_else(|| "lowercase".to_string());

    // Validate case option
    if !["lowercase", "uppercase"].contains(&case.as_str()) {
        return Err(format!(
            "Invalid case '{}'. Valid options are: lowercase, uppercase",
            case
        ));
    }

    // Convert string to bytes
    let bytes = input.data.as_bytes();

    // Encode to hex
    let encoded = match case.as_str() {
        "lowercase" => hex::encode(bytes),
        "uppercase" => hex::encode_upper(bytes),
        _ => unreachable!(), // We validated case above
    };

    Ok(HexEncoderOutput {
        encoded_length: encoded.len(),
        encoded,
        original_length: bytes.len(),
        case,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_string() {
        let input = HexEncoderInput {
            data: "Hello".to_string(),
            case: None,
        };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.encoded, "48656c6c6f");
        assert_eq!(result.original_length, 5);
        assert_eq!(result.encoded_length, 10);
        assert_eq!(result.case, "lowercase");
    }

    #[test]
    fn test_encode_uppercase() {
        let input = HexEncoderInput {
            data: "Hello".to_string(),
            case: Some("uppercase".to_string()),
        };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.encoded, "48656C6C6F");
        assert_eq!(result.case, "uppercase");
    }

    #[test]
    fn test_encode_empty_error() {
        let input = HexEncoderInput {
            data: "".to_string(),
            case: None,
        };

        let result = encode_hex(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Data cannot be empty");
    }

    #[test]
    fn test_encode_special_characters() {
        let input = HexEncoderInput {
            data: "!@#$%".to_string(),
            case: None,
        };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.encoded, "2140232425");
        assert_eq!(result.original_length, 5);
        assert_eq!(result.encoded_length, 10);
    }

    #[test]
    fn test_encode_unicode() {
        let input = HexEncoderInput {
            data: "Hello 世界".to_string(),
            case: None,
        };

        let result = encode_hex(input).unwrap();
        // "Hello " is 48656c6c6f20, "世界" in UTF-8 is e4b896e7958c
        assert_eq!(result.encoded, "48656c6c6f20e4b896e7958c");
        assert_eq!(result.original_length, 12); // "Hello " (6) + "世界" (6 bytes in UTF-8)
        assert_eq!(result.encoded_length, 24);
    }

    #[test]
    fn test_encode_newlines() {
        let input = HexEncoderInput {
            data: "line1\nline2".to_string(),
            case: None,
        };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.encoded, "6c696e65310a6c696e6532");
        assert_eq!(result.original_length, 11);
        assert_eq!(result.encoded_length, 22);
    }

    #[test]
    fn test_encode_null_bytes() {
        let input = HexEncoderInput {
            data: "a\0b".to_string(),
            case: None,
        };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.encoded, "610062");
        assert_eq!(result.original_length, 3);
        assert_eq!(result.encoded_length, 6);
    }

    #[test]
    fn test_invalid_case_error() {
        let input = HexEncoderInput {
            data: "test".to_string(),
            case: Some("invalid".to_string()),
        };

        let result = encode_hex(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid case"));
    }

    #[test]
    fn test_encode_all_byte_values() {
        // Test encoding of all possible byte values
        let mut data = String::new();
        for i in 0..=255u8 {
            data.push(char::from(i));
        }

        let input = HexEncoderInput { data, case: None };

        let result = encode_hex(input).unwrap();
        assert_eq!(result.original_length, 384); // UTF-8 encoding makes some chars multi-byte
        assert_eq!(result.encoded_length, 768); // Hex encoding doubles the byte length

        // Check that it starts with "000102..."
        assert!(result.encoded.starts_with("000102030405"));
    }

    #[test]
    fn test_length_relationship() {
        // Test that encoded length is always 2x original
        let test_strings = vec!["a", "ab", "abc", "abcd", "abcde"];

        for s in test_strings {
            let input = HexEncoderInput {
                data: s.to_string(),
                case: None,
            };

            let result = encode_hex(input).unwrap();
            assert_eq!(result.encoded_length, result.original_length * 2);
        }
    }

    #[test]
    fn test_hex_charset() {
        let input = HexEncoderInput {
            data: "test".to_string(),
            case: Some("lowercase".to_string()),
        };

        let result = encode_hex(input).unwrap();

        // Check that all characters are valid hex digits
        for ch in result.encoded.chars() {
            assert!(ch.is_ascii_hexdigit());
            assert!(ch.is_lowercase() || ch.is_numeric());
        }
    }
}
