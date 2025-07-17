use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{
    CurrentDatetimeInput as LogicInput, 
    CurrentDatetimeOutput as LogicOutput,
    DateTimeComponents as LogicComponents
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurrentDatetimeInput {
    /// Timezone (optional, default: UTC)
    /// Options: "UTC", "Local", or offset like "+05:30", "-08:00"
    pub timezone: Option<String>,
    /// Format for the output (optional)
    /// Options: "iso", "rfc2822", "rfc3339", "unix", "components"
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct CurrentDatetimeOutput {
    /// ISO 8601 formatted timestamp
    pub iso: String,
    /// Unix timestamp (seconds since epoch)
    pub unix_timestamp: i64,
    /// Unix timestamp in milliseconds
    pub unix_timestamp_ms: i64,
    /// RFC 2822 formatted timestamp
    pub rfc2822: String,
    /// RFC 3339 formatted timestamp
    pub rfc3339: String,
    /// Individual date/time components
    pub components: DateTimeComponents,
    /// Timezone used
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DateTimeComponents {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    pub hour: u32,
    pub minute: u32,
    pub second: u32,
    pub millisecond: u32,
    pub weekday: String,
    pub day_of_year: u32,
    pub week_of_year: u32,
}

#[cfg_attr(not(test), tool)]
pub fn current_datetime(input: CurrentDatetimeInput) -> Result<CurrentDatetimeOutput, String> {
    // Convert to logic types
    let logic_input = LogicInput {
        timezone: input.timezone,
        format: input.format,
    };
    
    // Call logic implementation
    let result = logic::get_current_datetime(logic_input)?;
    
    // Convert back to wrapper types
    Ok(CurrentDatetimeOutput {
        iso: result.iso,
        unix_timestamp: result.unix_timestamp,
        unix_timestamp_ms: result.unix_timestamp_ms,
        rfc2822: result.rfc2822,
        rfc3339: result.rfc3339,
        components: DateTimeComponents {
            year: result.components.year,
            month: result.components.month,
            day: result.components.day,
            hour: result.components.hour,
            minute: result.components.minute,
            second: result.components.second,
            millisecond: result.components.millisecond,
            weekday: result.components.weekday,
            day_of_year: result.components.day_of_year,
            week_of_year: result.components.week_of_year,
        },
        timezone: result.timezone,
    })
}