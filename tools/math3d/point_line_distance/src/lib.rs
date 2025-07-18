use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;
use logic::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Line3D {
    pub point: Vector3D,
    pub direction: Vector3D,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointLineInput {
    pub point: Vector3D,
    pub line: Line3D,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PointLineDistanceResult {
    pub distance: f64,
    pub closest_point_on_line: Vector3D,
    pub parameter_on_line: f64,
    pub perpendicular_vector: Vector3D,
    pub point_is_on_line: bool,
}

#[cfg_attr(not(test), tool)]
pub fn point_line_distance(input: PointLineInput) -> ToolResponse {
    // Convert JsonSchema types to logic types
    let logic_input = logic::PointLineInput {
        point: logic::Vector3D {
            x: input.point.x,
            y: input.point.y,
            z: input.point.z,
        },
        line: logic::Line3D {
            point: logic::Vector3D {
                x: input.line.point.x,
                y: input.line.point.y,
                z: input.line.point.z,
            },
            direction: logic::Vector3D {
                x: input.line.direction.x,
                y: input.line.direction.y,
                z: input.line.direction.z,
            },
        },
    };

    // Call business logic
    match point_line_distance_logic(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to JsonSchema types
            let result = PointLineDistanceResult {
                distance: logic_result.distance,
                closest_point_on_line: Vector3D {
                    x: logic_result.closest_point_on_line.x,
                    y: logic_result.closest_point_on_line.y,
                    z: logic_result.closest_point_on_line.z,
                },
                parameter_on_line: logic_result.parameter_on_line,
                perpendicular_vector: Vector3D {
                    x: logic_result.perpendicular_vector.x,
                    y: logic_result.perpendicular_vector.y,
                    z: logic_result.perpendicular_vector.z,
                },
                point_is_on_line: logic_result.point_is_on_line,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}