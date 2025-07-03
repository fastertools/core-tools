use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize, Serialize, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
    pub id: Option<String>,
}

#[derive(Deserialize)]
pub struct NearestPointsInput {
    pub query_point: Point,
    pub candidate_points: Vec<Point>,
    pub max_results: Option<usize>,
    pub max_distance_meters: Option<f64>,
}

#[derive(Deserialize)]
pub struct DistanceToPolygonInput {
    pub point: Point,
    pub polygon: Vec<Point>,
}

#[derive(Deserialize)]
pub struct ProximityZoneInput {
    pub center: Point,
    pub radius_meters: f64,
    pub candidate_points: Vec<Point>,
}

#[derive(Serialize)]
pub struct NearestPointResult {
    pub point: Point,
    pub distance_meters: f64,
    pub bearing_degrees: f64,
}

#[derive(Serialize)]
pub struct NearestPointsResult {
    pub query_point: Point,
    pub nearest_points: Vec<NearestPointResult>,
    pub total_candidates: usize,
    pub results_returned: usize,
}

#[derive(Serialize)]
pub struct DistanceToPolygonResult {
    pub distance_meters: f64,
    pub closest_point_on_polygon: Point,
    pub is_inside_polygon: bool,
    pub closest_segment_index: usize,
}

#[derive(Serialize)]
pub struct ProximityZoneResult {
    pub center: Point,
    pub radius_meters: f64,
    pub points_in_zone: Vec<NearestPointResult>,
    pub points_outside_zone: Vec<NearestPointResult>,
    pub summary: ProximityZoneSummary,
}

#[derive(Serialize)]
pub struct ProximityZoneSummary {
    pub total_points: usize,
    pub points_inside: usize,
    pub points_outside: usize,
    pub average_distance_inside: f64,
    pub closest_point_distance: f64,
    pub farthest_point_distance: f64,
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

pub fn find_nearest_points(input: NearestPointsInput) -> Result<NearestPointsResult, String> {
    if input.candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }
    
    // Validate query point
    if input.query_point.lat < -90.0 || input.query_point.lat > 90.0 {
        return Err(format!("Invalid query point latitude: {}. Must be between -90 and 90", input.query_point.lat));
    }
    if input.query_point.lon < -180.0 || input.query_point.lon > 180.0 {
        return Err(format!("Invalid query point longitude: {}. Must be between -180 and 180", input.query_point.lon));
    }
    
    let mut distances: Vec<(usize, f64)> = Vec::new();
    
    for (i, candidate) in input.candidate_points.iter().enumerate() {
        // Validate candidate coordinates
        if candidate.lat < -90.0 || candidate.lat > 90.0 {
            return Err(format!("Invalid candidate latitude: {}. Must be between -90 and 90", candidate.lat));
        }
        if candidate.lon < -180.0 || candidate.lon > 180.0 {
            return Err(format!("Invalid candidate longitude: {}. Must be between -180 and 180", candidate.lon));
        }
        
        let distance = haversine_distance(&input.query_point, candidate);
        
        // Apply distance filter if specified
        if let Some(max_dist) = input.max_distance_meters {
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
    let max_results = input.max_results.unwrap_or(distances.len()).min(distances.len());
    
    let mut nearest_points = Vec::new();
    for i in 0..max_results {
        let (idx, distance) = distances[i];
        let candidate = &input.candidate_points[idx];
        let bearing = calculate_bearing(&input.query_point, candidate);
        
        nearest_points.push(NearestPointResult {
            point: candidate.clone(),
            distance_meters: distance,
            bearing_degrees: bearing,
        });
    }
    
    Ok(NearestPointsResult {
        query_point: input.query_point,
        nearest_points,
        total_candidates: input.candidate_points.len(),
        results_returned: max_results,
    })
}

pub fn distance_to_polygon(input: DistanceToPolygonInput) -> Result<DistanceToPolygonResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }
    
    // Validate coordinates
    if input.point.lat < -90.0 || input.point.lat > 90.0 {
        return Err(format!("Invalid point latitude: {}. Must be between -90 and 90", input.point.lat));
    }
    if input.point.lon < -180.0 || input.point.lon > 180.0 {
        return Err(format!("Invalid point longitude: {}. Must be between -180 and 180", input.point.lon));
    }
    
    for poly_point in &input.polygon {
        if poly_point.lat < -90.0 || poly_point.lat > 90.0 {
            return Err(format!("Invalid polygon latitude: {}. Must be between -90 and 90", poly_point.lat));
        }
        if poly_point.lon < -180.0 || poly_point.lon > 180.0 {
            return Err(format!("Invalid polygon longitude: {}. Must be between -180 and 180", poly_point.lon));
        }
    }
    
