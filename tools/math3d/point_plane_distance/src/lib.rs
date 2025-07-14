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
struct Plane3D {
    /// A point on the plane
    point: Vector3D,
    /// Normal vector to the plane
    normal: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
struct PointPlaneInput {
    /// The point to measure distance from
    point: Vector3D,
    /// The plane to measure distance to
    plane: Plane3D,
}

#[derive(Serialize)]
struct PointPlaneResult {
    /// Absolute distance from point to plane
    distance: f64,
    /// Signed distance (positive if point is on the side of normal, negative otherwise)
    signed_distance: f64,
    /// Closest point on the plane to the given point
    closest_point_on_plane: Vector3D,
    /// Whether the point lies exactly on the plane
    is_on_plane: bool,
    /// Which side of the plane the point is on
    side_of_plane: String,
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

impl Plane3D {
    fn distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };
        
        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit).abs()
    }

    fn signed_distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };
        
        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit)
    }

    fn project_point(&self, point: &Vector3D) -> Vector3D {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return point.clone(),
        };
        
        let signed_dist = self.signed_distance_to_point(point);
        point.subtract(&normal_unit.scale(signed_dist))
    }
}

/// Calculate the distance from a point to a plane in 3D space
/// Returns both signed and unsigned distance, the closest point on the plane, and which side of the plane the point is on
#[tool]
fn point_plane_distance(input: PointPlaneInput) -> ToolResponse {
    let point = &input.point;
    let plane = &input.plane;

    if plane.normal.is_zero() {
        return ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": "Plane normal vector cannot be zero"
        })).unwrap());
    }

    let distance = plane.distance_to_point(point);
    let signed_distance = plane.signed_distance_to_point(point);
    let closest_point_on_plane = plane.project_point(point);
    let is_on_plane = distance < EPSILON;
    
    let side_of_plane = if is_on_plane {
        "on_plane".to_string()
    } else if signed_distance > 0.0 {
        "positive".to_string()
    } else {
        "negative".to_string()
    };

    let result = PointPlaneResult {
        distance,
        signed_distance,
        closest_point_on_plane,
        is_on_plane,
        side_of_plane,
    };

    ToolResponse::text(serde_json::to_string(&result).unwrap())
}