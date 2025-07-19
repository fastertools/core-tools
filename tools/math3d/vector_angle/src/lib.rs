use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{
    TwoVectorInput as LogicInput, Vector3D as LogicVector3D, VectorAngleResult, vector_angle_logic,
};

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct TwoVectorInput {
    pub vector1: Vector3D,
    pub vector2: Vector3D,
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

impl From<TwoVectorInput> for LogicInput {
    fn from(input: TwoVectorInput) -> Self {
        LogicInput {
            vector1: input.vector1.into(),
            vector2: input.vector2.into(),
        }
    }
}

#[cfg_attr(not(test), tool)]
pub fn vector_angle(input: TwoVectorInput) -> ToolResponse {
    match vector_angle_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
