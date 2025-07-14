use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema, Clone, Debug, PartialEq)]
struct Vector3D {
    /// X component of the vector
    x: f64,
    /// Y component of the vector
    y: f64,
    /// Z component of the vector
    z: f64,
}

#[derive(Deserialize, JsonSchema)]
struct DotProductInput {
    /// First 3D vector
    vector1: Vector3D,
    /// Second 3D vector
    vector2: Vector3D,
}

#[derive(Serialize)]
struct DotProductResult {
    dot_product: f64,
    angle_radians: f64,
    angle_degrees: f64,
    are_perpendicular: bool,
    are_parallel: bool,
}

impl Vector3D {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
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

    fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }

    fn are_parallel(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        let cross = self.cross(other);
        cross.magnitude() < EPSILON
    }

    fn are_perpendicular(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        self.dot(other).abs() < EPSILON
    }

    fn angle_with(&self, other: &Vector3D) -> Result<f64, String> {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();
        
        if mag1 == 0.0 || mag2 == 0.0 {
            return Err("Cannot compute angle with zero vector".to_string());
        }
        
        let cos_angle = self.dot(other) / (mag1 * mag2);
        // Clamp to [-1, 1] to handle floating point errors
        let cos_angle = cos_angle.max(-1.0).min(1.0);
        Ok(cos_angle.acos())
    }
}

fn compute_dot_product(vector1: Vector3D, vector2: Vector3D) -> Result<DotProductResult, String> {
    let dot_product = vector1.dot(&vector2);
    let are_perpendicular = vector1.are_perpendicular(&vector2);
    let are_parallel = vector1.are_parallel(&vector2);
    
    let (angle_radians, angle_degrees) = if vector1.is_zero() || vector2.is_zero() {
        (0.0, 0.0)
    } else {
        match vector1.angle_with(&vector2) {
            Ok(angle_rad) => (angle_rad, angle_rad.to_degrees()),
            Err(_) => (0.0, 0.0),
        }
    };
    
    Ok(DotProductResult {
        dot_product,
        angle_radians,
        angle_degrees,
        are_perpendicular,
        are_parallel,
    })
}

/// Calculate dot product of two 3D vectors
#[tool]
fn dot_product(input: DotProductInput) -> ToolResponse {
    match compute_dot_product(input.vector1, input.vector2) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}