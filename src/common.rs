use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

pub fn validate_coordinates(lat: f64, lon: f64) -> Result<(), String> {
    if lat < -90.0 || lat > 90.0 {
        return Err(format!("Invalid latitude: {}. Must be between -90 and 90", lat));
    }
    if lon < -180.0 || lon > 180.0 {
        return Err(format!("Invalid longitude: {}. Must be between -180 and 180", lon));
    }
    Ok(())
}