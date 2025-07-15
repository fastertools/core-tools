use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Line3D {
    pub point: Vector3D,
    pub direction: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
pub struct LineSegmentInput {
    pub segment1_start: Vector3D,
    pub segment1_end: Vector3D,
    pub segment2_start: Vector3D,
    pub segment2_end: Vector3D,
}

#[derive(Serialize, JsonSchema)]
pub struct LineSegmentIntersectionResult {
    pub intersects: bool,
    pub intersection_point: Option<Vector3D>,
    pub closest_point_seg1: Vector3D,
    pub closest_point_seg2: Vector3D,
    pub minimum_distance: f64,
    pub intersection_on_both_segments: bool,
    pub segment1_parameter: f64,
    pub segment2_parameter: f64,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn distance_to(&self, other: &Vector3D) -> f64 {
        self.subtract(other).magnitude()
    }
}

impl Line3D {
    pub fn new(point: Vector3D, direction: Vector3D) -> Result<Self, String> {
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
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
    
    let denom = a * c - b * b;
    
    let (t1, t2) = if denom.abs() < EPSILON {
        // Lines are parallel
        (0.0, d / c)
    } else {
        let t1 = (b * e - c * d) / denom;
        let t2 = (a * e - b * d) / denom;
        (t1, t2)
    };
    
    let closest1 = line1.point_at_parameter(t1);
    let closest2 = line2.point_at_parameter(t2);
    let distance = closest1.distance_to(&closest2);
    
    (t1, t2, closest1, closest2, distance)
}

#[tool]
pub fn line_segment_intersection(input: LineSegmentInput) -> Result<LineSegmentIntersectionResult, String> {
    // Convert segments to lines
    let dir1 = input.segment1_end.subtract(&input.segment1_start);
    let dir2 = input.segment2_end.subtract(&input.segment2_start);
    
    if dir1.is_zero() {
        return Err("Segment 1 has zero length".to_string());
    }
    if dir2.is_zero() {
        return Err("Segment 2 has zero length".to_string());
    }
    
    let line1 = Line3D::new(input.segment1_start.clone(), dir1)?;
    let line2 = Line3D::new(input.segment2_start.clone(), dir2)?;
    
    let (t1, t2, closest1, closest2, distance) = closest_points_skew_lines(&line1, &line2);
    
    // Check if parameters are within segment bounds [0, 1]
    let t1_in_bounds = t1 >= 0.0 && t1 <= 1.0;
    let t2_in_bounds = t2 >= 0.0 && t2 <= 1.0;
    let intersection_on_both_segments = t1_in_bounds && t2_in_bounds;
    
    // Clamp parameters to segment bounds for final closest points
    let t1_clamped = t1.max(0.0).min(1.0);
    let t2_clamped = t2.max(0.0).min(1.0);
    
    let final_closest1 = line1.point_at_parameter(t1_clamped);
    let final_closest2 = line2.point_at_parameter(t2_clamped);
    let final_distance = final_closest1.distance_to(&final_closest2);
    
    let intersects = intersection_on_both_segments && distance < EPSILON;
    let intersection_point = if intersects {
        Some(final_closest1.clone())
    } else {
        None
    };
    
    Ok(LineSegmentIntersectionResult {
        intersects,
        intersection_point,
        closest_point_seg1: final_closest1,
        closest_point_seg2: final_closest2,
        minimum_distance: final_distance,
        intersection_on_both_segments,
        segment1_parameter: t1_clamped,
        segment2_parameter: t2_clamped,
    })
}