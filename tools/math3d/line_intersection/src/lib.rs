use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
struct Vector3D {
    /// X component of the vector
    x: f64,
    /// Y component of the vector
    y: f64,
    /// Z component of the vector
    z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
struct Line3D {
    /// A point on the line
    point: Vector3D,
    /// Direction vector of the line
    direction: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
struct LineIntersectionInput {
    /// First 3D line
    line1: Line3D,
    /// Second 3D line
    line2: Line3D,
}

#[derive(Serialize)]
struct LineIntersectionResult {
    intersection_type: String,
    intersects: bool,
    intersection_point: Option<Vector3D>,
    closest_point_line1: Vector3D,
    closest_point_line2: Vector3D,
    minimum_distance: f64,
    parameter_line1: f64,
    parameter_line2: f64,
    are_parallel: bool,
    are_skew: bool,
    are_coincident: bool,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn distance_to(&self, other: &Vector3D) -> f64 {
        self.subtract(other).magnitude()
    }

    fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    fn are_parallel(&self, other: &Vector3D) -> bool {
        let cross = Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
        cross.magnitude() < EPSILON
    }
}

impl Line3D {
    fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
    }

    fn is_parallel_to(&self, other: &Line3D) -> bool {
        self.direction.are_parallel(&other.direction)
    }
}

fn closest_points_skew_lines(line1: &Line3D, line2: &Line3D) -> (f64, f64, Vector3D, Vector3D, f64) {
    let d1 = &line1.direction;
    let d2 = &line2.direction;
    let w = line1.point.subtract(&line2.point);
    
    let a = d1.dot(d1);
    let b = d1.dot(d2);
    let c = d2.dot(d2);
    let d = d1.dot(&w);
    let e = d2.dot(&w);
    
    let denominator = a * c - b * b;
    
    let (t1, t2) = if denominator.abs() < EPSILON {
        // Lines are parallel (shouldn't happen here, but safety check)
        (0.0, 0.0)
    } else {
        let t1 = (b * e - c * d) / denominator;
        let t2 = (a * e - b * d) / denominator;
        (t1, t2)
    };
    
    let closest1 = line1.point_at_parameter(t1);
    let closest2 = line2.point_at_parameter(t2);
    let distance = closest1.distance_to(&closest2);
    
    (t1, t2, closest1, closest2, distance)
}

fn closest_points_parallel_lines(line1: &Line3D, line2: &Line3D) -> (f64, f64, f64) {
    let w = line2.point.subtract(&line1.point);
    let d1 = &line1.direction;
    
    let t1 = d1.dot(&w) / d1.dot(d1);
    let closest1 = line1.point_at_parameter(t1);
    let distance = closest1.distance_to(&line2.point);
    
    (t1, 0.0, distance)
}

fn detect_line_intersection(line1: Line3D, line2: Line3D) -> Result<LineIntersectionResult, String> {
    // Validate direction vectors
    if line1.direction.is_zero() {
        return Err("Line1 direction vector cannot be zero".to_string());
    }
    if line2.direction.is_zero() {
        return Err("Line2 direction vector cannot be zero".to_string());
    }

    let are_parallel = line1.is_parallel_to(&line2);
    
    if are_parallel {
        // Check if lines are coincident (same line)
        let point_diff = line2.point.subtract(&line1.point);
        let are_coincident = point_diff.are_parallel(&line1.direction) || point_diff.is_zero();
        
        if are_coincident {
            return Ok(LineIntersectionResult {
                intersection_type: "coincident".to_string(),
                intersects: true,
                intersection_point: Some(line1.point.clone()),
                closest_point_line1: line1.point.clone(),
                closest_point_line2: line2.point.clone(),
                minimum_distance: 0.0,
                parameter_line1: 0.0,
                parameter_line2: 0.0,
                are_parallel: true,
                are_skew: false,
                are_coincident: true,
            });
        } else {
            // Parallel but not coincident - find closest points
            let (t1, _t2, dist) = closest_points_parallel_lines(&line1, &line2);
            let closest1 = line1.point_at_parameter(t1);
            
            return Ok(LineIntersectionResult {
                intersection_type: "parallel".to_string(),
                intersects: false,
                intersection_point: None,
                closest_point_line1: closest1.clone(),
                closest_point_line2: closest1.clone(), // Project onto line2
                minimum_distance: dist,
                parameter_line1: t1,
                parameter_line2: 0.0,
                are_parallel: true,
                are_skew: false,
                are_coincident: false,
            });
        }
    }

    // Lines are not parallel - find closest points
    let (t1, t2, closest1, closest2, distance) = closest_points_skew_lines(&line1, &line2);
    
    let intersects = distance < EPSILON;
    let intersection_point = if intersects {
        Some(closest1.clone())
    } else {
        None
    };

    let intersection_type = if intersects {
        "intersecting".to_string()
    } else {
        "skew".to_string()
    };

    Ok(LineIntersectionResult {
        intersection_type,
        intersects,
        intersection_point,
        closest_point_line1: closest1,
        closest_point_line2: closest2,
        minimum_distance: distance,
        parameter_line1: t1,
        parameter_line2: t2,
        are_parallel: false,
        are_skew: !intersects,
        are_coincident: false,
    })
}

/// Find intersection of two 3D lines
#[tool]
fn line_intersection(input: LineIntersectionInput) -> ToolResponse {
    match detect_line_intersection(input.line1, input.line2) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}