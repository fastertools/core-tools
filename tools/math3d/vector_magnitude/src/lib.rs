use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

// Re-export types from logic module
pub use logic::{
    Vector3D as LogicVector3D, VectorMagnitudeInput as LogicInput,
    VectorMagnitudeOutput as LogicOutput,
};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    /// X component of the vector
    pub x: f64,
    /// Y component of the vector
    pub y: f64,
    /// Z component of the vector
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VectorMagnitudeInput {
    /// 3D vector to calculate magnitude for
    pub vector: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VectorMagnitudeResult {
    pub magnitude: f64,
    pub unit_vector: Vector3D,
    pub is_zero_vector: bool,
}

#[cfg_attr(not(test), tool)]
pub fn vector_magnitude(input: VectorMagnitudeInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        vector: LogicVector3D {
            x: input.vector.x,
            y: input.vector.y,
            z: input.vector.z,
        },
    };

    // Call logic implementation
    match logic::compute_vector_magnitude(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = VectorMagnitudeResult {
                magnitude: result.magnitude,
                unit_vector: Vector3D {
                    x: result.unit_vector.x,
                    y: result.unit_vector.y,
                    z: result.unit_vector.z,
                },
                is_zero_vector: result.is_zero_vector,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
