use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringSplitInput {
    pub text: String,
    
    #[serde(default = "default_delimiter")]
    pub delimiter: String,
    
    #[serde(default = "default_split_type")]
    pub split_type: String,
    
    #[serde(default)]
    pub limit: Option<usize>,
    
    #[serde(default)]
    pub trim_parts: bool,
    
    #[serde(default)]
    pub remove_empty: bool,
    
    #[serde(default)]
    pub case_sensitive: Option<bool>,
}

fn default_delimiter() -> String {
    " ".to_string()
}

fn default_split_type() -> String {
    "string".to_string()
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct StringSplitResult {
    pub parts: Vec<String>,
    pub count: usize,
    pub original: String,
    pub delimiter_used: String,
    pub split_type: String,
}

pub fn split_string(input: StringSplitInput) -> Result<StringSplitResult, String> {
    let original = input.text.clone();
    
    let mut parts: Vec<String> = match input.split_type.as_str() {
        "string" => {
            if input.delimiter.is_empty() {
                original.chars().map(|c| c.to_string()).collect()
            } else if let Some(limit) = input.limit {
                original.splitn(limit, &input.delimiter).map(|s| s.to_string()).collect()
            } else {
                original.split(&input.delimiter).map(|s| s.to_string()).collect()
            }
        },
        
        "regex" => {
            let regex = Regex::new(&input.delimiter)
                .map_err(|e| format!("Invalid regex pattern: {}", e))?;
            
            if let Some(limit) = input.limit {
                regex.splitn(&original, limit).map(|s| s.to_string()).collect()
            } else {
                regex.split(&original).map(|s| s.to_string()).collect()
            }
        },
        
        "whitespace" => {
            if let Some(limit) = input.limit {
                let mut result = Vec::new();
                let mut remaining = original.as_str();
                let mut count = 0;
                
                while count < limit - 1 && !remaining.is_empty() {
                    if let Some(pos) = remaining.find(char::is_whitespace) {
                        if pos > 0 {
                            result.push(remaining[..pos].to_string());
                            count += 1;
                        }
                        remaining = remaining[pos..].trim_start();
                    } else {
                        break;
                    }
                }
                
                if !remaining.is_empty() {
                    result.push(remaining.to_string());
                }
                
                result
            } else {
                original.split_whitespace().map(|s| s.to_string()).collect()
            }
        },
        
        "lines" => {
            if let Some(limit) = input.limit {
                let mut lines: Vec<String> = original.lines().map(|s| s.to_string()).collect();
                lines.truncate(limit);
                lines
            } else {
                original.lines().map(|s| s.to_string()).collect()
            }
        },
        
        "chars" => {
            let chars: Vec<String> = original.chars().map(|c| c.to_string()).collect();
            if let Some(limit) = input.limit {
                chars.into_iter().take(limit).collect()
            } else {
                chars
            }
        },
        
        "words" => {
            let word_regex = Regex::new(r"\b\w+\b").unwrap();
            let words: Vec<String> = word_regex
                .find_iter(&original)
                .map(|m| m.as_str().to_string())
                .collect();
            
            if let Some(limit) = input.limit {
                words.into_iter().take(limit).collect()
            } else {
                words
            }
        },
        
        _ => return Err(format!("Unknown split_type: {}. Valid types: string, regex, whitespace, lines, chars, words", input.split_type)),
    };
    
    if input.trim_parts {
        parts = parts.into_iter().map(|s| s.trim().to_string()).collect();
    }
    
    if input.remove_empty {
        parts = parts.into_iter().filter(|s| !s.is_empty()).collect();
    }
    
    let count = parts.len();
    
    Ok(StringSplitResult {
        parts,
        count,
        original,
        delimiter_used: match input.split_type.as_str() {
            "whitespace" => "<whitespace>".to_string(),
            "lines" => "<newline>".to_string(),
            "chars" => "<none>".to_string(),
            "words" => "<word-boundary>".to_string(),
            _ => input.delimiter,
        },
        split_type: input.split_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_split() {
        let input = StringSplitInput {
            text: "apple,banana,cherry".to_string(),
            delimiter: ",".to_string(),
            split_type: "string".to_string(),
            limit: None,
            trim_parts: false,
            remove_empty: false,
            case_sensitive: None,
        };
        
        let result = split_string(input).unwrap();
        assert_eq!(result.parts, vec!["apple", "banana", "cherry"]);
        assert_eq!(result.count, 3);
    }

    #[test]
    fn test_whitespace_split() {
        let input = StringSplitInput {
            text: "hello   world\t\nfrom rust".to_string(),
            delimiter: " ".to_string(),
            split_type: "whitespace".to_string(),
            limit: None,
            trim_parts: false,
            remove_empty: false,
            case_sensitive: None,
        };
        
        let result = split_string(input).unwrap();
        assert_eq!(result.parts, vec!["hello", "world", "from", "rust"]);
        assert_eq!(result.count, 4);
    }

    #[test]
    fn test_regex_split() {
        let input = StringSplitInput {
            text: "one1two2three3four".to_string(),
            delimiter: r"\d+".to_string(),
            split_type: "regex".to_string(),
            limit: None,
            trim_parts: false,
            remove_empty: false,
            case_sensitive: None,
        };
        
        let result = split_string(input).unwrap();
        assert_eq!(result.parts, vec!["one", "two", "three", "four"]);
    }

    #[test]
    fn test_split_with_limit() {
        let input = StringSplitInput {
            text: "a-b-c-d-e".to_string(),
            delimiter: "-".to_string(),
            split_type: "string".to_string(),
            limit: Some(3),
            trim_parts: false,
            remove_empty: false,
            case_sensitive: None,
        };
        
        let result = split_string(input).unwrap();
        assert_eq!(result.parts, vec!["a", "b", "c-d-e"]);
        assert_eq!(result.count, 3);
    }

    #[test]
    fn test_trim_and_remove_empty() {
        let input = StringSplitInput {
            text: "  a  ,  , b  ,  c  ".to_string(),
            delimiter: ",".to_string(),
            split_type: "string".to_string(),
            limit: None,
            trim_parts: true,
            remove_empty: true,
            case_sensitive: None,
        };
        
        let result = split_string(input).unwrap();
        assert_eq!(result.parts, vec!["a", "b", "c"]);
        assert_eq!(result.count, 3);
    }
}