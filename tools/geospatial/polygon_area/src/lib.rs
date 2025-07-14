use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::f64::consts::PI;

#[derive(Deserialize, JsonSchema, Clone)]
struct Coordinate {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

#[derive(Deserialize, JsonSchema)]
struct PolygonInput {
    /// Array of coordinates defining the polygon
    coordinates: Vec<Coordinate>,
}

#[derive(Serialize)]
struct PolygonAreaResult {
    area_square_meters: f64,
    area_square_kilometers: f64,
    area_square_miles: f64,
    area_hectares: f64,
    area_acres: f64,
}

fn calculate_polygon_area(coordinates: &[Coordinate]) -> Result<f64, String> {
    if coordinates.len() < 3 {
        return Err("Polygon must have at least 3 coordinates".to_string());
    }
    
    const EARTH_RADIUS_M: f64 = 6378137.0; // WGS84 equatorial radius in meters
    
    let mut area = 0.0;
    let n = coordinates.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let lat1 = coordinates[i].lat * PI / 180.0;
        let lat2 = coordinates[j].lat * PI / 180.0;
        let lon1 = coordinates[i].lon * PI / 180.0;
        let lon2 = coordinates[j].lon * PI / 180.0;
        
        area += (lon2 - lon1) * (2.0 + lat1.sin() + lat2.sin());
    }
    
    area = area.abs() * EARTH_RADIUS_M * EARTH_RADIUS_M / 2.0;
    
    Ok(area)
}

fn get_polygon_area(coordinates: Vec<Coordinate>) -> Result<PolygonAreaResult, String> {
    // Validate coordinates
    for coord in &coordinates {
        if coord.lat < -90.0 || coord.lat > 90.0 {
            return Err(format!("Invalid latitude: {}. Must be between -90 and 90", coord.lat));
        }
        if coord.lon < -180.0 || coord.lon > 180.0 {
            return Err(format!("Invalid longitude: {}. Must be between -180 and 180", coord.lon));
        }
    }
    
    let area_m2 = calculate_polygon_area(&coordinates)?;
    
    Ok(PolygonAreaResult {
        area_square_meters: area_m2,
        area_square_kilometers: area_m2 / 1_000_000.0,
        area_square_miles: area_m2 / 2_589_988.11,
        area_hectares: area_m2 / 10_000.0,
        area_acres: area_m2 / 4_046.86,
    })
}

/// Calculate area of a GPS polygon
#[tool]
fn polygon_area(input: PolygonInput) -> ToolResponse {
    match get_polygon_area(input.coordinates) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}