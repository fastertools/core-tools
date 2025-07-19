use percent_encoding::{AsciiSet, CONTROLS, NON_ALPHANUMERIC, utf8_percent_encode};
use serde::{Deserialize, Serialize};

// Define different encoding sets
const QUERY_FRAGMENT_SET: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

const PATH_SEGMENT_SET: &AsciiSet = &QUERY_FRAGMENT_SET.add(b'#').add(b'?').add(b'{').add(b'}');

const USERINFO_SET: &AsciiSet = &PATH_SEGMENT_SET
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'|');

const COMPONENT_SET: &AsciiSet = &USERINFO_SET
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'+')
    .add(b',');

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEncoderInput {
    /// The string to encode
    pub data: String,
    /// Encoding mode (optional, default: "component")
    /// Options: "component", "path", "query", "full"
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEncoderOutput {
    /// URL encoded string
    pub encoded: String,
    /// Original data length
    pub original_length: usize,
    /// Encoded length
    pub encoded_length: usize,
    /// Encoding mode used
    pub mode: String,
    /// Number of characters encoded
    pub chars_encoded: usize,
}

pub fn encode_url(input: UrlEncoderInput) -> Result<UrlEncoderOutput, String> {
    if input.data.is_empty() {
        return Err("Data cannot be empty".to_string());
    }

    let mode = input.mode.unwrap_or_else(|| "component".to_string());

    // Select the appropriate encoding set based on mode
    let (encoded, set_name) = match mode.as_str() {
        "component" => {
            let encoded = utf8_percent_encode(&input.data, COMPONENT_SET).to_string();
            (encoded, "component")
        }
        "path" => {
            let encoded = utf8_percent_encode(&input.data, PATH_SEGMENT_SET).to_string();
            (encoded, "path")
        }
        "query" => {
            let encoded = utf8_percent_encode(&input.data, QUERY_FRAGMENT_SET).to_string();
            (encoded, "query")
        }
        "full" => {
            // Full encoding encodes all non-alphanumeric characters
            let encoded = utf8_percent_encode(&input.data, NON_ALPHANUMERIC).to_string();
            (encoded, "full")
        }
        _ => {
            return Err(format!(
                "Invalid mode '{}'. Valid modes are: component, path, query, full",
                mode
            ));
        }
    };

    // Count how many characters were encoded
    let chars_encoded = count_encoded_chars(&input.data, &encoded);

    Ok(UrlEncoderOutput {
        encoded_length: encoded.len(),
        encoded,
        original_length: input.data.len(),
        mode: set_name.to_string(),
        chars_encoded,
    })
}

fn count_encoded_chars(_original: &str, encoded: &str) -> usize {
    // Count the number of '%' characters in the encoded string
    // Each encoded character becomes %XX (3 characters)
    encoded.matches('%').count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_simple_text() {
        let input = UrlEncoderInput {
            data: "hello world".to_string(),
            mode: None,
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "hello%20world");
        assert_eq!(result.mode, "component");
        assert_eq!(result.chars_encoded, 1); // space encoded
    }

    #[test]
    fn test_encode_special_characters() {
        let input = UrlEncoderInput {
            data: "hello@world.com".to_string(),
            mode: Some("component".to_string()),
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "hello%40world.com");
        assert_eq!(result.chars_encoded, 1); // @ encoded
    }

    #[test]
    fn test_encode_path_mode() {
        let input = UrlEncoderInput {
            data: "path/to/file name.txt".to_string(),
            mode: Some("path".to_string()),
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "path/to/file%20name.txt");
        assert_eq!(result.mode, "path");
        assert_eq!(result.chars_encoded, 1); // only space encoded
    }

    #[test]
    fn test_encode_query_mode() {
        let input = UrlEncoderInput {
            data: "name=John Doe&age=30".to_string(),
            mode: Some("query".to_string()),
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "name=John%20Doe&age=30");
        assert_eq!(result.mode, "query");
        assert_eq!(result.chars_encoded, 1); // only space encoded
    }

    #[test]
    fn test_encode_full_mode() {
        let input = UrlEncoderInput {
            data: "hello-world_123.txt".to_string(),
            mode: Some("full".to_string()),
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "hello%2Dworld%5F123%2Etxt");
        assert_eq!(result.mode, "full");
        assert_eq!(result.chars_encoded, 3); // -, _, . encoded
    }

    #[test]
    fn test_encode_unicode() {
        let input = UrlEncoderInput {
            data: "Hello 世界".to_string(),
            mode: None,
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "Hello%20%E4%B8%96%E7%95%8C");
        assert!(result.chars_encoded > 1); // space and unicode chars encoded
    }

    #[test]
    fn test_encode_reserved_characters() {
        let input = UrlEncoderInput {
            data: "?foo=bar&baz=qux".to_string(),
            mode: Some("component".to_string()),
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "%3Ffoo%3Dbar%26baz%3Dqux");
        assert_eq!(result.chars_encoded, 4); // ?, =, &, = encoded
    }

    #[test]
    fn test_encode_empty_error() {
        let input = UrlEncoderInput {
            data: "".to_string(),
            mode: None,
        };

        let result = encode_url(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Data cannot be empty");
    }

    #[test]
    fn test_invalid_mode_error() {
        let input = UrlEncoderInput {
            data: "test".to_string(),
            mode: Some("invalid".to_string()),
        };

        let result = encode_url(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid mode"));
    }

    #[test]
    fn test_encode_already_encoded() {
        let input = UrlEncoderInput {
            data: "hello%20world".to_string(),
            mode: None,
        };

        let result = encode_url(input).unwrap();
        // Should double-encode the %
        assert_eq!(result.encoded, "hello%2520world");
    }

    #[test]
    fn test_encode_newlines() {
        let input = UrlEncoderInput {
            data: "line1\nline2\r\nline3".to_string(),
            mode: None,
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.encoded, "line1%0Aline2%0D%0Aline3");
        assert_eq!(result.chars_encoded, 3); // \n, \r, \n encoded
    }

    #[test]
    fn test_length_calculations() {
        let input = UrlEncoderInput {
            data: "a b c".to_string(),
            mode: None,
        };

        let result = encode_url(input).unwrap();
        assert_eq!(result.original_length, 5);
        assert_eq!(result.encoded_length, 9); // "a%20b%20c"
        assert_eq!(result.chars_encoded, 2);
    }
}
