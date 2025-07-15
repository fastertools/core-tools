use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

const EPSILON: f64 = 1e-10;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Line3D {
    pub point: Vector3D,     // A point on the line
    pub direction: Vector3D, // Direction vector of the line
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointLineInput {
    pub point: Vector3D,
    pub line: Line3D,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointLineDistanceResult {
    pub distance: f64,
    pub closest_point_on_line: Vector3D,
    pub parameter_on_line: f64,
    pub perpendicular_vector: Vector3D,
    pub point_is_on_line: bool,
}

impl Vector3D {
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
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

    pub fn is_zero(&self) -> bool {
        self.magnitude_squared() < EPSILON * EPSILON
    }
}

impl Line3D {
    pub fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
    }
}

#[tool]
pub fn point_line_distance(input: PointLineInput) -> Result<PointLineDistanceResult, String> {
    let point = &input.point;
    let line = &input.line;

    // Validate line direction
    if line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }

    // Vector from line point to query point
    let to_point = point.subtract(&line.point);
    
    // Project this vector onto the line direction to find the closest point
    let line_dir_mag_sq = line.direction.magnitude_squared();
    let t = to_point.dot(&line.direction) / line_dir_mag_sq;
    
    // Find closest point on line
    let closest_point_on_line = line.point_at_parameter(t);
    
    // Calculate distance and perpendicular vector
    let perpendicular_vector = point.subtract(&closest_point_on_line);
    let distance = perpendicular_vector.magnitude();
    
    // Check if point is on line
    let point_is_on_line = distance < EPSILON;

    Ok(PointLineDistanceResult {
        distance,
        closest_point_on_line,
        parameter_on_line: t,
        perpendicular_vector,
        point_is_on_line,
    })
}