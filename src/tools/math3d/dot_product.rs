use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct DotProductInput {
    /// First 3D vector
    vector1: Vector3D,
    /// Second 3D vector
    vector2: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
struct Vector3D {
    /// X component
    x: f64,
    /// Y component
    y: f64,
    /// Z component
    z: f64,
}

/// Calculate the dot product of two 3D vectors
#[tool]
fn dot_product_3d(input: DotProductInput) -> ToolResponse {
    use crate::math_3d::vector_ops::{TwoVectorInput, Vector3D as InternalVector, compute_dot_product};
    
    let internal_input = TwoVectorInput {
        vector1: InternalVector {
            x: input.vector1.x,
            y: input.vector1.y,
            z: input.vector1.z,
        },
        vector2: InternalVector {
            x: input.vector2.x,
            y: input.vector2.y,
            z: input.vector2.z,
        },
    };
    
    match compute_dot_product(internal_input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}