use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct PolygonSimplificationInput {
    pub polygon: Vec<Point>,
    pub tolerance_meters: f64,
    pub algorithm: Option<String>, // "douglas_peucker" or "visvalingam" (default: douglas_peucker)
}

#[derive(Serialize, JsonSchema)]
pub struct PolygonSimplificationResult {
    pub original_polygon: Vec<Point>,
    pub simplified_polygon: Vec<Point>,
    pub original_vertex_count: usize,
    pub simplified_vertex_count: usize,
    pub reduction_percentage: f64,
    pub algorithm_used: String,
    pub tolerance_used_meters: f64,
}

const EARTH_RADIUS_M: f64 = 6378137.0;

fn haversine_distance(point1: &Point, point2: &Point) -> f64 {
    let lat1_rad = point1.lat.to_radians();
    let lat2_rad = point2.lat.to_radians();
    let delta_lat = (point2.lat - point1.lat).to_radians();
    let delta_lon = (point2.lon - point1.lon).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_M * c
}

fn perpendicular_distance(point: &Point, line_start: &Point, line_end: &Point) -> f64 {
    // Calculate perpendicular distance from point to line segment using cross product
    let line_length = haversine_distance(line_start, line_end);
    
    if line_length == 0.0 {
        return haversine_distance(point, line_start);
    }
    
    // Convert to approximate Cartesian coordinates for calculation
    let x0 = point.lon;
    let y0 = point.lat;
    let x1 = line_start.lon;
    let y1 = line_start.lat;
    let x2 = line_end.lon;
    let y2 = line_end.lat;
    
    // Calculate perpendicular distance using cross product formula
    let numerator = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let denominator = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();
    
    if denominator == 0.0 {
        return haversine_distance(point, line_start);
    }
    
    // Convert back to meters (approximate)
    let distance_degrees = numerator / denominator;
    distance_degrees * 111320.0 // Approximate meters per degree at equator
}

fn douglas_peucker_simplify(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }
    
    let mut max_distance = 0.0;
    let mut max_index = 0;
    
    // Find the point with maximum distance from the line between first and last points
    for i in 1..points.len() - 1 {
        let distance = perpendicular_distance(&points[i], &points[0], &points[points.len() - 1]);
        if distance > max_distance {
            max_distance = distance;
            max_index = i;
        }
    }
    
    // If the maximum distance is greater than tolerance, recursively simplify
    if max_distance > tolerance {
        // Recursively simplify the two segments
        let left_segment = douglas_peucker_simplify(&points[0..=max_index], tolerance);
        let right_segment = douglas_peucker_simplify(&points[max_index..], tolerance);
        
        // Combine the results (avoiding duplicate middle point)
        let mut result = left_segment;
        result.extend(right_segment.into_iter().skip(1));
        result
    } else {
        // All points are within tolerance, return only endpoints
        vec![points[0].clone(), points[points.len() - 1].clone()]
    }
}

fn visvalingam_simplify(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() <= 3 {
        return points.to_vec();
    }
    
    let mut result = points.to_vec();
    let mut areas: Vec<f64> = Vec::new();
    
    // Calculate initial effective areas for all points
    for i in 1..result.len() - 1 {
        let area = triangle_area(&result[i - 1], &result[i], &result[i + 1]);
        areas.push(area);
    }
    
    // Convert tolerance to area threshold (approximate)
    let area_threshold = tolerance * tolerance;
    
    // Remove points with smallest effective areas iteratively
    while areas.len() > 1 {
        // Find minimum area
        let (min_index, &min_area) = areas.iter().enumerate().min_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
        
        if min_area > area_threshold {
            break;
        }
        
        // Remove the point with minimum area
        let point_index = min_index + 1; // Account for first point not having area
        result.remove(point_index);
        areas.remove(min_index);
        
        // Update areas for neighboring points
        if min_index > 0 && min_index < areas.len() {
            let new_area = triangle_area(&result[min_index - 1], &result[min_index], &result[min_index + 1]);
            areas[min_index - 1] = new_area;
        }
        if min_index < areas.len() {
            let new_area = triangle_area(&result[min_index], &result[min_index + 1], &result[min_index + 2]);
            areas[min_index] = new_area;
        }
    }
    
    result
}

fn triangle_area(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    // Calculate area of triangle using cross product (approximate for small areas)
    let x1 = p1.lon;
    let y1 = p1.lat;
    let x2 = p2.lon;
    let y2 = p2.lat;
    let x3 = p3.lon;
    let y3 = p3.lat;
    
    0.5 * ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs())
}

#[tool]
pub fn polygon_simplification(input: PolygonSimplificationInput) -> Result<PolygonSimplificationResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }
    
    if input.tolerance_meters <= 0.0 {
        return Err("Tolerance must be positive".to_string());
    }
    
    // Validate coordinates
    for point in &input.polygon {
        if point.lat < -90.0 || point.lat > 90.0 {
            return Err(format!("Invalid latitude: {}. Must be between -90 and 90", point.lat));
        }
        if point.lon < -180.0 || point.lon > 180.0 {
            return Err(format!("Invalid longitude: {}. Must be between -180 and 180", point.lon));
        }
    }
    
    let algorithm = input.algorithm.as_deref().unwrap_or("douglas_peucker");
    
    let simplified = match algorithm {
        "douglas_peucker" => douglas_peucker_simplify(&input.polygon, input.tolerance_meters),
        "visvalingam" => visvalingam_simplify(&input.polygon, input.tolerance_meters),
        _ => return Err("Algorithm must be 'douglas_peucker' or 'visvalingam'".to_string()),
    };
    
    let original_count = input.polygon.len();
    let simplified_count = simplified.len();
    let reduction_percentage = if original_count > 0 {
        ((original_count - simplified_count) as f64 / original_count as f64) * 100.0
    } else {
        0.0
    };
    
    Ok(PolygonSimplificationResult {
        original_polygon: input.polygon,
        simplified_polygon: simplified,
        original_vertex_count: original_count,
        simplified_vertex_count: simplified_count,
        reduction_percentage,
        algorithm_used: algorithm.to_string(),
        tolerance_used_meters: input.tolerance_meters,
    })
}