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
struct NearestPointsInput {
    /// Point to search from
    query_point: Point,
    /// Points to search among
    candidate_points: Vec<Point>,
    /// Maximum number of results to return
    max_results: Option<usize>,
    /// Only return points within this distance (meters)
    max_distance_meters: Option<f64>,
}

#[derive(Serialize)]
struct NearestPointResult {
    point: Point,
    distance_meters: f64,
    bearing_degrees: f64,
}

#[derive(Serialize)]
struct NearestPointsResult {
    query_point: Point,
    nearest_points: Vec<NearestPointResult>,
    total_candidates: usize,
    results_returned: usize,
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

fn find_nearest_points(query_point: Point, candidate_points: Vec<Point>, max_results: Option<usize>, max_distance_meters: Option<f64>) -> Result<NearestPointsResult, String> {
    if candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }
    
    // Validate query point
    if query_point.lat < -90.0 || query_point.lat > 90.0 {
        return Err(format!("Invalid query point latitude: {}. Must be between -90 and 90", query_point.lat));
    }
    if query_point.lon < -180.0 || query_point.lon > 180.0 {
        return Err(format!("Invalid query point longitude: {}. Must be between -180 and 180", query_point.lon));
    }
    
    let mut distances: Vec<(usize, f64)> = Vec::new();
    
    for (i, candidate) in candidate_points.iter().enumerate() {
        // Validate candidate coordinates
        if candidate.lat < -90.0 || candidate.lat > 90.0 {
            return Err(format!("Invalid candidate latitude: {}. Must be between -90 and 90", candidate.lat));
        }
        if candidate.lon < -180.0 || candidate.lon > 180.0 {
            return Err(format!("Invalid candidate longitude: {}. Must be between -180 and 180", candidate.lon));
        }
        
        let distance = haversine_distance(&query_point, candidate);
        
        // Apply distance filter if specified
        if let Some(max_dist) = max_distance_meters {
            if distance <= max_dist {
                distances.push((i, distance));
            }
        } else {
            distances.push((i, distance));
        }
    }
    
    // Sort by distance
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    
    // Apply result limit
    let max_results = max_results.unwrap_or(distances.len()).min(distances.len());
    
    let mut nearest_points = Vec::new();
    for i in 0..max_results {
        let (idx, distance) = distances[i];
        let candidate = &candidate_points[idx];
        let bearing = calculate_bearing(&query_point, candidate);
        
        nearest_points.push(NearestPointResult {
            point: candidate.clone(),
            distance_meters: distance,
            bearing_degrees: bearing,
        });
    }
    
    Ok(NearestPointsResult {
        query_point,
        nearest_points,
        total_candidates: candidate_points.len(),
        results_returned: max_results,
    })
}

/// Find nearest points to a query location with distance and bearing
#[tool]
fn proximity_search(input: NearestPointsInput) -> ToolResponse {
    match find_nearest_points(input.query_point, input.candidate_points, input.max_results, input.max_distance_meters) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}