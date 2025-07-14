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
struct VectorMagnitudeInput {
    /// 3D vector to calculate magnitude for
    vector: Vector3D,
}

#[derive(Serialize)]
struct VectorMagnitudeResult {
    magnitude: f64,
    unit_vector: Vector3D,
    is_zero_vector: bool,
}

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn normalize(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }

    fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }
}

fn compute_vector_magnitude(vector: Vector3D) -> Result<VectorMagnitudeResult, String> {
    let magnitude = vector.magnitude();
    let is_zero_vector = vector.is_zero();
    
    let unit_vector = if is_zero_vector {
        Vector3D::new(0.0, 0.0, 0.0)
    } else {
        vector.normalize()?
    };
    
    Ok(VectorMagnitudeResult {
        magnitude,
        unit_vector,
        is_zero_vector,
    })
}

/// Calculate magnitude and unit vector of a 3D vector
#[tool]
fn vector_magnitude(input: VectorMagnitudeInput) -> ToolResponse {
    match compute_vector_magnitude(input.vector) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}