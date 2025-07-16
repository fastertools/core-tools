use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{dot_product_logic, DotProductInput as LogicInput, DotProductResult, Vector3D as LogicVector3D};

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

impl From<Vector3D> for LogicVector3D {
    fn from(v: Vector3D) -> Self {
        LogicVector3D { x: v.x, y: v.y, z: v.z }
    }
}

impl From<DotProductInput> for LogicInput {
    fn from(input: DotProductInput) -> Self {
        LogicInput {
            vector1: input.vector1.into(),
            vector2: input.vector2.into(),
        }
    }
}

/// Calculate dot product of two 3D vectors
#[cfg_attr(not(test), tool)]
fn dot_product(input: DotProductInput) -> ToolResponse {
    match dot_product_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(error) => ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": error
        })).unwrap()),
    }
}