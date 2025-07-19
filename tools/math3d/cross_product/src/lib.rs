use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{
    CrossProductInput as LogicInput, CrossProductResult as LogicResult, Vector3D as LogicVector3D,
    cross_product_logic,
};

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

#[derive(Serialize, JsonSchema)]
struct CrossProductResult {
    /// The resulting cross product vector
    pub cross_product: Vector3D,
    /// Magnitude of the cross product vector
    pub magnitude: f64,
    /// Area of the parallelogram formed by the two vectors
    pub area_parallelogram: f64,
    /// Whether the vectors are parallel (cross product ≈ zero vector)
    pub are_parallel: bool,
}

impl From<Vector3D> for LogicVector3D {
    fn from(v: Vector3D) -> Self {
        LogicVector3D {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<CrossProductInput> for LogicInput {
    fn from(input: CrossProductInput) -> Self {
        LogicInput {
            vector1: input.vector1.into(),
            vector2: input.vector2.into(),
        }
    }
}

/// Calculate cross product of two 3D vectors
#[cfg_attr(not(test), tool)]
fn cross_product(input: CrossProductInput) -> ToolResponse {
    match cross_product_logic(input.into()) {
        Ok(logic_result) => {
            let result = CrossProductResult {
                cross_product: Vector3D {
                    x: logic_result.cross_product.x,
                    y: logic_result.cross_product.y,
                    z: logic_result.cross_product.z,
                },
                magnitude: logic_result.magnitude,
                area_parallelogram: logic_result.area_parallelogram,
                are_parallel: logic_result.are_parallel,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
