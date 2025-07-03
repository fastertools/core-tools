use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize, Serialize, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct CircularBufferInput {
    pub center: Point,
    pub radius_meters: f64,
    pub num_points: Option<usize>, // Number of points to approximate circle (default 32)
}

#[derive(Deserialize)]
pub struct PolygonBufferInput {
    pub polygon: Vec<Point>,
    pub buffer_distance_meters: f64,
    pub num_points_per_vertex: Option<usize>, // Points for rounded corners (default 8)
}

#[derive(Serialize)]
pub struct BufferResult {
    pub buffer_polygon: Vec<Point>,
    pub area_square_meters: f64,
    pub perimeter_meters: f64,
    pub algorithm_used: String,
}

#[derive(Serialize)]
pub struct MultiBufferResult {
    pub buffers: Vec<BufferDistance>,
}

#[derive(Serialize)]
pub struct BufferDistance {
    pub distance_meters: f64,
    pub buffer_polygon: Vec<Point>,
    pub area_square_meters: f64,
}

const EARTH_RADIUS_M: f64 = 6378137.0; // WGS84 equatorial radius

pub fn create_circular_buffer(input: CircularBufferInput) -> Result<BufferResult, String> {
    if input.radius_meters <= 0.0 {
        return Err("Radius must be positive".to_string());
    }
    
    if input.center.lat < -90.0 || input.center.lat > 90.0 {
        return Err(format!("Invalid latitude: {}. Must be between -90 and 90", input.center.lat));
    }
    if input.center.lon < -180.0 || input.center.lon > 180.0 {
        return Err(format!("Invalid longitude: {}. Must be between -180 and 180", input.center.lon));
    }
    
    let num_points = input.num_points.unwrap_or(32).max(8).min(360);
    let mut buffer_points = Vec::new();
    
    let lat_rad = input.center.lat * PI / 180.0;
    let lon_rad = input.center.lon * PI / 180.0;
    
    // Angular distance
    let angular_distance = input.radius_meters / EARTH_RADIUS_M;
    
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
    let area = PI * input.radius_meters * input.radius_meters;
    
    // Calculate perimeter (2πr)
    let perimeter = 2.0 * PI * input.radius_meters;
    
    Ok(BufferResult {
        buffer_polygon: buffer_points,
        area_square_meters: area,
        perimeter_meters: perimeter,
        algorithm_used: "circular_geodesic".to_string(),
    })
}

pub fn create_polygon_buffer(input: PolygonBufferInput) -> Result<BufferResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }
    
    if input.buffer_distance_meters <= 0.0 {
        return Err("Buffer distance must be positive".to_string());
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
    
    // Simplified buffer algorithm - create circular buffers around each vertex and connect
    let num_points_per_vertex = input.num_points_per_vertex.unwrap_or(8).max(4).min(32);
    let mut buffer_points = Vec::new();
    
    let angular_distance = input.buffer_distance_meters / EARTH_RADIUS_M;
    
    for vertex in &input.polygon {
        let lat_rad = vertex.lat * PI / 180.0;
        let lon_rad = vertex.lon * PI / 180.0;
        
        // Create arc around vertex
        for i in 0..num_points_per_vertex {
            let bearing = 2.0 * PI * i as f64 / num_points_per_vertex as f64;
            
            let dest_lat_rad = (lat_rad.sin() * angular_distance.cos() + 
                               lat_rad.cos() * angular_distance.sin() * bearing.cos()).asin();
            
            let dest_lon_rad = lon_rad + (bearing.sin() * angular_distance.sin() * lat_rad.cos())
                .atan2(angular_distance.cos() - lat_rad.sin() * dest_lat_rad.sin());
            
            buffer_points.push(Point {
                lat: dest_lat_rad * 180.0 / PI,
                lon: dest_lon_rad * 180.0 / PI,
            });
        }
    }
    
    // Estimate area and perimeter (simplified)
    let original_area = estimate_polygon_area(&input.polygon);
    let original_perimeter = estimate_polygon_perimeter(&input.polygon);
    
    // Buffered area ≈ original + perimeter × buffer + π × buffer²
    let buffer_area = original_area + 
                     original_perimeter * input.buffer_distance_meters + 
                     PI * input.buffer_distance_meters * input.buffer_distance_meters;
    
    let buffer_perimeter = original_perimeter + 2.0 * PI * input.buffer_distance_meters;
    
    Ok(BufferResult {
        buffer_polygon: buffer_points,
        area_square_meters: buffer_area,
        perimeter_meters: buffer_perimeter,
        algorithm_used: "simplified_vertex_buffer".to_string(),
    })
}

fn estimate_polygon_area(coordinates: &[Point]) -> f64 {
    if coordinates.len() < 3 {
        return 0.0;
    }
    
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
    
    area.abs() * EARTH_RADIUS_M * EARTH_RADIUS_M / 2.0
}

fn estimate_polygon_perimeter(coordinates: &[Point]) -> f64 {
    if coordinates.len() < 2 {
        return 0.0;
    }
    
    let mut perimeter = 0.0;
    let n = coordinates.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        perimeter += haversine_distance(&coordinates[i], &coordinates[j]);
    }
    
    perimeter
}

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

pub fn create_multi_distance_buffers(center: Point, distances: Vec<f64>) -> Result<MultiBufferResult, String> {
    if distances.is_empty() {
        return Err("At least one distance must be provided".to_string());
    }
    
    let mut buffers = Vec::new();
    
    for distance in distances {
        if distance <= 0.0 {
            return Err("All distances must be positive".to_string());
        }
        
        let buffer_input = CircularBufferInput {
            center: center.clone(),
            radius_meters: distance,
            num_points: Some(32),
        };
        
        let buffer_result = create_circular_buffer(buffer_input)?;
        
        buffers.push(BufferDistance {
            distance_meters: distance,
            buffer_polygon: buffer_result.buffer_polygon,
            area_square_meters: buffer_result.area_square_meters,
        });
    }
    
    Ok(MultiBufferResult { buffers })
}