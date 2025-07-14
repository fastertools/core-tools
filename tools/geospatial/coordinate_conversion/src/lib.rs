use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct DecimalDegreesInput {
    /// Latitude in decimal degrees
    latitude: f64,
    /// Longitude in decimal degrees
    longitude: f64,
}

#[derive(Serialize)]
struct DMSCoordinate {
    degrees: i32,
    minutes: i32,
    seconds: f64,
    direction: String,
}

#[derive(Serialize)]
struct DMSResult {
    latitude: DMSCoordinate,
    longitude: DMSCoordinate,
}

fn decimal_to_dms(decimal: f64, is_latitude: bool) -> DMSCoordinate {
    let abs_decimal = decimal.abs();
    let degrees = abs_decimal.floor() as i32;
    let minutes_float = (abs_decimal - degrees as f64) * 60.0;
    let minutes = minutes_float.floor() as i32;
    let seconds = (minutes_float - minutes as f64) * 60.0;
    
    let direction = if is_latitude {
        if decimal >= 0.0 { "N".to_string() } else { "S".to_string() }
    } else {
        if decimal >= 0.0 { "E".to_string() } else { "W".to_string() }
    };
    
    DMSCoordinate {
        degrees,
        minutes,
        seconds,
        direction,
    }
}

fn convert_to_dms(latitude: f64, longitude: f64) -> Result<DMSResult, String> {
    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }
    
    Ok(DMSResult {
        latitude: decimal_to_dms(latitude, true),
        longitude: decimal_to_dms(longitude, false),
    })
}

/// Convert decimal degrees to degrees, minutes, seconds (DMS) format
#[tool]
fn coordinate_conversion(input: DecimalDegreesInput) -> ToolResponse {
    match convert_to_dms(input.latitude, input.longitude) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}