use percent_encoding::percent_decode_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDecoderInput {
    /// The URL encoded string to decode
    pub encoded: String,
    /// Whether to decode plus signs as spaces (optional, default: false)
    /// This is common in query strings where spaces are encoded as +
    pub decode_plus: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlDecoderOutput {
    /// Decoded string
    pub decoded: String,
    /// Original encoded length
    pub encoded_length: usize,
    /// Decoded length
    pub decoded_length: usize,
    /// Number of percent-encoded sequences decoded
    pub sequences_decoded: usize,
    /// Whether the decoded result is valid UTF-8
    pub is_valid_utf8: bool,
    /// Error message if decoding failed
    pub error: Option<String>,
}

pub fn decode_url(input: UrlDecoderInput) -> Result<UrlDecoderOutput, String> {
    if input.encoded.is_empty() {
        return Err("Encoded data cannot be empty".to_string());
    }

    let decode_plus = input.decode_plus.unwrap_or(false);

    // If decode_plus is true, replace + with space before decoding
    let to_decode = if decode_plus {
        input.encoded.replace('+', " ")
    } else {
        input.encoded.clone()
    };

    // Count encoded sequences before decoding
    let sequences_decoded = count_encoded_sequences(&to_decode);

    // Perform the decoding
    match percent_decode_str(&to_decode).decode_utf8() {
        Ok(decoded) => {
            let decoded_string = decoded.to_string();

            Ok(UrlDecoderOutput {
                decoded_length: decoded_string.len(),
                decoded: decoded_string,
                encoded_length: input.encoded.len(),
                sequences_decoded,
                is_valid_utf8: true,
                error: None,
            })
        }
        Err(e) => {
            // If UTF-8 decoding fails, return the raw bytes as a lossy string
            let decoded_bytes = percent_decode_str(&to_decode).collect::<Vec<u8>>();
            let decoded_lossy = String::from_utf8_lossy(&decoded_bytes).to_string();

            Ok(UrlDecoderOutput {
                decoded_length: decoded_lossy.len(),
                decoded: decoded_lossy,
                encoded_length: input.encoded.len(),
                sequences_decoded,
                is_valid_utf8: false,
                error: Some(format!("Invalid UTF-8 sequence: {e}")),
            })
        }
    }
}

fn count_encoded_sequences(encoded: &str) -> usize {
    // Count %XX sequences
    let mut count = 0;
    let chars: Vec<char> = encoded.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '%' && i + 2 < chars.len() {
            // Check if the next two characters are valid hex digits
            if chars[i + 1].is_ascii_hexdigit() && chars[i + 2].is_ascii_hexdigit() {
                count += 1;
                i += 3; // Skip the %XX sequence
                continue;
            }
        }
        i += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_simple() {
        let input = UrlDecoderInput {
            encoded: "hello%20world".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello world");
        assert_eq!(result.sequences_decoded, 1);
        assert!(result.is_valid_utf8);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_decode_special_characters() {
        let input = UrlDecoderInput {
            encoded: "hello%40world.com".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello@world.com");
        assert_eq!(result.sequences_decoded, 1);
    }

    #[test]
    fn test_decode_with_plus() {
        let input = UrlDecoderInput {
            encoded: "hello+world".to_string(),
            decode_plus: Some(true),
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello world");
        assert_eq!(result.sequences_decoded, 0); // + is not a %XX sequence
    }

    #[test]
    fn test_decode_without_plus() {
        let input = UrlDecoderInput {
            encoded: "hello+world".to_string(),
            decode_plus: Some(false),
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello+world");
        assert_eq!(result.sequences_decoded, 0);
    }

    #[test]
    fn test_decode_unicode() {
        let input = UrlDecoderInput {
            encoded: "Hello%20%E4%B8%96%E7%95%8C".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "Hello 世界");
        assert_eq!(result.sequences_decoded, 7); // 1 space + 6 for unicode
        assert!(result.is_valid_utf8);
    }

    #[test]
    fn test_decode_reserved_characters() {
        let input = UrlDecoderInput {
            encoded: "%3Ffoo%3Dbar%26baz%3Dqux".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "?foo=bar&baz=qux");
        assert_eq!(result.sequences_decoded, 4);
    }

    #[test]
    fn test_decode_already_decoded() {
        let input = UrlDecoderInput {
            encoded: "hello world".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello world");
        assert_eq!(result.sequences_decoded, 0);
    }

    #[test]
    fn test_decode_double_encoded() {
        let input = UrlDecoderInput {
            encoded: "hello%2520world".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello%20world");
        assert_eq!(result.sequences_decoded, 1);
    }

    #[test]
    fn test_decode_newlines() {
        let input = UrlDecoderInput {
            encoded: "line1%0Aline2%0D%0Aline3".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "line1\nline2\r\nline3");
        assert_eq!(result.sequences_decoded, 3);
    }

    #[test]
    fn test_decode_empty_error() {
        let input = UrlDecoderInput {
            encoded: "".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Encoded data cannot be empty");
    }

    #[test]
    fn test_decode_invalid_sequences() {
        let input = UrlDecoderInput {
            encoded: "hello%2world%".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        // Invalid sequences are passed through unchanged
        assert_eq!(result.decoded, "hello%2world%");
        assert_eq!(result.sequences_decoded, 0);
    }

    #[test]
    fn test_decode_mixed_valid_invalid() {
        let input = UrlDecoderInput {
            encoded: "hello%20world%2".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.decoded, "hello world%2");
        assert_eq!(result.sequences_decoded, 1); // Only %20 is valid
    }

    #[test]
    fn test_length_calculations() {
        let input = UrlDecoderInput {
            encoded: "a%20b%20c".to_string(),
            decode_plus: None,
        };

        let result = decode_url(input).unwrap();
        assert_eq!(result.encoded_length, 9);
        assert_eq!(result.decoded_length, 5); // "a b c"
        assert_eq!(result.sequences_decoded, 2);
    }
}
