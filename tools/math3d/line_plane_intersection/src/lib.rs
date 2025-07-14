use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
struct Line3D {
    /// A point on the line
    point: Vector3D,
    /// Direction vector of the line
    direction: Vector3D,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
struct Plane3D {
    /// A point on the plane
    point: Vector3D,
    /// Normal vector to the plane
    normal: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
struct LinePlaneInput {
    /// The line to test for intersection
    line: Line3D,
    /// The plane to test against
    plane: Plane3D,
}

#[derive(Serialize)]
struct LinePlaneIntersectionResult {
    /// Type of intersection: "point", "line_in_plane", or "no_intersection"
    intersection_type: String,
    /// Whether the line intersects the plane
    intersects: bool,
    /// The intersection point if it exists
    intersection_point: Option<Vector3D>,
    /// Parameter t where intersection occurs (line_point = line.point + t * line.direction)
    parameter: Option<f64>,
    /// Whether the line is parallel to the plane
    line_is_parallel: bool,
    /// Whether the line lies entirely in the plane
    line_is_in_plane: bool,
    /// Distance from line to plane (0 if intersecting)
    distance_to_plane: f64,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag < EPSILON {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(self.scale(1.0 / mag))
    }

    fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }
}

fn distance_point_to_plane(point: &Vector3D, plane: &Plane3D) -> f64 {
    let normal_unit = match plane.normal.normalize() {
        Ok(n) => n,
        Err(_) => return 0.0,
    };
    
    let to_point = point.subtract(&plane.point);
    to_point.dot(&normal_unit).abs()
}

/// Calculate the intersection between a 3D line and a plane
/// Returns detailed information about the intersection including type, point, and geometric relationships
#[tool]
fn line_plane_intersection(input: LinePlaneInput) -> ToolResponse {
    // Validate inputs
    if input.line.direction.is_zero() {
        return ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": "Line direction vector cannot be zero"
        })).unwrap());
    }

    if input.plane.normal.is_zero() {
        return ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": "Plane normal vector cannot be zero"
        })).unwrap());
    }

    // Calculate dot product of line direction and plane normal
    let dot_product = input.line.direction.dot(&input.plane.normal);
    
    // Check if line is parallel to plane (direction perpendicular to normal)
    let is_parallel = dot_product.abs() < EPSILON;
    
    if is_parallel {
        // Line is parallel to plane
        // Check if line is in the plane
        let point_to_plane = input.line.point.subtract(&input.plane.point);
        let distance = point_to_plane.dot(&input.plane.normal).abs();
        
        let normal_mag = input.plane.normal.magnitude();
        let normalized_distance = if normal_mag > EPSILON {
            distance / normal_mag
        } else {
            0.0
        };
        
        let is_in_plane = normalized_distance < EPSILON;
        
        let result = LinePlaneIntersectionResult {
            intersection_type: if is_in_plane { 
                "line_in_plane".to_string() 
            } else { 
                "no_intersection".to_string() 
            },
            intersects: is_in_plane,
            intersection_point: None,
            parameter: None,
            line_is_parallel: true,
            line_is_in_plane: is_in_plane,
            distance_to_plane: if is_in_plane { 0.0 } else { normalized_distance },
        };
        
        return ToolResponse::text(serde_json::to_string(&result).unwrap());
    }
    
    // Line is not parallel - calculate intersection point
    // Using parametric form: P = line.point + t * line.direction
    // Plane equation: (P - plane.point) · plane.normal = 0
    // Substituting: ((line.point + t * line.direction) - plane.point) · plane.normal = 0
    // Solving for t: t = (plane.point - line.point) · plane.normal / (line.direction · plane.normal)
    
    let point_diff = input.plane.point.subtract(&input.line.point);
    let t = point_diff.dot(&input.plane.normal) / dot_product;
    
    // Calculate intersection point
    let intersection_point = input.line.point.add(&input.line.direction.scale(t));
    
    let result = LinePlaneIntersectionResult {
        intersection_type: "point".to_string(),
        intersects: true,
        intersection_point: Some(intersection_point),
        parameter: Some(t),
        line_is_parallel: false,
        line_is_in_plane: false,
        distance_to_plane: 0.0,
    };
    
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}