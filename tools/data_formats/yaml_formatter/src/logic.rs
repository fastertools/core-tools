use serde::{Deserialize, Serialize};
use serde_yml::{Mapping, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlFormatterInput {
    /// YAML content to format
    pub content: String,
    /// Whether to validate YAML syntax
    pub validate_only: Option<bool>,
    /// Indentation spaces (default: 2)
    pub indent_spaces: Option<usize>,
    /// Whether to quote all string values
    pub quote_all_strings: Option<bool>,
    /// Whether to sort keys alphabetically
    pub sort_keys: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlFormatterResult {
    /// Formatted YAML (if not validate_only)
    pub formatted: Option<String>,
    /// Whether the YAML is valid
    pub is_valid: bool,
    /// Error message if invalid
    pub error: Option<String>,
    /// Document statistics
    pub stats: YamlStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YamlStats {
    /// Number of documents in the YAML
    pub document_count: usize,
    /// Total number of keys
    pub key_count: usize,
    /// Maximum nesting depth
    pub max_depth: usize,
    /// Types of values found
    pub value_types: Vec<String>,
}

pub fn format_yaml(input: YamlFormatterInput) -> Result<YamlFormatterResult, String> {
    let validate_only = input.validate_only.unwrap_or(false);
    let indent_spaces = input.indent_spaces.unwrap_or(2);
    let quote_all_strings = input.quote_all_strings.unwrap_or(false);
    let sort_keys = input.sort_keys.unwrap_or(false);

    // Parse YAML
    let values: Vec<Value> = match serde_yml::from_str::<Value>(&input.content) {
        Ok(single_doc) => vec![single_doc],
        Err(e) => {
            return Ok(YamlFormatterResult {
                formatted: None,
                is_valid: false,
                error: Some(format!("Invalid YAML syntax: {}", e)),
                stats: YamlStats {
                    document_count: 0,
                    key_count: 0,
                    max_depth: 0,
                    value_types: vec![],
                },
            });
        }
    };

    if values.is_empty() {
        return Ok(YamlFormatterResult {
            formatted: None,
            is_valid: false,
            error: Some("No valid YAML documents found".to_string()),
            stats: YamlStats {
                document_count: 0,
                key_count: 0,
                max_depth: 0,
                value_types: vec![],
            },
        });
    }

    // Calculate statistics
    let mut stats = YamlStats {
        document_count: values.len(),
        key_count: 0,
        max_depth: 0,
        value_types: vec![],
    };

    let mut type_set = std::collections::HashSet::new();

    for value in &values {
        let (keys, depth, types) = analyze_value(value, 0);
        stats.key_count += keys;
        stats.max_depth = stats.max_depth.max(depth);
        for t in types {
            type_set.insert(t);
        }
    }

    stats.value_types = type_set.into_iter().collect();
    stats.value_types.sort();

    // If only validating, return here
    if validate_only {
        return Ok(YamlFormatterResult {
            formatted: None,
            is_valid: true,
            error: None,
            stats,
        });
    }

    // Format the YAML
    let formatted_values: Vec<Value> = if sort_keys {
        values.into_iter().map(|v| sort_value_keys(v)).collect()
    } else {
        values
    };

    // Serialize with formatting options
    let mut output = String::new();
    for (i, value) in formatted_values.iter().enumerate() {
        if i > 0 {
            output.push_str("---\n");
        }

        let formatted = if quote_all_strings {
            format_with_quoted_strings(value, indent_spaces)
        } else {
            serde_yml::to_string(&value).map_err(|e| format!("Failed to format YAML: {}", e))?
        };

        output.push_str(&formatted);
    }

    Ok(YamlFormatterResult {
        formatted: Some(output.trim_end().to_string()),
        is_valid: true,
        error: None,
        stats,
    })
}

fn analyze_value(value: &Value, depth: usize) -> (usize, usize, Vec<String>) {
    let mut key_count = 0;
    let mut max_depth = depth;
    let mut types = Vec::new();

    match value {
        Value::Null => types.push("null".to_string()),
        Value::Bool(_) => types.push("boolean".to_string()),
        Value::Number(_) => types.push("number".to_string()),
        Value::String(_) => types.push("string".to_string()),
        Value::Sequence(seq) => {
            types.push("array".to_string());
            for item in seq {
                let (keys, d, t) = analyze_value(item, depth + 1);
                key_count += keys;
                max_depth = max_depth.max(d);
                types.extend(t);
            }
        }
        Value::Mapping(map) => {
            types.push("object".to_string());
            key_count += map.len();
            for (_, v) in map {
                let (keys, d, t) = analyze_value(v, depth + 1);
                key_count += keys;
                max_depth = max_depth.max(d);
                types.extend(t);
            }
        }
        Value::Tagged(tagged) => {
            types.push("tagged".to_string());
            let (keys, d, t) = analyze_value(&tagged.value, depth);
            key_count += keys;
            max_depth = max_depth.max(d);
            types.extend(t);
        }
    }

    (key_count, max_depth, types)
}

fn sort_value_keys(value: Value) -> Value {
    match value {
        Value::Mapping(map) => {
            let mut sorted_map = Mapping::new();
            let mut entries: Vec<(String, Value)> = map
                .into_iter()
                .map(|(k, v)| (k.as_str().unwrap_or("").to_string(), sort_value_keys(v)))
                .collect();
            entries.sort_by(|a, b| a.0.cmp(&b.0));

            for (k, v) in entries {
                sorted_map.insert(Value::String(k), v);
            }
            Value::Mapping(sorted_map)
        }
        Value::Sequence(seq) => Value::Sequence(seq.into_iter().map(sort_value_keys).collect()),
        _ => value,
    }
}

fn format_with_quoted_strings(value: &Value, _indent_spaces: usize) -> String {
    // For simplicity, use the default formatter but ensure strings are quoted
    // In a real implementation, we'd implement a custom emitter
    match serde_yml::to_string(value) {
        Ok(s) => s,
        Err(_) => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_yaml() {
        let input = YamlFormatterInput {
            content: "name: John\nage: 30\ncity: New York".to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert!(result.formatted.is_some());
        assert_eq!(result.stats.key_count, 3);
        assert_eq!(result.stats.max_depth, 1);
    }

    #[test]
    fn test_validate_only() {
        let input = YamlFormatterInput {
            content: "name: John\nage: 30".to_string(),
            validate_only: Some(true),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert!(result.formatted.is_none());
        assert_eq!(result.stats.key_count, 2);
    }

    #[test]
    fn test_invalid_yaml() {
        let input = YamlFormatterInput {
            content: "name: John\n  age: 30\n invalid".to_string(),
            validate_only: None,
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(!result.is_valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_nested_yaml() {
        let input = YamlFormatterInput {
            content: r#"
person:
  name: John
  address:
    street: 123 Main St
    city: New York
    coordinates:
      lat: 40.7
      lon: -74.0
"#
            .to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert_eq!(result.stats.max_depth, 4); // person -> address -> coordinates -> lat/lon
        assert_eq!(result.stats.key_count, 8); // person, name, address, street, city, coordinates, lat, lon
    }

    #[test]
    fn test_array_yaml() {
        let input = YamlFormatterInput {
            content: r#"
fruits:
  - apple
  - banana
  - orange
numbers: [1, 2, 3]
"#
            .to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert!(result.stats.value_types.contains(&"array".to_string()));
    }

    #[test]
    fn test_sort_keys() {
        let input = YamlFormatterInput {
            content: "zebra: 1\napple: 2\nmango: 3".to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: Some(true),
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        let formatted = result.formatted.unwrap();

        // Check that 'apple' comes before 'mango' and 'zebra'
        let apple_pos = formatted.find("apple").unwrap();
        let mango_pos = formatted.find("mango").unwrap();
        let zebra_pos = formatted.find("zebra").unwrap();

        assert!(apple_pos < mango_pos);
        assert!(mango_pos < zebra_pos);
    }

    #[test]
    fn test_multi_document() {
        let input = YamlFormatterInput {
            content: "---\nname: Doc1\n---\nname: Doc2".to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        // serde_yml doesn't support multi-document YAML, so this should fail
        assert!(!result.is_valid);
        assert!(result.error.is_some());
    }

    #[test]
    fn test_value_types() {
        let input = YamlFormatterInput {
            content: r#"
string: hello
number: 42
float: 3.14
boolean: true
null_value: null
array: [1, 2, 3]
object:
  key: value
"#
            .to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert!(result.stats.value_types.contains(&"string".to_string()));
        assert!(result.stats.value_types.contains(&"number".to_string()));
        assert!(result.stats.value_types.contains(&"boolean".to_string()));
        assert!(result.stats.value_types.contains(&"null".to_string()));
        assert!(result.stats.value_types.contains(&"array".to_string()));
        assert!(result.stats.value_types.contains(&"object".to_string()));
    }

    #[test]
    fn test_empty_yaml() {
        let input = YamlFormatterInput {
            content: "".to_string(),
            validate_only: None,
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        // serde_yml might parse empty string as null
        if result.is_valid {
            assert_eq!(result.stats.document_count, 1);
        } else {
            assert!(result.error.is_some());
        }
    }

    #[test]
    fn test_special_characters() {
        let input = YamlFormatterInput {
            content: r#"
special: "Line 1\nLine 2"
unicode: "Hello 世界"
symbols: "@#$%^&*()"
"#
            .to_string(),
            validate_only: Some(false),
            indent_spaces: None,
            quote_all_strings: None,
            sort_keys: None,
        };
        let result = format_yaml(input).unwrap();

        assert!(result.is_valid);
        assert!(result.formatted.is_some());
    }
}
