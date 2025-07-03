use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize)]
pub struct CoordinateInput {
    pub lat1: f64,
    pub lon1: f64,
    pub lat2: f64,
    pub lon2: f64,
}

#[derive(Serialize)]
pub struct DistanceResult {
    pub distance_km: f64,
    pub distance_miles: f64,
    pub distance_nautical_miles: f64,
}

pub fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
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

pub fn calculate_distance(input: CoordinateInput) -> DistanceResult {
    let distance_km = haversine_distance(input.lat1, input.lon1, input.lat2, input.lon2);
    DistanceResult {
        distance_km,
        distance_miles: distance_km * 0.621371,
        distance_nautical_miles: distance_km * 0.539957,
    }
}