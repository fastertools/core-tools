use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
struct Point {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

#[derive(Deserialize, JsonSchema)]
struct PointInPolygonInput {
    /// Point to test
    point: Point,
    /// Polygon vertices
    polygon: Vec<Point>,
}

#[derive(Serialize)]
struct PointInPolygonResult {
    is_inside: bool,
    algorithm_used: String,
    on_boundary: bool,
}

const EPSILON: f64 = 1e-10;

fn ray_casting_algorithm(point: &Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    
    let x = point.lon;
    let y = point.lat;
    let mut inside = false;
    let n = polygon.len();
    
    let mut j = n - 1;
    for i in 0..n {
        let xi = polygon[i].lon;
        let yi = polygon[i].lat;
        let xj = polygon[j].lon;
        let yj = polygon[j].lat;
        
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    
    inside
}

fn is_on_boundary(point: &Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    
    let n = polygon.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        if is_point_on_segment(point, &polygon[i], &polygon[j]) {
            return true;
        }
    }
    
    false
}

fn is_point_on_segment(point: &Point, seg_start: &Point, seg_end: &Point) -> bool {
    let cross_product = (point.lat - seg_start.lat) * (seg_end.lon - seg_start.lon) - 
                       (point.lon - seg_start.lon) * (seg_end.lat - seg_start.lat);
    
    if cross_product.abs() > EPSILON {
        return false;
    }
    
    let dot_product = (point.lon - seg_start.lon) * (seg_end.lon - seg_start.lon) + 
                     (point.lat - seg_start.lat) * (seg_end.lat - seg_start.lat);
    
    let squared_length = (seg_end.lon - seg_start.lon) * (seg_end.lon - seg_start.lon) + 
                        (seg_end.lat - seg_start.lat) * (seg_end.lat - seg_start.lat);
    
    dot_product >= 0.0 && dot_product <= squared_length
}

fn point_in_polygon_check(point: Point, polygon: Vec<Point>) -> Result<PointInPolygonResult, String> {
    if polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }
    
    // Validate coordinates
    for poly_point in &polygon {
        if poly_point.lat < -90.0 || poly_point.lat > 90.0 {
            return Err(format!("Invalid latitude: {}. Must be between -90 and 90", poly_point.lat));
        }
        if poly_point.lon < -180.0 || poly_point.lon > 180.0 {
            return Err(format!("Invalid longitude: {}. Must be between -180 and 180", poly_point.lon));
        }
    }
    
    if point.lat < -90.0 || point.lat > 90.0 {
        return Err(format!("Invalid point latitude: {}. Must be between -90 and 90", point.lat));
    }
    if point.lon < -180.0 || point.lon > 180.0 {
        return Err(format!("Invalid point longitude: {}. Must be between -180 and 180", point.lon));
    }
    
    let on_boundary = is_on_boundary(&point, &polygon);
    let is_inside = ray_casting_algorithm(&point, &polygon);
    
    Ok(PointInPolygonResult {
        is_inside,
        algorithm_used: "ray_casting".to_string(),
        on_boundary,
    })
}

/// Check if a point is inside a polygon using ray casting algorithm
#[tool]
fn point_in_polygon(input: PointInPolygonInput) -> ToolResponse {
    match point_in_polygon_check(input.point, input.polygon) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}