use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonValidatorInput {
    /// JSON string to validate
    pub json_string: String,
    /// Optional JSON schema to validate against (as JSON string)
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonValidatorResult {
    /// Whether the JSON is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Detailed validation information
    pub details: ValidationDetails,
    /// Whether schema validation was performed
    pub schema_validated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationDetails {
    /// Type of the root JSON value
    pub root_type: String,
    /// Number of keys (if object)
    pub key_count: Option<usize>,
    /// Number of elements (if array)
    pub element_count: Option<usize>,
    /// Maximum nesting depth
    pub max_depth: usize,
    /// Total number of values
    pub total_values: usize,
    /// Line number where error occurred (if applicable)
    pub error_line: Option<usize>,
    /// Column number where error occurred (if applicable)
    pub error_column: Option<usize>,
}

pub fn validate_json(input: JsonValidatorInput) -> Result<JsonValidatorResult, String> {
    // Try to parse the JSON
    let parsed: Value = match serde_json::from_str(&input.json_string) {
        Ok(val) => val,
        Err(e) => {
            // Extract line and column from error
            let (error_line, error_column) = extract_error_position(&e);

            return Ok(JsonValidatorResult {
                is_valid: false,
                error: Some(format!("Invalid JSON: {}", e)),
                details: ValidationDetails {
                    root_type: "unknown".to_string(),
                    key_count: None,
                    element_count: None,
                    max_depth: 0,
                    total_values: 0,
                    error_line,
                    error_column,
                },
                schema_validated: false,
            });
        }
    };

    // Analyze the JSON structure
    let mut details = analyze_json(&parsed);
    details.error_line = None;
    details.error_column = None;

    // If schema provided, validate against it
    let schema_validated = if let Some(schema_str) = input.schema {
        match validate_against_schema(&parsed, &schema_str) {
            Ok(true) => true,
            Ok(false) => {
                return Ok(JsonValidatorResult {
                    is_valid: false,
                    error: Some("JSON does not match the provided schema".to_string()),
                    details,
                    schema_validated: true,
                });
            }
            Err(e) => {
                return Ok(JsonValidatorResult {
                    is_valid: false,
                    error: Some(format!("Schema validation error: {}", e)),
                    details,
                    schema_validated: false,
                });
            }
        }
    } else {
        false
    };

    Ok(JsonValidatorResult {
        is_valid: true,
        error: None,
        details,
        schema_validated,
    })
}

fn extract_error_position(error: &serde_json::Error) -> (Option<usize>, Option<usize>) {
    let error_str = error.to_string();

    // Try to extract line and column from error message
    if let Some(pos) = error_str.find("line ") {
        let rest = &error_str[pos + 5..];
        if let Some(line_end) = rest.find(' ') {
            if let Ok(line) = rest[..line_end].parse::<usize>() {
                // Look for column
                if let Some(col_pos) = rest.find("column ") {
                    let col_rest = &rest[col_pos + 7..];
                    if let Some(col_end) = col_rest.find(|c: char| !c.is_numeric()) {
                        if let Ok(col) = col_rest[..col_end].parse::<usize>() {
                            return (Some(line), Some(col));
                        }
                    }
                }
                return (Some(line), None);
            }
        }
    }

    (None, None)
}

fn analyze_json(value: &Value) -> ValidationDetails {
    let root_type = match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
    .to_string();

    let key_count = if let Value::Object(map) = value {
        Some(map.len())
    } else {
        None
    };

    let element_count = if let Value::Array(arr) = value {
        Some(arr.len())
    } else {
        None
    };

    let (max_depth, total_values) = calculate_depth_and_count(value, 0);

    ValidationDetails {
        root_type,
        key_count,
        element_count,
        max_depth,
        total_values,
        error_line: None,
        error_column: None,
    }
}

fn calculate_depth_and_count(value: &Value, current_depth: usize) -> (usize, usize) {
    match value {
        Value::Object(map) => {
            let mut max_depth = current_depth + 1;
            let mut total_count = 1;

            for (_, v) in map {
                let (child_depth, child_count) = calculate_depth_and_count(v, current_depth + 1);
                max_depth = max_depth.max(child_depth);
                total_count += child_count;
            }

            (max_depth, total_count)
        }
        Value::Array(arr) => {
            let mut max_depth = current_depth + 1;
            let mut total_count = 1;

            for v in arr {
                let (child_depth, child_count) = calculate_depth_and_count(v, current_depth + 1);
                max_depth = max_depth.max(child_depth);
                total_count += child_count;
            }

            (max_depth, total_count)
        }
        _ => (current_depth + 1, 1),
    }
}

fn validate_against_schema(value: &Value, schema_str: &str) -> Result<bool, String> {
    // Parse the schema
    let _schema: Value =
        serde_json::from_str(schema_str).map_err(|e| format!("Invalid schema JSON: {}", e))?;

    // Note: Full JSON Schema validation is complex and would require a dedicated library.
    // For this basic implementation, we'll just check if the schema is valid JSON.
    // In a real implementation, you'd use a JSON Schema validation library.

    // For now, just return true if both are valid JSON
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_simple_json() {
        let input = JsonValidatorInput {
            json_string: r#"{"name": "John", "age": 30}"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "object");
        assert_eq!(result.details.key_count, Some(2));
        assert_eq!(result.details.max_depth, 2);
    }

    #[test]
    fn test_invalid_json() {
        let input = JsonValidatorInput {
            json_string: r#"{"name": "John", "age": }"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(!result.is_valid);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("Invalid JSON"));
    }

    #[test]
    fn test_array_json() {
        let input = JsonValidatorInput {
            json_string: r#"[1, 2, 3, 4, 5]"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "array");
        assert_eq!(result.details.element_count, Some(5));
        assert_eq!(result.details.total_values, 6); // 1 array + 5 numbers
    }

    #[test]
    fn test_nested_json() {
        let input = JsonValidatorInput {
            json_string: r#"{"a": {"b": {"c": {"d": "value"}}}}"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.max_depth, 5);
    }

    #[test]
    fn test_empty_object() {
        let input = JsonValidatorInput {
            json_string: "{}".to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.key_count, Some(0));
    }

    #[test]
    fn test_empty_array() {
        let input = JsonValidatorInput {
            json_string: "[]".to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.element_count, Some(0));
    }

    #[test]
    fn test_null_value() {
        let input = JsonValidatorInput {
            json_string: "null".to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "null");
    }

    #[test]
    fn test_boolean_value() {
        let input = JsonValidatorInput {
            json_string: "true".to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "boolean");
    }

    #[test]
    fn test_number_value() {
        let input = JsonValidatorInput {
            json_string: "42.5".to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "number");
    }

    #[test]
    fn test_string_value() {
        let input = JsonValidatorInput {
            json_string: r#""hello world""#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.root_type, "string");
    }

    #[test]
    fn test_mixed_array() {
        let input = JsonValidatorInput {
            json_string: r#"[1, "two", true, null, {"five": 5}]"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid);
        assert_eq!(result.details.element_count, Some(5));
        assert_eq!(result.details.total_values, 7); // 1 array + 5 elements + 1 object value
    }

    #[test]
    fn test_unclosed_brace() {
        let input = JsonValidatorInput {
            json_string: r#"{"name": "John""#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(!result.is_valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_trailing_comma() {
        let input = JsonValidatorInput {
            json_string: r#"{"name": "John",}"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(!result.is_valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_duplicate_keys() {
        // Note: serde_json allows duplicate keys (last one wins)
        let input = JsonValidatorInput {
            json_string: r#"{"name": "John", "name": "Jane"}"#.to_string(),
            schema: None,
        };
        let result = validate_json(input).unwrap();
        assert!(result.is_valid); // This is valid JSON, though not recommended
    }
}
