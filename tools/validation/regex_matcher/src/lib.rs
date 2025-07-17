use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{RegexMatcherInput as LogicInput, RegexMatcherResult as LogicOutput, RegexFlags as LogicFlags, Match as LogicMatch, CaptureGroup as LogicGroup, PatternInfo as LogicInfo};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct RegexFlags {
    /// Case insensitive matching
    pub case_insensitive: Option<bool>,
    /// Multiline mode (^ and $ match line boundaries)
    pub multiline: Option<bool>,
    /// Dot matches newline
    pub dot_all: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
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

#[cfg_attr(not(test), tool)]
pub fn regex_matcher(input: RegexMatcherInput) -> Result<RegexMatcherResult, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        text: input.text,
        pattern: input.pattern,
        find_all: input.find_all,
        capture_groups: input.capture_groups,
        flags: input.flags.map(|f| LogicFlags {
            case_insensitive: f.case_insensitive,
            multiline: f.multiline,
            dot_all: f.dot_all,
        }),
    };
    
    // Call logic implementation
    let result = logic::match_regex(logic_input)?;
    
    // Convert back to wrapper types
    Ok(RegexMatcherResult {
        has_match: result.has_match,
        match_count: result.match_count,
        matches: result.matches.into_iter().map(|m| Match {
            text: m.text,
            start: m.start,
            end: m.end,
            groups: m.groups.map(|groups| {
                groups.into_iter().map(|g| CaptureGroup {
                    index: g.index,
                    name: g.name,
                    text: g.text,
                    start: g.start,
                    end: g.end,
                }).collect()
            }),
        }).collect(),
        pattern_info: PatternInfo {
            pattern: result.pattern_info.pattern,
            is_valid: result.pattern_info.is_valid,
            capture_group_count: result.pattern_info.capture_group_count,
            flags_applied: result.pattern_info.flags_applied,
        },
        error: result.error,
    })
}