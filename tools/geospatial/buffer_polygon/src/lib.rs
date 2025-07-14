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
}

#[derive(Deserialize, JsonSchema)]
struct CircularBufferInput {
    /// Center point for the buffer
    center: Point,
    /// Buffer radius in meters
    radius_meters: f64,
    /// Number of points to approximate circle (8-360, default 32)
    num_points: Option<usize>,
}

#[derive(Serialize)]
struct BufferResult {
    buffer_polygon: Vec<Point>,
    area_square_meters: f64,
    perimeter_meters: f64,
    algorithm_used: String,
}

const EARTH_RADIUS_M: f64 = 6378137.0; // WGS84 equatorial radius

fn create_circular_buffer(center: Point, radius_meters: f64, num_points: Option<usize>) -> Result<BufferResult, String> {
    if radius_meters <= 0.0 {
        return Err("Radius must be positive".to_string());
    }
    
    if center.lat < -90.0 || center.lat > 90.0 {
        return Err(format!("Invalid latitude: {}. Must be between -90 and 90", center.lat));
    }
    if center.lon < -180.0 || center.lon > 180.0 {
        return Err(format!("Invalid longitude: {}. Must be between -180 and 180", center.lon));
    }
    
    let num_points = num_points.unwrap_or(32).max(8).min(360);
    let mut buffer_points = Vec::new();
    
    let lat_rad = center.lat * PI / 180.0;
    let lon_rad = center.lon * PI / 180.0;
    
    // Angular distance
    let angular_distance = radius_meters / EARTH_RADIUS_M;
    
    for i in 0..num_points {
        let bearing = 2.0 * PI * i as f64 / num_points as f64;
        
        // Calculate destination point using spherical trigonometry
        let dest_lat_rad = (lat_rad.sin() * angular_distance.cos() + 
                           lat_rad.cos() * angular_distance.sin() * bearing.cos()).asin();
        
        let dest_lon_rad = lon_rad + (bearing.sin() * angular_distance.sin() * lat_rad.cos())
            .atan2(angular_distance.cos() - lat_rad.sin() * dest_lat_rad.sin());
        
        buffer_points.push(Point {
            lat: dest_lat_rad * 180.0 / PI,
            lon: dest_lon_rad * 180.0 / PI,
        });
    }
    
    // Calculate area (approximately πr²)
    let area = PI * radius_meters * radius_meters;
    
    // Calculate perimeter (2πr)
    let perimeter = 2.0 * PI * radius_meters;
    
    Ok(BufferResult {
        buffer_polygon: buffer_points,
        area_square_meters: area,
        perimeter_meters: perimeter,
        algorithm_used: "circular_geodesic".to_string(),
    })
}

/// Create circular buffer around a point using geodesic calculations
#[tool]
fn buffer_polygon(input: CircularBufferInput) -> ToolResponse {
    match create_circular_buffer(input.center, input.radius_meters, input.num_points) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}