    // Check if point is inside polygon first
    let is_inside = point_in_polygon_ray_casting(&input.point, &input.polygon);
    
    if is_inside {
        // Point is inside, distance is 0
        return Ok(DistanceToPolygonResult {
            distance_meters: 0.0,
            closest_point_on_polygon: input.point.clone(),
            is_inside_polygon: true,
            closest_segment_index: 0,
        });
    }
    
    // Find closest point on polygon boundary
    let mut min_distance = f64::INFINITY;
    let mut closest_point = input.point.clone();
    let mut closest_segment = 0;
    
    let n = input.polygon.len();
    for i in 0..n {
        let j = (i + 1) % n;
        let seg_start = &input.polygon[i];
        let seg_end = &input.polygon[j];
        
        let (dist, point_on_seg) = distance_to_line_segment(&input.point, seg_start, seg_end);
        
        if dist < min_distance {
            min_distance = dist;
            closest_point = point_on_seg;
            closest_segment = i;
        }
    }
    
    Ok(DistanceToPolygonResult {
        distance_meters: min_distance,
        closest_point_on_polygon: closest_point,
        is_inside_polygon: false,
        closest_segment_index: closest_segment,
    })
}

fn point_in_polygon_ray_casting(point: &Point, polygon: &[Point]) -> bool {
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

fn distance_to_line_segment(point: &Point, seg_start: &Point, seg_end: &Point) -> (f64, Point) {
    // Simplified distance calculation - projects point onto line segment
    let a = haversine_distance(point, seg_start);
    let b = haversine_distance(point, seg_end);
    let c = haversine_distance(seg_start, seg_end);
    
    if c == 0.0 {
        return (a, seg_start.clone());
    }
    
    // Use dot product to find projection
    let t = ((point.lat - seg_start.lat) * (seg_end.lat - seg_start.lat) + 
             (point.lon - seg_start.lon) * (seg_end.lon - seg_start.lon)) / 
            ((seg_end.lat - seg_start.lat).powi(2) + (seg_end.lon - seg_start.lon).powi(2));
    
    let t_clamped = t.max(0.0).min(1.0);
    
    let closest_point = Point {
        lat: seg_start.lat + t_clamped * (seg_end.lat - seg_start.lat),
        lon: seg_start.lon + t_clamped * (seg_end.lon - seg_start.lon),
        id: None,
    };
    
    let distance = haversine_distance(point, &closest_point);
    (distance, closest_point)
}

pub fn proximity_zone_analysis(input: ProximityZoneInput) -> Result<ProximityZoneResult, String> {
    if input.radius_meters <= 0.0 {
        return Err("Radius must be positive".to_string());
    }
    
    if input.candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }
    
    // Validate center coordinates
    if input.center.lat < -90.0 || input.center.lat > 90.0 {
        return Err(format!("Invalid center latitude: {}. Must be between -90 and 90", input.center.lat));
    }
    if input.center.lon < -180.0 || input.center.lon > 180.0 {
        return Err(format!("Invalid center longitude: {}. Must be between -180 and 180", input.center.lon));
    }
    
    let mut points_inside = Vec::new();
    let mut points_outside = Vec::new();
    let mut distances_inside = Vec::new();
    let mut all_distances = Vec::new();
    
    for candidate in input.candidate_points {
        // Validate candidate coordinates
        if candidate.lat < -90.0 || candidate.lat > 90.0 {
            return Err(format!("Invalid candidate latitude: {}. Must be between -90 and 90", candidate.lat));
        }
        if candidate.lon < -180.0 || candidate.lon > 180.0 {
            return Err(format!("Invalid candidate longitude: {}. Must be between -180 and 180", candidate.lon));
        }
        
        let distance = haversine_distance(&input.center, &candidate);
        let bearing = calculate_bearing(&input.center, &candidate);
        all_distances.push(distance);
        
        let result = NearestPointResult {
            point: candidate,
            distance_meters: distance,
            bearing_degrees: bearing,
        };
        
        if distance <= input.radius_meters {
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
    
    let closest_distance = all_distances.iter().fold(f64::INFINITY, |a, &b| a.min(b));
    let farthest_distance = all_distances.iter().fold(0.0f64, |a, &b| a.max(b));
    
    Ok(ProximityZoneResult {
        center: input.center,
        radius_meters: input.radius_meters,
        points_in_zone: points_inside,
        points_outside_zone: points_outside,
        summary: ProximityZoneSummary {
            total_points,
            points_inside: distances_inside.len(),
            points_outside: total_points - distances_inside.len(),
            average_distance_inside,
            closest_point_distance: closest_distance,
            farthest_point_distance: farthest_distance,
        },
    })
}