use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DecimalDegreesInput {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Deserialize)]
pub struct DMSInput {
    pub lat_degrees: i32,
    pub lat_minutes: i32,
    pub lat_seconds: f64,
    pub lat_direction: String, // N or S
    pub lon_degrees: i32,
    pub lon_minutes: i32,
    pub lon_seconds: f64,
    pub lon_direction: String, // E or W
}

#[derive(Serialize)]
pub struct DMSCoordinate {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
    pub direction: String,
}

#[derive(Serialize)]
pub struct DMSResult {
    pub latitude: DMSCoordinate,
    pub longitude: DMSCoordinate,
}

#[derive(Serialize)]
pub struct DecimalDegreesResult {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn decimal_to_dms(decimal: f64, is_latitude: bool) -> DMSCoordinate {
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

pub fn dms_to_decimal(degrees: i32, minutes: i32, seconds: f64, direction: &str) -> Result<f64, String> {
    if minutes < 0 || minutes >= 60 {
        return Err("Minutes must be between 0 and 59".to_string());
    }
    if seconds < 0.0 || seconds >= 60.0 {
        return Err("Seconds must be between 0 and 59.999".to_string());
    }
    
    let decimal = degrees.abs() as f64 + minutes as f64 / 60.0 + seconds / 3600.0;
    
    match direction.to_uppercase().as_str() {
        "N" | "E" => Ok(decimal),
        "S" | "W" => Ok(-decimal),
        _ => Err("Direction must be N, S, E, or W".to_string()),
    }
}

pub fn convert_to_dms(input: DecimalDegreesInput) -> Result<DMSResult, String> {
    if input.latitude < -90.0 || input.latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if input.longitude < -180.0 || input.longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }
    
    Ok(DMSResult {
        latitude: decimal_to_dms(input.latitude, true),
        longitude: decimal_to_dms(input.longitude, false),
    })
}

pub fn convert_to_decimal(input: DMSInput) -> Result<DecimalDegreesResult, String> {
    let lat = dms_to_decimal(input.lat_degrees, input.lat_minutes, input.lat_seconds, &input.lat_direction)?;
    let lon = dms_to_decimal(input.lon_degrees, input.lon_minutes, input.lon_seconds, &input.lon_direction)?;
    
    if lat < -90.0 || lat > 90.0 {
        return Err("Resulting latitude must be between -90 and 90".to_string());
    }
    if lon < -180.0 || lon > 180.0 {
        return Err("Resulting longitude must be between -180 and 180".to_string());
    }
    
    Ok(DecimalDegreesResult {
        latitude: lat,
        longitude: lon,
    })
}