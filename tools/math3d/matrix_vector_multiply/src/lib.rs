use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
struct MatrixVectorInput {
    matrix: Matrix3x3,
    vector: Vector3D,
}

#[derive(Serialize)]
struct MatrixVectorResponse {
    result: Vector3D,
}

impl Matrix3x3 {
    pub fn multiply_vector(&self, v: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.m00 * v.x + self.m01 * v.y + self.m02 * v.z,
            y: self.m10 * v.x + self.m11 * v.y + self.m12 * v.z,
            z: self.m20 * v.x + self.m21 * v.y + self.m22 * v.z,
        }
    }
}

#[tool]
fn matrix_vector_multiply(input: MatrixVectorInput) -> ToolResponse {
    let result = input.matrix.multiply_vector(&input.vector);
    let response = MatrixVectorResponse { result };
    
    match serde_json::to_string(&response) {
        Ok(json) => ToolResponse::text(json),
        Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
    }
}