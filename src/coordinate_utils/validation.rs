use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CoordinateValidationInput {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Serialize)]
pub struct CoordinateInfo {
    pub latitude: f64,
    pub longitude: f64,
    pub hemisphere: String,
    pub quadrant: String,
    pub is_on_equator: bool,
    pub is_on_prime_meridian: bool,
}

pub fn validate_coordinates(lat: f64, lon: f64) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    
    // Check latitude bounds
    if lat < -90.0 || lat > 90.0 {
        errors.push(format!("Invalid latitude: {}. Must be between -90 and 90", lat));
    }
    
    // Check longitude bounds
    if lon < -180.0 || lon > 180.0 {
        errors.push(format!("Invalid longitude: {}. Must be between -180 and 180", lon));
    }
    
    // Check for extreme values
    if lat.abs() > 85.0 {
        warnings.push("Latitude is in polar regions where projections may be less accurate".to_string());
    }
    
    // Check for null island (0,0)
    if lat == 0.0 && lon == 0.0 {
        warnings.push("Coordinates are at Null Island (0,0) in the Gulf of Guinea".to_string());
    }
    
    // Check for NaN or infinite values
    if lat.is_nan() || lat.is_infinite() {
        errors.push("Latitude contains invalid numeric value (NaN or Infinity)".to_string());
    }
    
    if lon.is_nan() || lon.is_infinite() {
        errors.push("Longitude contains invalid numeric value (NaN or Infinity)".to_string());
    }
    
    ValidationResult {
        is_valid: errors.is_empty(),
        errors,
        warnings,
    }
}

pub fn get_coordinate_info(input: CoordinateValidationInput) -> Result<CoordinateInfo, String> {
    let validation = validate_coordinates(input.latitude, input.longitude);
    
    if !validation.is_valid {
        return Err(validation.errors.join(", "));
    }
    
    let hemisphere = match (input.latitude >= 0.0, input.longitude >= 0.0) {
        (true, true) => "Northern/Eastern",
        (true, false) => "Northern/Western",
        (false, true) => "Southern/Eastern",
        (false, false) => "Southern/Western",
    }.to_string();
    
    let quadrant = match (input.latitude >= 0.0, input.longitude >= 0.0) {
        (true, true) => "NE (Quadrant I)",
        (true, false) => "NW (Quadrant II)",
        (false, false) => "SW (Quadrant III)",
        (false, true) => "SE (Quadrant IV)",
    }.to_string();
    
    Ok(CoordinateInfo {
        latitude: input.latitude,
        longitude: input.longitude,
        hemisphere,
        quadrant,
        is_on_equator: input.latitude == 0.0,
        is_on_prime_meridian: input.longitude == 0.0,
    })
}

pub fn validate_coordinate_input(input: CoordinateValidationInput) -> ValidationResult {
    validate_coordinates(input.latitude, input.longitude)
}