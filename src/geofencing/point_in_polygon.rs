use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct PointInPolygonInput {
    pub point: Point,
    pub polygon: Vec<Point>,
}

#[derive(Deserialize)]
pub struct MultiPointInput {
    pub points: Vec<Point>,
    pub polygon: Vec<Point>,
}

#[derive(Serialize)]
pub struct PointInPolygonResult {
    pub is_inside: bool,
    pub algorithm_used: String,
    pub on_boundary: bool,
}

#[derive(Serialize)]
pub struct MultiPointResult {
    pub results: Vec<PointInPolygonSingleResult>,
    pub summary: PointInPolygonSummary,
}

#[derive(Serialize)]
pub struct PointInPolygonSingleResult {
    pub point: Point,
    pub is_inside: bool,
    pub on_boundary: bool,
}

#[derive(Serialize)]
pub struct PointInPolygonSummary {
    pub total_points: usize,
    pub points_inside: usize,
    pub points_outside: usize,
    pub points_on_boundary: usize,
}

const EPSILON: f64 = 1e-10;

pub fn ray_casting_algorithm(point: &Point, polygon: &[Point]) -> bool {
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

pub fn winding_number_algorithm(point: &Point, polygon: &[Point]) -> i32 {
    if polygon.len() < 3 {
        return 0;
    }
    
    let x = point.lon;
    let y = point.lat;
    let mut wn = 0;
    let n = polygon.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let xi = polygon[i].lon;
        let yi = polygon[i].lat;
        let xj = polygon[j].lon;
        let yj = polygon[j].lat;
        
        if yi <= y {
            if yj > y {
                // Upward crossing
                if is_left(&Point { lat: yi, lon: xi }, &Point { lat: yj, lon: xj }, point) > 0.0 {
                    wn += 1;
                }
            }
        } else {
            if yj <= y {
                // Downward crossing
                if is_left(&Point { lat: yi, lon: xi }, &Point { lat: yj, lon: xj }, point) < 0.0 {
                    wn -= 1;
                }
            }
        }
    }
    
    wn
}

fn is_left(p0: &Point, p1: &Point, p2: &Point) -> f64 {
    (p1.lon - p0.lon) * (p2.lat - p0.lat) - (p2.lon - p0.lon) * (p1.lat - p0.lat)
}

pub fn is_on_boundary(point: &Point, polygon: &[Point]) -> bool {
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

pub fn point_in_polygon_check(input: PointInPolygonInput, use_winding: bool) -> Result<PointInPolygonResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
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
    
    if input.point.lat < -90.0 || input.point.lat > 90.0 {
        return Err(format!("Invalid point latitude: {}. Must be between -90 and 90", input.point.lat));
    }
    if input.point.lon < -180.0 || input.point.lon > 180.0 {
        return Err(format!("Invalid point longitude: {}. Must be between -180 and 180", input.point.lon));
    }
    
    let on_boundary = is_on_boundary(&input.point, &input.polygon);
    
    let is_inside = if use_winding {
        winding_number_algorithm(&input.point, &input.polygon) != 0
    } else {
        ray_casting_algorithm(&input.point, &input.polygon)
    };
    
    let algorithm_used = if use_winding { "winding_number" } else { "ray_casting" }.to_string();
    
    Ok(PointInPolygonResult {
        is_inside,
        algorithm_used,
        on_boundary,
    })
}

pub fn multi_point_check(input: MultiPointInput, use_winding: bool) -> Result<MultiPointResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }
    
    if input.points.is_empty() {
        return Err("At least one point must be provided".to_string());
    }
    
    // Validate polygon coordinates
    for point in &input.polygon {
        if point.lat < -90.0 || point.lat > 90.0 {
            return Err(format!("Invalid polygon latitude: {}. Must be between -90 and 90", point.lat));
        }
        if point.lon < -180.0 || point.lon > 180.0 {
            return Err(format!("Invalid polygon longitude: {}. Must be between -180 and 180", point.lon));
        }
    }
    
    let mut results = Vec::new();
    let mut inside_count = 0;
    let mut outside_count = 0;
    let mut boundary_count = 0;
    
    for point in input.points {
        // Validate point coordinates
        if point.lat < -90.0 || point.lat > 90.0 {
            return Err(format!("Invalid point latitude: {}. Must be between -90 and 90", point.lat));
        }
        if point.lon < -180.0 || point.lon > 180.0 {
            return Err(format!("Invalid point longitude: {}. Must be between -180 and 180", point.lon));
        }
        
        let on_boundary = is_on_boundary(&point, &input.polygon);
        
        let is_inside = if use_winding {
            winding_number_algorithm(&point, &input.polygon) != 0
        } else {
            ray_casting_algorithm(&point, &input.polygon)
        };
        
        if on_boundary {
            boundary_count += 1;
        } else if is_inside {
            inside_count += 1;
        } else {
            outside_count += 1;
        }
        
        results.push(PointInPolygonSingleResult {
            point,
            is_inside,
            on_boundary,
        });
    }
    
    Ok(MultiPointResult {
        results,
        summary: PointInPolygonSummary {
            total_points: inside_count + outside_count + boundary_count,
            points_inside: inside_count,
            points_outside: outside_count,
            points_on_boundary: boundary_count,
        },
    })
}