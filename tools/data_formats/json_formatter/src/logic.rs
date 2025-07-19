use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonFormatterInput {
    /// JSON string to format
    pub json_string: String,
    /// Number of spaces for indentation (0 for compact)
    pub indent: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonFormatterResult {
    /// Formatted JSON string
    pub formatted: String,
    /// Whether the input was valid JSON
    pub is_valid: bool,
    /// Error message if parsing failed
    pub error: Option<String>,
    /// Number of characters in input
    pub input_length: usize,
    /// Number of characters in output
    pub output_length: usize,
}

pub fn format_json(input: JsonFormatterInput) -> Result<JsonFormatterResult, String> {
    let input_length = input.json_string.len();

    // Try to parse the JSON
    let parsed: serde_json::Value = match serde_json::from_str(&input.json_string) {
        Ok(val) => val,
        Err(e) => {
            return Ok(JsonFormatterResult {
                formatted: input.json_string.clone(),
                is_valid: false,
                error: Some(format!("JSON parse error: {}", e)),
                input_length,
                output_length: input_length,
            });
        }
    };

    // Format based on indent preference
    let formatted = match input.indent {
        Some(0) => {
            // Compact format
            match serde_json::to_string(&parsed) {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to serialize JSON: {}", e)),
            }
        }
        Some(n) => {
            // Pretty format with custom indent
            let mut writer = Vec::new();
            let indent_string = " ".repeat(n);
            let formatter = serde_json::ser::PrettyFormatter::with_indent(indent_string.as_bytes());
            let mut ser = serde_json::Serializer::with_formatter(&mut writer, formatter);
            if let Err(e) = parsed.serialize(&mut ser) {
                return Err(format!("Failed to serialize JSON: {}", e));
            }
            match String::from_utf8(writer) {
                Ok(s) => s,
                Err(e) => return Err(format!("UTF-8 conversion error: {}", e)),
            }
        }
        None => {
            // Default pretty format (2 spaces)
            match serde_json::to_string_pretty(&parsed) {
                Ok(s) => s,
                Err(e) => return Err(format!("Failed to serialize JSON: {}", e)),
            }
        }
    };

    let output_length = formatted.len();

    Ok(JsonFormatterResult {
        formatted,
        is_valid: true,
        error: None,
        input_length,
        output_length,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let input = JsonFormatterInput {
            json_string: r#"{"name":"John","age":30,"city":"New York"}"#.to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("  \"name\": \"John\""));
        assert!(result.formatted.contains("  \"age\": 30"));
        assert!(result.formatted.contains("  \"city\": \"New York\""));
        assert_eq!(result.error, None);
    }

    #[test]
    fn test_compact_formatting() {
        let input = JsonFormatterInput {
            json_string: r#"{ "name" : "John" , "age" : 30 }"#.to_string(),
            indent: Some(0),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        // JSON object key order is not guaranteed, so check both possibilities
        assert!(
            result.formatted == r#"{"name":"John","age":30}"#
                || result.formatted == r#"{"age":30,"name":"John"}"#
        );
    }

    #[test]
    fn test_custom_indent() {
        let input = JsonFormatterInput {
            json_string: r#"{"a":{"b":"c"}}"#.to_string(),
            indent: Some(4),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("    \"a\""));
        assert!(result.formatted.contains("        \"b\""));
    }

    #[test]
    fn test_invalid_json() {
        let input = JsonFormatterInput {
            json_string: r#"{"name": "John", "age": }"#.to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert!(!result.is_valid);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("JSON parse error"));
    }

    #[test]
    fn test_array_formatting() {
        let input = JsonFormatterInput {
            json_string: r#"[1,2,3,{"a":"b"}]"#.to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("[\n  1,\n  2,\n  3,"));
        assert!(result.formatted.contains("    \"a\": \"b\""));
    }

    #[test]
    fn test_nested_objects() {
        let input = JsonFormatterInput {
            json_string: r#"{"level1":{"level2":{"level3":"value"}}}"#.to_string(),
            indent: Some(2),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("  \"level1\":"));
        assert!(result.formatted.contains("    \"level2\":"));
        assert!(result.formatted.contains("      \"level3\":"));
    }

    #[test]
    fn test_empty_json() {
        let input = JsonFormatterInput {
            json_string: "{}".to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.formatted, "{}");
    }

    #[test]
    fn test_null_value() {
        let input = JsonFormatterInput {
            json_string: r#"{"value": null}"#.to_string(),
            indent: Some(0),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.formatted, r#"{"value":null}"#);
    }

    #[test]
    fn test_boolean_values() {
        let input = JsonFormatterInput {
            json_string: r#"{"active": true, "deleted": false}"#.to_string(),
            indent: Some(0),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.formatted, r#"{"active":true,"deleted":false}"#);
    }

    #[test]
    fn test_number_precision() {
        let input = JsonFormatterInput {
            json_string: r#"{"pi": 3.14159265359, "e": 2.71828182846}"#.to_string(),
            indent: Some(0),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("3.14159265359"));
        assert!(result.formatted.contains("2.71828182846"));
    }

    #[test]
    fn test_escape_sequences() {
        let input = JsonFormatterInput {
            json_string: r#"{"quote": "He said \"Hello\"", "newline": "Line1\nLine2"}"#.to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains(r#"\"Hello\""#));
        assert!(result.formatted.contains(r#"\n"#));
    }

    #[test]
    fn test_unicode_support() {
        let input = JsonFormatterInput {
            json_string: r#"{"emoji": "🎉", "chinese": "你好", "arabic": "مرحبا"}"#.to_string(),
            indent: Some(0),
        };
        let result = format_json(input).unwrap();
        assert!(result.is_valid);
        assert!(result.formatted.contains("🎉"));
        assert!(result.formatted.contains("你好"));
        assert!(result.formatted.contains("مرحبا"));
    }

    #[test]
    fn test_length_tracking() {
        let input = JsonFormatterInput {
            json_string: r#"{"a":1}"#.to_string(),
            indent: None,
        };
        let result = format_json(input).unwrap();
        assert_eq!(result.input_length, 7);
        assert!(result.output_length > 7); // Pretty format adds whitespace
    }
}
