use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{DotProductInput as LogicInput, Vector3D as LogicVector3D, dot_product_logic};

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

#[derive(Serialize, JsonSchema)]
struct DotProductResult {
    /// The calculated dot product value
    pub dot_product: f64,
    /// Angle between vectors in radians
    pub angle_radians: f64,
    /// Angle between vectors in degrees
    pub angle_degrees: f64,
    /// Whether the vectors are perpendicular (dot product ≈ 0)
    pub are_perpendicular: bool,
    /// Whether the vectors are parallel (angle ≈ 0° or 180°)
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
pub fn dot_product(input: DotProductInput) -> ToolResponse {
    match dot_product_logic(input.into()) {
        Ok(logic_result) => {
            let result = DotProductResult {
                dot_product: logic_result.dot_product,
                angle_radians: logic_result.angle_radians,
                angle_degrees: logic_result.angle_degrees,
                are_perpendicular: logic_result.are_perpendicular,
                are_parallel: logic_result.are_parallel,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
