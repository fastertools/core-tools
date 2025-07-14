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
struct TwoPlaneInput {
    /// First plane
    plane1: Plane3D,
    /// Second plane
    plane2: Plane3D,
}

#[derive(Serialize)]
struct PlanePlaneIntersectionResult {
    /// Type of intersection: "intersecting", "parallel", or "coincident"
    intersection_type: String,
    /// Whether the planes intersect
    intersects: bool,
    /// The intersection line if planes intersect
    intersection_line: Option<Line3D>,
    /// Whether the planes are parallel
    are_parallel: bool,
    /// Whether the planes are coincident (same plane)
    are_coincident: bool,
    /// Angle between planes in radians
    angle_radians: f64,
    /// Angle between planes in degrees
    angle_degrees: f64,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
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

    fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Line3D {
    fn new(point: Vector3D, direction: Vector3D) -> Result<Self, String> {
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }
}

impl Plane3D {
    fn is_parallel_to(&self, other: &Plane3D) -> bool {
        let cross = self.normal.cross(&other.normal);
        cross.is_zero()
    }

    fn angle_with(&self, other: &Plane3D) -> Result<f64, String> {
        let n1 = self.normal.normalize()?;
        let n2 = other.normal.normalize()?;
        let dot = n1.dot(&n2);
        let clamped = dot.max(-1.0).min(1.0);
        Ok(clamped.acos())
    }

    fn distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };
        
        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit).abs()
    }
}

/// Calculate the intersection between two 3D planes
/// Returns detailed information about the intersection including the line of intersection if it exists
#[tool]
fn plane_plane_intersection(input: TwoPlaneInput) -> ToolResponse {
    let plane1 = &input.plane1;
    let plane2 = &input.plane2;

    // Validate inputs
    if plane1.normal.is_zero() {
        return ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": "Plane1 normal vector cannot be zero"
        })).unwrap());
    }
    if plane2.normal.is_zero() {
        return ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": "Plane2 normal vector cannot be zero"
        })).unwrap());
    }

    let are_parallel = plane1.is_parallel_to(plane2);
    let angle_radians = plane1.angle_with(plane2).unwrap_or(0.0);
    let angle_degrees = angle_radians.to_degrees();

    if are_parallel {
        // Check if planes are coincident
        let distance = plane1.distance_to_point(&plane2.point);
        let are_coincident = distance < EPSILON;

        let result = PlanePlaneIntersectionResult {
            intersection_type: if are_coincident { 
                "coincident".to_string() 
            } else { 
                "parallel".to_string() 
            },
            intersects: are_coincident,
            intersection_line: None,
            are_parallel: true,
            are_coincident,
            angle_radians,
            angle_degrees,
        };

        return ToolResponse::text(serde_json::to_string(&result).unwrap());
    }

    // Planes intersect in a line
    let direction = plane1.normal.cross(&plane2.normal);
    
    // Find a point on the intersection line
    // We'll find the point closest to the origin that lies on both planes
    let n1 = &plane1.normal;
    let n2 = &plane2.normal;
    let d1 = n1.dot(&plane1.point);
    let d2 = n2.dot(&plane2.point);

    // Find the direction with the largest component to avoid division by small numbers
    let abs_dir = Vector3D::new(direction.x.abs(), direction.y.abs(), direction.z.abs());
    
    let intersection_point = if abs_dir.z >= abs_dir.x && abs_dir.z >= abs_dir.y {
        // Solve for x and y, set z = 0
        let det = n1.x * n2.y - n1.y * n2.x;
        if det.abs() < EPSILON {
            return ToolResponse::text(serde_json::to_string(&serde_json::json!({
                "error": "Cannot find intersection point"
            })).unwrap());
        }
        let x = (d1 * n2.y - d2 * n1.y) / det;
        let y = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, y, 0.0)
    } else if abs_dir.y >= abs_dir.x {
        // Solve for x and z, set y = 0
        let det = n1.x * n2.z - n1.z * n2.x;
        if det.abs() < EPSILON {
            return ToolResponse::text(serde_json::to_string(&serde_json::json!({
                "error": "Cannot find intersection point"
            })).unwrap());
        }
        let x = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, 0.0, z)
    } else {
        // Solve for y and z, set x = 0
        let det = n1.y * n2.z - n1.z * n2.y;
        if det.abs() < EPSILON {
            return ToolResponse::text(serde_json::to_string(&serde_json::json!({
                "error": "Cannot find intersection point"
            })).unwrap());
        }
        let y = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.y - d1 * n2.y) / det;
        Vector3D::new(0.0, y, z)
    };

    let intersection_line = match Line3D::new(intersection_point, direction) {
        Ok(line) => line,
        Err(e) => {
            return ToolResponse::text(serde_json::to_string(&serde_json::json!({
                "error": e
            })).unwrap());
        }
    };

    let result = PlanePlaneIntersectionResult {
        intersection_type: "intersecting".to_string(),
        intersects: true,
        intersection_line: Some(intersection_line),
        are_parallel: false,
        are_coincident: false,
        angle_radians,
        angle_degrees,
    };

    ToolResponse::text(serde_json::to_string(&result).unwrap())
}