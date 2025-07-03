use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize)]
pub struct BearingInput {
    pub lat1: f64,
    pub lon1: f64,
    pub lat2: f64,
    pub lon2: f64,
}

#[derive(Serialize)]
pub struct BearingResult {
    pub bearing_degrees: f64,
    pub bearing_radians: f64,
    pub compass_direction: String,
}

pub fn calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let delta_lon = (lon2 - lon1) * PI / 180.0;
    
    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();
    
    let bearing_rad = y.atan2(x);
    let bearing_deg = (bearing_rad * 180.0 / PI + 360.0) % 360.0;
    
    bearing_deg
}

pub fn degrees_to_compass(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE",
        "S", "SSW", "SW", "WSW", "W", "WNW", "NW", "NNW"
    ];
    
    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    directions[index].to_string()
}

pub fn get_bearing(input: BearingInput) -> BearingResult {
    let bearing_deg = calculate_bearing(input.lat1, input.lon1, input.lat2, input.lon2);
    let bearing_rad = bearing_deg * PI / 180.0;
    let compass = degrees_to_compass(bearing_deg);
    
    BearingResult {
        bearing_degrees: bearing_deg,
        bearing_radians: bearing_rad,
        compass_direction: compass,
    }
}