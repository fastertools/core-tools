use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::f64::consts::PI;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
struct Point {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
    /// Optional identifier for the point
    id: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
struct ProximityZoneInput {
    /// Center of the proximity zone
    center: Point,
    /// Radius of the zone in meters
    radius_meters: f64,
    /// Points to analyze
    candidate_points: Vec<Point>,
}

#[derive(Serialize, Clone)]
struct NearestPointResult {
    point: Point,
    distance_meters: f64,
    bearing_degrees: f64,
}

#[derive(Serialize)]
struct ProximityZoneResult {
    center: Point,
    radius_meters: f64,
    points_in_zone: Vec<NearestPointResult>,
    points_outside_zone: Vec<NearestPointResult>,
    summary: ProximityZoneSummary,
}

#[derive(Serialize)]
struct ProximityZoneSummary {
    total_points: usize,
    points_inside: usize,
    points_outside: usize,
    average_distance_inside: f64,
    closest_point_distance: f64,
    farthest_point_distance: f64,
}

const EARTH_RADIUS_M: f64 = 6378137.0;

fn haversine_distance(point1: &Point, point2: &Point) -> f64 {
    let lat1_rad = point1.lat * PI / 180.0;
    let lat2_rad = point2.lat * PI / 180.0;
    let delta_lat = (point2.lat - point1.lat) * PI / 180.0;
    let delta_lon = (point2.lon - point1.lon) * PI / 180.0;
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_M * c
}

fn calculate_bearing(from: &Point, to: &Point) -> f64 {
    let lat1_rad = from.lat * PI / 180.0;
    let lat2_rad = to.lat * PI / 180.0;
    let delta_lon = (to.lon - from.lon) * PI / 180.0;
    
    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();
    
    let bearing_rad = y.atan2(x);
    (bearing_rad * 180.0 / PI + 360.0) % 360.0
}

fn proximity_zone_analysis(center: Point, radius_meters: f64, candidate_points: Vec<Point>) -> Result<ProximityZoneResult, String> {
    if radius_meters <= 0.0 {
        return Err("Radius must be positive".to_string());
    }
    
    if candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }
    
    // Validate center coordinates
    if center.lat < -90.0 || center.lat > 90.0 {
        return Err(format!("Invalid center latitude: {}. Must be between -90 and 90", center.lat));
    }
    if center.lon < -180.0 || center.lon > 180.0 {
        return Err(format!("Invalid center longitude: {}. Must be between -180 and 180", center.lon));
    }
    
    let mut points_inside = Vec::new();
    let mut points_outside = Vec::new();
    let mut distances_inside = Vec::new();
    let mut all_distances = Vec::new();
    
    for candidate in candidate_points {
        // Validate candidate coordinates
        if candidate.lat < -90.0 || candidate.lat > 90.0 {
            return Err(format!("Invalid candidate latitude: {}. Must be between -90 and 90", candidate.lat));
        }
        if candidate.lon < -180.0 || candidate.lon > 180.0 {
            return Err(format!("Invalid candidate longitude: {}. Must be between -180 and 180", candidate.lon));
        }
        
        let distance = haversine_distance(&center, &candidate);
        let bearing = calculate_bearing(&center, &candidate);
        all_distances.push(distance);
        
        let result = NearestPointResult {
            point: candidate,
            distance_meters: distance,
            bearing_degrees: bearing,
        };
        
        if distance <= radius_meters {
            distances_inside.push(distance);
            points_inside.push(result);
        } else {
            points_outside.push(result);
        }
    }
    
    // Calculate summary statistics
    let total_points = points_inside.len() + points_outside.len();
    let average_distance_inside = if distances_inside.is_empty() {
        0.0
    } else {
        distances_inside.iter().sum::<f64>() / distances_inside.len() as f64
    };
    
    let closest_point_distance = all_distances.iter().cloned().fold(f64::INFINITY, f64::min);
    let farthest_point_distance = all_distances.iter().cloned().fold(0.0, f64::max);
    
    Ok(ProximityZoneResult {
        center,
        radius_meters,
        points_in_zone: points_inside.clone(),
        points_outside_zone: points_outside,
        summary: ProximityZoneSummary {
            total_points,
            points_inside: points_inside.len(),
            points_outside: total_points - points_inside.len(),
            average_distance_inside,
            closest_point_distance,
            farthest_point_distance,
        },
    })
}

/// Analyze points within a proximity zone and provide detailed statistics
#[tool]
fn proximity_zone(input: ProximityZoneInput) -> ToolResponse {
    match proximity_zone_analysis(input.center, input.radius_meters, input.candidate_points) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}