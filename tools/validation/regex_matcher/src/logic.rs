use serde::{Deserialize, Serialize};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatcherInput {
    /// Text to match against
    pub text: String,
    /// Regular expression pattern
    pub pattern: String,
    /// Whether to find all matches (vs just first)
    pub find_all: Option<bool>,
    /// Whether to capture groups
    pub capture_groups: Option<bool>,
    /// Regex flags (case_insensitive, multiline, dot_all)
    pub flags: Option<RegexFlags>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexFlags {
    /// Case insensitive matching
    pub case_insensitive: Option<bool>,
    /// Multiline mode (^ and $ match line boundaries)
    pub multiline: Option<bool>,
    /// Dot matches newline
    pub dot_all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegexMatcherResult {
    /// Whether any match was found
    pub has_match: bool,
    /// Number of matches found
    pub match_count: usize,
    /// All matches found
    pub matches: Vec<Match>,
    /// Pattern validation info
    pub pattern_info: PatternInfo,
    /// Error if pattern is invalid
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Match {
    /// The matched text
    pub text: String,
    /// Start position in the input
    pub start: usize,
    /// End position in the input
    pub end: usize,
    /// Captured groups (if any)
    pub groups: Option<Vec<CaptureGroup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureGroup {
    /// Group index (0 is full match)
    pub index: usize,
    /// Group name (if named)
    pub name: Option<String>,
    /// Captured text
    pub text: Option<String>,
    /// Start position
    pub start: Option<usize>,
    /// End position
    pub end: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternInfo {
    /// The pattern used
    pub pattern: String,
    /// Whether pattern is valid
    pub is_valid: bool,
    /// Number of capture groups
    pub capture_group_count: usize,
    /// Flags applied
    pub flags_applied: String,
}

pub fn match_regex(input: RegexMatcherInput) -> Result<RegexMatcherResult, String> {
    // Build regex with flags
    let mut pattern_with_flags = String::new();
    let mut flags_applied = Vec::new();
    
    if let Some(flags) = &input.flags {
        if flags.case_insensitive.unwrap_or(false) {
            pattern_with_flags.push_str("(?i)");
            flags_applied.push("case_insensitive");
        }
        if flags.multiline.unwrap_or(false) {
            pattern_with_flags.push_str("(?m)");
            flags_applied.push("multiline");
        }
        if flags.dot_all.unwrap_or(false) {
            pattern_with_flags.push_str("(?s)");
            flags_applied.push("dot_all");
        }
    }
    
    pattern_with_flags.push_str(&input.pattern);
    
    // Compile regex
    let regex = match Regex::new(&pattern_with_flags) {
        Ok(re) => re,
        Err(e) => {
            return Ok(RegexMatcherResult {
                has_match: false,
                match_count: 0,
                matches: vec![],
                pattern_info: PatternInfo {
                    pattern: input.pattern.clone(),
                    is_valid: false,
                    capture_group_count: 0,
                    flags_applied: flags_applied.join(", "),
                },
                error: Some(format!("Invalid regex pattern: {}", e)),
            });
        }
    };
    
    let capture_groups = input.capture_groups.unwrap_or(false);
    let find_all = input.find_all.unwrap_or(true);
    
    // Find matches
    let mut matches = Vec::new();
    
    if find_all {
        for cap in regex.captures_iter(&input.text) {
            let full_match = cap.get(0).unwrap();
            let mut match_item = Match {
                text: full_match.as_str().to_string(),
                start: full_match.start(),
                end: full_match.end(),
                groups: None,
            };
            
            if capture_groups {
                let mut groups = Vec::new();
                
                // Include all groups (including full match at index 0)
                for i in 0..cap.len() {
                    if let Some(group) = cap.get(i) {
                        groups.push(CaptureGroup {
                            index: i,
                            name: None, // Named groups would need regex crate feature
                            text: Some(group.as_str().to_string()),
                            start: Some(group.start()),
                            end: Some(group.end()),
                        });
                    } else {
                        groups.push(CaptureGroup {
                            index: i,
                            name: None,
                            text: None,
                            start: None,
                            end: None,
                        });
                    }
                }
                
                match_item.groups = Some(groups);
            }
            
            matches.push(match_item);
        }
    } else {
        // Find only first match
        if let Some(cap) = regex.captures(&input.text) {
            let full_match = cap.get(0).unwrap();
            let mut match_item = Match {
                text: full_match.as_str().to_string(),
                start: full_match.start(),
                end: full_match.end(),
                groups: None,
            };
            
            if capture_groups {
                let mut groups = Vec::new();
                
                for i in 0..cap.len() {
                    if let Some(group) = cap.get(i) {
                        groups.push(CaptureGroup {
                            index: i,
                            name: None,
                            text: Some(group.as_str().to_string()),
                            start: Some(group.start()),
                            end: Some(group.end()),
                        });
                    } else {
                        groups.push(CaptureGroup {
                            index: i,
                            name: None,
                            text: None,
                            start: None,
                            end: None,
                        });
                    }
                }
                
                match_item.groups = Some(groups);
            }
            
            matches.push(match_item);
        }
    }
    
    let has_match = !matches.is_empty();
    let match_count = matches.len();
    
    Ok(RegexMatcherResult {
        has_match,
        match_count,
        matches,
        pattern_info: PatternInfo {
            pattern: input.pattern.clone(),
            is_valid: true,
            capture_group_count: regex.captures_len() - 1, // Exclude full match
            flags_applied: if flags_applied.is_empty() { 
                "none".to_string() 
            } else { 
                flags_applied.join(", ") 
            },
        },
        error: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_match() {
        let input = RegexMatcherInput {
            text: "The quick brown fox".to_string(),
            pattern: "quick".to_string(),
            find_all: None,
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.match_count, 1);
        assert_eq!(result.matches[0].text, "quick");
        assert_eq!(result.matches[0].start, 4);
        assert_eq!(result.matches[0].end, 9);
    }

    #[test]
    fn test_multiple_matches() {
        let input = RegexMatcherInput {
            text: "cat bat rat".to_string(),
            pattern: r"\b\w{3}\b".to_string(),
            find_all: Some(true),
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.match_count, 3);
        assert_eq!(result.matches[0].text, "cat");
        assert_eq!(result.matches[1].text, "bat");
        assert_eq!(result.matches[2].text, "rat");
    }

    #[test]
    fn test_no_match() {
        let input = RegexMatcherInput {
            text: "hello world".to_string(),
            pattern: "xyz".to_string(),
            find_all: None,
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(!result.has_match);
        assert_eq!(result.match_count, 0);
    }

    #[test]
    fn test_case_insensitive() {
        let input = RegexMatcherInput {
            text: "Hello WORLD".to_string(),
            pattern: "hello".to_string(),
            find_all: None,
            capture_groups: None,
            flags: Some(RegexFlags {
                case_insensitive: Some(true),
                multiline: None,
                dot_all: None,
            }),
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.matches[0].text, "Hello");
    }

    #[test]
    fn test_capture_groups() {
        let input = RegexMatcherInput {
            text: "email: user@example.com".to_string(),
            pattern: r"(\w+)@(\w+\.\w+)".to_string(),
            find_all: None,
            capture_groups: Some(true),
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        
        let groups = result.matches[0].groups.as_ref().unwrap();
        assert_eq!(groups.len(), 3); // Full match + 2 groups
        assert_eq!(groups[0].text.as_ref().unwrap(), "user@example.com");
        assert_eq!(groups[1].text.as_ref().unwrap(), "user");
        assert_eq!(groups[2].text.as_ref().unwrap(), "example.com");
    }

    #[test]
    fn test_multiline_flag() {
        let input = RegexMatcherInput {
            text: "line1\nline2\nline3".to_string(),
            pattern: "^line2$".to_string(),
            find_all: None,
            capture_groups: None,
            flags: Some(RegexFlags {
                case_insensitive: None,
                multiline: Some(true),
                dot_all: None,
            }),
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.matches[0].text, "line2");
    }

    #[test]
    fn test_dot_all_flag() {
        let input = RegexMatcherInput {
            text: "line1\nline2".to_string(),
            pattern: "line1.line2".to_string(),
            find_all: None,
            capture_groups: None,
            flags: Some(RegexFlags {
                case_insensitive: None,
                multiline: None,
                dot_all: Some(true),
            }),
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
    }

    #[test]
    fn test_invalid_pattern() {
        let input = RegexMatcherInput {
            text: "test".to_string(),
            pattern: "[".to_string(), // Invalid regex - unclosed bracket
            find_all: None,
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(!result.has_match);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("Invalid regex pattern"));
        assert!(!result.pattern_info.is_valid);
    }

    #[test]
    fn test_first_match_only() {
        let input = RegexMatcherInput {
            text: "abc abc abc".to_string(),
            pattern: "abc".to_string(),
            find_all: Some(false),
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.match_count, 1);
        assert_eq!(result.matches[0].start, 0);
    }

    #[test]
    fn test_digit_pattern() {
        let input = RegexMatcherInput {
            text: "The price is $42.99".to_string(),
            pattern: r"\d+\.?\d*".to_string(),
            find_all: Some(true),
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.match_count, 1);
        assert_eq!(result.matches[0].text, "42.99");
    }

    #[test]
    fn test_email_pattern() {
        let input = RegexMatcherInput {
            text: "Contact us at support@example.com or sales@example.org".to_string(),
            pattern: r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b".to_string(),
            find_all: Some(true),
            capture_groups: None,
            flags: None,
        };
        let result = match_regex(input).unwrap();
        assert!(result.has_match);
        assert_eq!(result.match_count, 2);
        assert_eq!(result.matches[0].text, "support@example.com");
        assert_eq!(result.matches[1].text, "sales@example.org");
    }

    #[test]
    fn test_pattern_info() {
        let input = RegexMatcherInput {
            text: "test".to_string(),
            pattern: r"(te)(st)".to_string(),
            find_all: None,
            capture_groups: None,
            flags: Some(RegexFlags {
                case_insensitive: Some(true),
                multiline: Some(true),
                dot_all: None,
            }),
        };
        let result = match_regex(input).unwrap();
        assert!(result.pattern_info.is_valid);
        assert_eq!(result.pattern_info.capture_group_count, 2);
        assert!(result.pattern_info.flags_applied.contains("case_insensitive"));
        assert!(result.pattern_info.flags_applied.contains("multiline"));
    }
}