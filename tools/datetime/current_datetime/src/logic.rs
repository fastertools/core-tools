use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Local, FixedOffset, Datelike, Timelike};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentDatetimeInput {
    /// Timezone (optional, default: UTC)
    /// Options: "UTC", "Local", or offset like "+05:30", "-08:00"
    pub timezone: Option<String>,
    /// Format for the output (optional)
    /// Options: "iso", "rfc2822", "rfc3339", "unix", "components"
    pub format: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub fn get_current_datetime(input: CurrentDatetimeInput) -> Result<CurrentDatetimeOutput, String> {
    let timezone = input.timezone.unwrap_or_else(|| "UTC".to_string());
    
    // Get current time in UTC first
    let utc_now = Utc::now();
    
    // Convert to requested timezone
    let (datetime_str, tz_name): (String, String) = match timezone.as_str() {
        "UTC" => {
            let dt = utc_now;
            (dt.to_rfc3339(), "UTC".to_string())
        },
        "Local" => {
            let dt = Local::now();
            (dt.to_rfc3339(), "Local".to_string())
        },
        tz if tz.starts_with('+') || tz.starts_with('-') => {
            // Parse offset like "+05:30" or "-08:00"
            let offset = parse_timezone_offset(&timezone)?;
            let dt = utc_now.with_timezone(&offset);
            (dt.to_rfc3339(), timezone.clone())
        },
        _ => {
            return Err(format!(
                "Invalid timezone '{}'. Use 'UTC', 'Local', or offset like '+05:30', '-08:00'",
                timezone
            ));
        }
    };
    
    // Parse the datetime string to get a proper DateTime object
    let datetime = DateTime::parse_from_rfc3339(&datetime_str)
        .map_err(|e| format!("Failed to parse datetime: {}", e))?;
    
    // Calculate components
    let components = DateTimeComponents {
        year: datetime.year(),
        month: datetime.month(),
        day: datetime.day(),
        hour: datetime.hour(),
        minute: datetime.minute(),
        second: datetime.second(),
        millisecond: datetime.timestamp_subsec_millis(),
        weekday: datetime.format("%A").to_string(),
        day_of_year: datetime.ordinal(),
        week_of_year: datetime.iso_week().week(),
    };
    
    Ok(CurrentDatetimeOutput {
        iso: datetime.to_rfc3339(),
        unix_timestamp: datetime.timestamp(),
        unix_timestamp_ms: datetime.timestamp_millis(),
        rfc2822: datetime.to_rfc2822(),
        rfc3339: datetime.to_rfc3339(),
        components,
        timezone: tz_name,
    })
}

fn parse_timezone_offset(offset_str: &str) -> Result<FixedOffset, String> {
    // Remove the sign for parsing
    let sign = if offset_str.starts_with('-') { -1 } else { 1 };
    let offset_str = offset_str.trim_start_matches(&['+', '-'][..]);
    
    // Split hours and minutes
    let parts: Vec<&str> = offset_str.split(':').collect();
    if parts.len() != 2 {
        return Err("Timezone offset must be in format '+HH:MM' or '-HH:MM'".to_string());
    }
    
    let hours: i32 = parts[0].parse()
        .map_err(|_| "Invalid hours in timezone offset".to_string())?;
    let minutes: i32 = parts[1].parse()
        .map_err(|_| "Invalid minutes in timezone offset".to_string())?;
    
    if hours < 0 || hours > 14 {
        return Err("Timezone offset hours must be between 0 and 14".to_string());
    }
    if minutes < 0 || minutes > 59 {
        return Err("Timezone offset minutes must be between 0 and 59".to_string());
    }
    
    let total_seconds = sign * (hours * 3600 + minutes * 60);
    
    FixedOffset::east_opt(total_seconds)
        .ok_or_else(|| "Invalid timezone offset".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_utc_default() {
        let input = CurrentDatetimeInput {
            timezone: None,
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        assert_eq!(result.timezone, "UTC");
        
        // Verify timestamps are reasonable (within last minute)
        let now = Utc::now().timestamp();
        assert!(result.unix_timestamp >= now - 60);
        assert!(result.unix_timestamp <= now + 1);
        
        // Verify components make sense
        assert!(result.components.year >= 2024);
        assert!(result.components.month >= 1 && result.components.month <= 12);
        assert!(result.components.day >= 1 && result.components.day <= 31);
        assert!(result.components.hour <= 23);
        assert!(result.components.minute <= 59);
        assert!(result.components.second <= 59);
    }
    
    #[test]
    fn test_local_timezone() {
        let input = CurrentDatetimeInput {
            timezone: Some("Local".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        assert_eq!(result.timezone, "Local");
    }
    
    #[test]
    fn test_positive_offset() {
        let input = CurrentDatetimeInput {
            timezone: Some("+05:30".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        assert_eq!(result.timezone, "+05:30");
        
        // Verify the time is offset correctly
        assert!(result.iso.contains("+05:30"));
    }
    
    #[test]
    fn test_negative_offset() {
        let input = CurrentDatetimeInput {
            timezone: Some("-08:00".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        assert_eq!(result.timezone, "-08:00");
        
        // Verify the time is offset correctly
        assert!(result.iso.contains("-08:00"));
    }
    
    #[test]
    fn test_all_output_formats() {
        let input = CurrentDatetimeInput {
            timezone: Some("UTC".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        
        // Check all formats are present and valid
        assert!(!result.iso.is_empty());
        assert!(!result.rfc2822.is_empty());
        assert!(!result.rfc3339.is_empty());
        assert!(result.unix_timestamp > 0);
        assert!(result.unix_timestamp_ms > 0);
        
        // Verify millisecond timestamp is 1000x the second timestamp
        assert_eq!(result.unix_timestamp_ms / 1000, result.unix_timestamp);
    }
    
    #[test]
    fn test_weekday_names() {
        let input = CurrentDatetimeInput {
            timezone: Some("UTC".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input).unwrap();
        
        let valid_weekdays = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
        assert!(valid_weekdays.contains(&result.components.weekday.as_str()));
    }
    
    #[test]
    fn test_invalid_timezone() {
        let input = CurrentDatetimeInput {
            timezone: Some("Invalid/Timezone".to_string()),
            format: None,
        };
        
        let result = get_current_datetime(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid timezone"));
    }
    
    #[test]
    fn test_invalid_offset_format() {
        let input = CurrentDatetimeInput {
            timezone: Some("+5:30".to_string()), // Missing leading zero
            format: None,
        };
        
        let result = get_current_datetime(input);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_extreme_offset() {
        let input = CurrentDatetimeInput {
            timezone: Some("+14:00".to_string()), // Max valid offset
            format: None,
        };
        
        let result = get_current_datetime(input);
        assert!(result.is_ok());
        
        let input = CurrentDatetimeInput {
            timezone: Some("-12:00".to_string()), // Valid negative offset
            format: None,
        };
        
        let result = get_current_datetime(input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_parse_timezone_offset() {
        assert!(parse_timezone_offset("+05:30").is_ok());
        assert!(parse_timezone_offset("-08:00").is_ok());
        assert!(parse_timezone_offset("+00:00").is_ok());
        assert!(parse_timezone_offset("-12:00").is_ok());
        
        assert!(parse_timezone_offset("05:30").is_err()); // Missing sign
        assert!(parse_timezone_offset("+5:30").is_err()); // Missing leading zero
        assert!(parse_timezone_offset("+15:00").is_err()); // Too large
        assert!(parse_timezone_offset("+05:60").is_err()); // Invalid minutes
    }
}