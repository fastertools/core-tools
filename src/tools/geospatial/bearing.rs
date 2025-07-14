use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct BearingInput {
    /// Latitude of the first point
    lat1: f64,
    /// Longitude of the first point
    lon1: f64,
    /// Latitude of the second point
    lat2: f64,
    /// Longitude of the second point
    lon2: f64,
}

/// Calculate bearing between two GPS coordinates
#[tool]
fn bearing(input: BearingInput) -> ToolResponse {
    use crate::geospatial::bearing::calculate_bearing;
    
    let bearing_degrees = calculate_bearing(input.lat1, input.lon1, input.lat2, input.lon2);
    
    // Create a response with bearing information
    let result = serde_json::json!({
        "bearing_degrees": bearing_degrees,
        "bearing_radians": bearing_degrees * std::f64::consts::PI / 180.0,
        "compass_direction": get_compass_direction(bearing_degrees)
    });
    
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}

fn get_compass_direction(bearing: f64) -> &'static str {
    match bearing {
        b if b < 11.25 || b >= 348.75 => "N",
        b if b < 33.75 => "NNE",
        b if b < 56.25 => "NE",
        b if b < 78.75 => "ENE",
        b if b < 101.25 => "E",
        b if b < 123.75 => "ESE",
        b if b < 146.25 => "SE",
        b if b < 168.75 => "SSE",
        b if b < 191.25 => "S",
        b if b < 213.75 => "SSW",
        b if b < 236.25 => "SW",
        b if b < 258.75 => "WSW",
        b if b < 281.25 => "W",
        b if b < 303.75 => "WNW",
        b if b < 326.25 => "NW",
        _ => "NNW",
    }
}