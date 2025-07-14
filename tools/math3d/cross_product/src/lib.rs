use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, PartialEq)]
struct Vector3D {
    /// X component of the vector
    x: f64,
    /// Y component of the vector
    y: f64,
    /// Z component of the vector
    z: f64,
}

#[derive(Deserialize, JsonSchema)]
struct CrossProductInput {
    /// First 3D vector
    vector1: Vector3D,
    /// Second 3D vector
    vector2: Vector3D,
}

#[derive(Serialize)]
struct CrossProductResult {
    cross_product: Vector3D,
    magnitude: f64,
    area_parallelogram: f64,
    are_parallel: bool,
}

impl Vector3D {
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    fn are_parallel(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        let cross = self.cross(other);
        cross.magnitude() < EPSILON
    }
}

fn compute_cross_product(vector1: Vector3D, vector2: Vector3D) -> CrossProductResult {
    let cross_product = vector1.cross(&vector2);
    let magnitude = cross_product.magnitude();
    let area_parallelogram = magnitude;
    let are_parallel = vector1.are_parallel(&vector2);
    
    CrossProductResult {
        cross_product,
        magnitude,
        area_parallelogram,
        are_parallel,
    }
}

/// Calculate cross product of two 3D vectors
#[tool]
fn cross_product(input: CrossProductInput) -> ToolResponse {
    let result = compute_cross_product(input.vector1, input.vector2);
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}