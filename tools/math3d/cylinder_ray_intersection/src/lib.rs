use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;
use logic::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Cylinder {
    pub center: Vector3,
    pub axis: Vector3,
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CylinderRayInput {
    pub cylinder: Cylinder,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CylinderRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

#[cfg_attr(not(test), tool)]
pub fn cylinder_ray_intersection(input: CylinderRayInput) -> ToolResponse {
    // Convert JsonSchema types to logic types
    let logic_input = logic::CylinderRayInput {
        cylinder: logic::Cylinder {
            center: logic::Vector3 {
                x: input.cylinder.center.x,
                y: input.cylinder.center.y,
                z: input.cylinder.center.z,
            },
            axis: logic::Vector3 {
                x: input.cylinder.axis.x,
                y: input.cylinder.axis.y,
                z: input.cylinder.axis.z,
            },
            radius: input.cylinder.radius,
            height: input.cylinder.height,
        },
        ray: logic::Ray {
            origin: logic::Vector3 {
                x: input.ray.origin.x,
                y: input.ray.origin.y,
                z: input.ray.origin.z,
            },
            direction: logic::Vector3 {
                x: input.ray.direction.x,
                y: input.ray.direction.y,
                z: input.ray.direction.z,
            },
        },
    };

    // Call business logic
    match cylinder_ray_intersection_logic(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to JsonSchema types
            let intersection_points = logic_result.intersection_points
                .into_iter()
                .map(|point| IntersectionPoint {
                    point: Vector3 {
                        x: point.point.x,
                        y: point.point.y,
                        z: point.point.z,
                    },
                    distance: point.distance,
                    normal: Vector3 {
                        x: point.normal.x,
                        y: point.normal.y,
                        z: point.normal.z,
                    },
                })
                .collect();

            let result = CylinderRayResult {
                intersects: logic_result.intersects,
                intersection_points,
                closest_distance: logic_result.closest_distance,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}