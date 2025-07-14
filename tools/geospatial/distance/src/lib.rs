use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::f64::consts::PI;

#[derive(Deserialize, JsonSchema)]
struct DistanceInput {
    /// Latitude of the first point
    lat1: f64,
    /// Longitude of the first point
    lon1: f64,
    /// Latitude of the second point
    lat2: f64,
    /// Longitude of the second point
    lon2: f64,
}

#[derive(Serialize)]
struct DistanceResult {
    distance_km: f64,
    distance_miles: f64,
    distance_nautical_miles: f64,
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let delta_lat = (lat2 - lat1) * PI / 180.0;
    let delta_lon = (lon2 - lon1) * PI / 180.0;
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

fn calculate_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> DistanceResult {
    let distance_km = haversine_distance(lat1, lon1, lat2, lon2);
    DistanceResult {
        distance_km,
        distance_miles: distance_km * 0.621371,
        distance_nautical_miles: distance_km * 0.539957,
    }
}

/// Calculate distance between two GPS coordinates using Haversine formula
#[tool]
fn distance(input: DistanceInput) -> ToolResponse {
    let result = calculate_distance(input.lat1, input.lon1, input.lat2, input.lon2);
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}