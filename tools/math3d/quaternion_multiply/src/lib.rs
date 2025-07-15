use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, JsonSchema)]
struct QuaternionMultiplyInput {
    q1: Quaternion,
    q2: Quaternion,
}

#[derive(Serialize)]
struct QuaternionMultiplyResponse {
    result: Quaternion,
}

impl Quaternion {
    pub fn multiply(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

#[tool]
fn quaternion_multiply(input: QuaternionMultiplyInput) -> ToolResponse {
    let result = input.q1.multiply(&input.q2);
    let response = QuaternionMultiplyResponse { result };
    
    match serde_json::to_string(&response) {
        Ok(json) => ToolResponse::text(json),
        Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
    }
}
