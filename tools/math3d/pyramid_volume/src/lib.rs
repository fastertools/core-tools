use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
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
pub struct PyramidInput {
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[derive(Serialize, JsonSchema)]
pub struct PyramidResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub base_area: f64,
    pub height: f64,
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[cfg_attr(not(test), tool)]
pub fn pyramid_volume(input: PyramidInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::PyramidInput {
        base_points: input
            .base_points
            .into_iter()
            .map(|p| logic::Vector3D {
                x: p.x,
                y: p.y,
                z: p.z,
            })
            .collect(),
        apex: logic::Vector3D {
            x: input.apex.x,
            y: input.apex.y,
            z: input.apex.z,
        },
    };

    // Call business logic
    match logic::compute_pyramid_volume(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to API types
            let result = PyramidResponse {
                volume: logic_result.volume,
                calculation_method: logic_result.calculation_method,
                base_area: logic_result.base_area,
                height: logic_result.height,
                base_points: logic_result
                    .base_points
                    .into_iter()
                    .map(|p| Vector3D {
                        x: p.x,
                        y: p.y,
                        z: p.z,
                    })
                    .collect(),
                apex: Vector3D {
                    x: logic_result.apex.x,
                    y: logic_result.apex.y,
                    z: logic_result.apex.z,
                },
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
