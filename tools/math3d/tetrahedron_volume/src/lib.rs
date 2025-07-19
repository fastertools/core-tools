use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct TetrahedronVolumeInput {
    pub point_a: Vector3D,
    pub point_b: Vector3D,
    pub point_c: Vector3D,
    pub point_d: Vector3D,
}

#[derive(Serialize, JsonSchema)]
pub struct TetrahedronVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub points: [Vector3D; 4],
}

#[cfg_attr(not(test), tool)]
pub fn tetrahedron_volume(input: TetrahedronVolumeInput) -> ToolResponse {
    // Convert API types to logic types
    let logic_input = logic::TetrahedronVolumeInput {
        point_a: logic::Vector3D {
            x: input.point_a.x,
            y: input.point_a.y,
            z: input.point_a.z,
        },
        point_b: logic::Vector3D {
            x: input.point_b.x,
            y: input.point_b.y,
            z: input.point_b.z,
        },
        point_c: logic::Vector3D {
            x: input.point_c.x,
            y: input.point_c.y,
            z: input.point_c.z,
        },
        point_d: logic::Vector3D {
            x: input.point_d.x,
            y: input.point_d.y,
            z: input.point_d.z,
        },
    };

    // Call business logic
    match logic::compute_tetrahedron_volume(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to API types
            let result = TetrahedronVolumeResponse {
                volume: logic_result.volume,
                calculation_method: logic_result.calculation_method,
                points: [
                    Vector3D {
                        x: logic_result.points[0].x,
                        y: logic_result.points[0].y,
                        z: logic_result.points[0].z,
                    },
                    Vector3D {
                        x: logic_result.points[1].x,
                        y: logic_result.points[1].y,
                        z: logic_result.points[1].z,
                    },
                    Vector3D {
                        x: logic_result.points[2].x,
                        y: logic_result.points[2].y,
                        z: logic_result.points[2].z,
                    },
                    Vector3D {
                        x: logic_result.points[3].x,
                        y: logic_result.points[3].y,
                        z: logic_result.points[3].z,
                    },
                ],
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
