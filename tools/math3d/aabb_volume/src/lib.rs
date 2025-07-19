#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct BoundingBoxInput {
    pub points: Vec<Vector3D>,
}

#[derive(Serialize, JsonSchema)]
pub struct BoundingBoxResponse {
    pub volume: f64,
    pub box_type: String,
    pub min_point: Vector3D,
    pub max_point: Vector3D,
    pub dimensions: Vector3D,
}

#[cfg_attr(not(test), tool)]
pub fn aabb_volume(input: BoundingBoxInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::BoundingBoxInput {
        points: input
            .points
            .into_iter()
            .map(|p| logic::Vector3D {
                x: p.x,
                y: p.y,
                z: p.z,
            })
            .collect(),
    };

    // Call business logic
    match logic::compute_aabb_volume(logic_input) {
        Ok(logic_result) => {
            let result = BoundingBoxResponse {
                volume: logic_result.volume,
                box_type: logic_result.box_type,
                min_point: Vector3D {
                    x: logic_result.min_point.x,
                    y: logic_result.min_point.y,
                    z: logic_result.min_point.z,
                },
                max_point: Vector3D {
                    x: logic_result.max_point.x,
                    y: logic_result.max_point.y,
                    z: logic_result.max_point.z,
                },
                dimensions: Vector3D {
                    x: logic_result.dimensions.x,
                    y: logic_result.dimensions.y,
                    z: logic_result.dimensions.z,
                },
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
