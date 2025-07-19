use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringTrimInput {
    pub text: String,

    #[serde(default = "default_operation")]
    pub operation: String,

    #[serde(default)]
    pub char_to_trim: Option<String>,

    #[serde(default)]
    pub pad_length: Option<usize>,

    #[serde(default = "default_pad_char")]
    pub pad_char: String,

    #[serde(default = "default_pad_side")]
    pub pad_side: String,
}

fn default_operation() -> String {
    "trim".to_string()
}

fn default_pad_char() -> String {
    " ".to_string()
}

fn default_pad_side() -> String {
    "right".to_string()
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringTrimResult {
    pub original: String,
    pub processed: String,
    pub operation: String,
    pub length_before: usize,
    pub length_after: usize,
}

pub fn process_string(input: StringTrimInput) -> Result<StringTrimResult, String> {
    let original = input.text.clone();
    let length_before = original.len();

    let processed = match input.operation.as_str() {
        "trim" => original.trim().to_string(),

        "trim_start" => original.trim_start().to_string(),

        "trim_end" => original.trim_end().to_string(),

        "trim_char" => {
            if let Some(ch) = input.char_to_trim.as_ref().and_then(|s| s.chars().next()) {
                original.trim_matches(ch).to_string()
            } else {
                return Err("char_to_trim must be provided for trim_char operation".to_string());
            }
        }

        "trim_char_start" => {
            if let Some(ch) = input.char_to_trim.as_ref().and_then(|s| s.chars().next()) {
                original.trim_start_matches(ch).to_string()
            } else {
                return Err(
                    "char_to_trim must be provided for trim_char_start operation".to_string(),
                );
            }
        }

        "trim_char_end" => {
            if let Some(ch) = input.char_to_trim.as_ref().and_then(|s| s.chars().next()) {
                original.trim_end_matches(ch).to_string()
            } else {
                return Err("char_to_trim must be provided for trim_char_end operation".to_string());
            }
        }

        "pad" | "pad_left" | "pad_right" | "pad_center" => {
            let pad_length = input
                .pad_length
                .ok_or("pad_length must be provided for padding operations")?;

            if pad_length < original.len() {
                original.clone()
            } else {
                let pad_char = input.pad_char.chars().next().unwrap_or(' ');

                match input.operation.as_str() {
                    "pad" | "pad_right" => {
                        let mut result = original.clone();
                        while result.len() < pad_length {
                            result.push(pad_char);
                        }
                        result
                    }
                    "pad_left" => {
                        let pad_count = pad_length - original.len();
                        let padding = pad_char.to_string().repeat(pad_count);
                        format!("{padding}{original}")
                    }
                    "pad_center" => {
                        let total_pad = pad_length - original.len();
                        let left_pad = total_pad / 2;
                        let right_pad = total_pad - left_pad;
                        let left_padding = pad_char.to_string().repeat(left_pad);
                        let right_padding = pad_char.to_string().repeat(right_pad);
                        format!("{left_padding}{original}{right_padding}")
                    }
                    _ => unreachable!(),
                }
            }
        }

        _ => {
            return Err(format!(
                "Unknown operation: {}. Valid operations: trim, trim_start, trim_end, trim_char, trim_char_start, trim_char_end, pad, pad_left, pad_right, pad_center",
                input.operation
            ));
        }
    };

    let length_after = processed.len();

    Ok(StringTrimResult {
        original,
        processed,
        operation: input.operation,
        length_before,
        length_after,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trim() {
        let input = StringTrimInput {
            text: "  hello world  ".to_string(),
            operation: "trim".to_string(),
            char_to_trim: None,
            pad_length: None,
            pad_char: " ".to_string(),
            pad_side: "right".to_string(),
        };

        let result = process_string(input).unwrap();
        assert_eq!(result.processed, "hello world");
        assert_eq!(result.length_before, 15);
        assert_eq!(result.length_after, 11);
    }

    #[test]
    fn test_trim_char() {
        let input = StringTrimInput {
            text: "---hello---".to_string(),
            operation: "trim_char".to_string(),
            char_to_trim: Some("-".to_string()),
            pad_length: None,
            pad_char: " ".to_string(),
            pad_side: "right".to_string(),
        };

        let result = process_string(input).unwrap();
        assert_eq!(result.processed, "hello");
    }

    #[test]
    fn test_pad_right() {
        let input = StringTrimInput {
            text: "hello".to_string(),
            operation: "pad_right".to_string(),
            char_to_trim: None,
            pad_length: Some(10),
            pad_char: "-".to_string(),
            pad_side: "right".to_string(),
        };

        let result = process_string(input).unwrap();
        assert_eq!(result.processed, "hello-----");
        assert_eq!(result.length_after, 10);
    }

    #[test]
    fn test_pad_center() {
        let input = StringTrimInput {
            text: "hello".to_string(),
            operation: "pad_center".to_string(),
            char_to_trim: None,
            pad_length: Some(11),
            pad_char: "*".to_string(),
            pad_side: "right".to_string(),
        };

        let result = process_string(input).unwrap();
        assert_eq!(result.processed, "***hello***");
        assert_eq!(result.length_after, 11);
    }
}
