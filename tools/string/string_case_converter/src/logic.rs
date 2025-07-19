use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToUpperCamelCase,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringCaseConverterInput {
    /// The text to convert
    pub text: String,
    /// Target case format
    /// Options: "lower", "upper", "title", "sentence", "camelCase", "PascalCase",
    /// "snake_case", "SCREAMING_SNAKE_CASE", "kebab-case", "SCREAMING-KEBAB-CASE"
    pub target_case: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StringCaseConverterOutput {
    /// Converted text
    pub converted: String,
    /// Original text
    pub original: String,
    /// Target case used
    pub target_case: String,
    /// Whether conversion was applied (false if already in target case)
    pub changed: bool,
}

pub fn convert_case(input: StringCaseConverterInput) -> Result<StringCaseConverterOutput, String> {
    if input.text.is_empty() {
        return Err("Text cannot be empty".to_string());
    }

    let converted = match input.target_case.as_str() {
        "lower" => input.text.to_lowercase(),
        "upper" => input.text.to_uppercase(),
        "title" => input.text.to_title_case(),
        "sentence" => to_sentence_case(&input.text),
        "camelCase" => input.text.to_lower_camel_case(),
        "PascalCase" => input.text.to_upper_camel_case(),
        "snake_case" => input.text.to_snake_case(),
        "SCREAMING_SNAKE_CASE" => input.text.to_shouty_snake_case(),
        "kebab-case" => input.text.to_kebab_case(),
        "SCREAMING-KEBAB-CASE" => input.text.to_shouty_kebab_case(),
        _ => {
            return Err(format!(
                "Invalid target_case '{}'. Valid options are: lower, upper, title, sentence, \
                camelCase, PascalCase, snake_case, SCREAMING_SNAKE_CASE, kebab-case, SCREAMING-KEBAB-CASE",
                input.target_case
            ));
        }
    };

    let changed = converted != input.text;

    Ok(StringCaseConverterOutput {
        original: input.text.clone(),
        converted,
        target_case: input.target_case,
        changed,
    })
}

fn to_sentence_case(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_uppercase().collect::<String>();
            result.push_str(&chars.as_str().to_lowercase());
            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_lowercase() {
        let input = StringCaseConverterInput {
            text: "HELLO World".to_string(),
            target_case: "lower".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "hello world");
        assert_eq!(result.target_case, "lower");
        assert!(result.changed);
    }

    #[test]
    fn test_to_uppercase() {
        let input = StringCaseConverterInput {
            text: "hello world".to_string(),
            target_case: "upper".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "HELLO WORLD");
        assert!(result.changed);
    }

    #[test]
    fn test_to_title_case() {
        let input = StringCaseConverterInput {
            text: "hello world from rust".to_string(),
            target_case: "title".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "Hello World From Rust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_sentence_case() {
        let input = StringCaseConverterInput {
            text: "hello WORLD from RUST".to_string(),
            target_case: "sentence".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "Hello world from rust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_camel_case() {
        let input = StringCaseConverterInput {
            text: "hello_world_from_rust".to_string(),
            target_case: "camelCase".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "helloWorldFromRust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_pascal_case() {
        let input = StringCaseConverterInput {
            text: "hello_world_from_rust".to_string(),
            target_case: "PascalCase".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "HelloWorldFromRust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_snake_case() {
        let input = StringCaseConverterInput {
            text: "HelloWorldFromRust".to_string(),
            target_case: "snake_case".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "hello_world_from_rust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_screaming_snake_case() {
        let input = StringCaseConverterInput {
            text: "helloWorldFromRust".to_string(),
            target_case: "SCREAMING_SNAKE_CASE".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "HELLO_WORLD_FROM_RUST");
        assert!(result.changed);
    }

    #[test]
    fn test_to_kebab_case() {
        let input = StringCaseConverterInput {
            text: "HelloWorldFromRust".to_string(),
            target_case: "kebab-case".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "hello-world-from-rust");
        assert!(result.changed);
    }

    #[test]
    fn test_to_screaming_kebab_case() {
        let input = StringCaseConverterInput {
            text: "helloWorldFromRust".to_string(),
            target_case: "SCREAMING-KEBAB-CASE".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "HELLO-WORLD-FROM-RUST");
        assert!(result.changed);
    }

    #[test]
    fn test_no_change() {
        let input = StringCaseConverterInput {
            text: "hello world".to_string(),
            target_case: "lower".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "hello world");
        assert!(!result.changed);
    }

    #[test]
    fn test_empty_text_error() {
        let input = StringCaseConverterInput {
            text: "".to_string(),
            target_case: "lower".to_string(),
        };

        let result = convert_case(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Text cannot be empty");
    }

    #[test]
    fn test_invalid_case_error() {
        let input = StringCaseConverterInput {
            text: "test".to_string(),
            target_case: "invalid".to_string(),
        };

        let result = convert_case(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid target_case"));
    }

    #[test]
    fn test_mixed_input_to_various_cases() {
        let text = "Hello-World_from RUST";

        let cases = vec![
            ("camelCase", "helloWorldFromRust"),
            ("PascalCase", "HelloWorldFromRust"),
            ("snake_case", "hello_world_from_rust"),
            ("kebab-case", "hello-world-from-rust"),
        ];

        for (target, expected) in cases {
            let input = StringCaseConverterInput {
                text: text.to_string(),
                target_case: target.to_string(),
            };

            let result = convert_case(input).unwrap();
            assert_eq!(result.converted, expected);
        }
    }

    #[test]
    fn test_numbers_and_special_chars() {
        let input = StringCaseConverterInput {
            text: "hello123world456".to_string(),
            target_case: "snake_case".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "hello123world456");
    }

    #[test]
    fn test_unicode_support() {
        let input = StringCaseConverterInput {
            text: "hello 世界".to_string(),
            target_case: "upper".to_string(),
        };

        let result = convert_case(input).unwrap();
        assert_eq!(result.converted, "HELLO 世界");
    }
}
