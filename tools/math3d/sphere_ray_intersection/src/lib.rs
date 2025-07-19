use ftl_sdk::{ToolResponse, tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Sphere {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereRayInput {
    pub sphere: Sphere,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3D,
    pub distance: f64,
    pub normal: Vector3D,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

#[cfg_attr(not(test), tool)]
pub fn sphere_ray_intersection(input: SphereRayInput) -> ToolResponse {
    // Convert JsonSchema types to logic types
    let logic_input = logic::SphereRayInput {
        sphere: logic::Sphere {
            center: logic::Vector3D {
                x: input.sphere.center.x,
                y: input.sphere.center.y,
                z: input.sphere.center.z,
            },
            radius: input.sphere.radius,
        },
        ray: logic::Ray {
            origin: logic::Vector3D {
                x: input.ray.origin.x,
                y: input.ray.origin.y,
                z: input.ray.origin.z,
            },
            direction: logic::Vector3D {
                x: input.ray.direction.x,
                y: input.ray.direction.y,
                z: input.ray.direction.z,
            },
        },
    };

    // Call business logic
    match sphere_ray_intersection_logic(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to JsonSchema types
            let intersection_points = logic_result
                .intersection_points
                .into_iter()
                .map(|point| IntersectionPoint {
                    point: Vector3D {
                        x: point.point.x,
                        y: point.point.y,
                        z: point.point.z,
                    },
                    distance: point.distance,
                    normal: Vector3D {
                        x: point.normal.x,
                        y: point.normal.y,
                        z: point.normal.z,
                    },
                })
                .collect();

            let result = SphereRayResult {
                intersects: logic_result.intersects,
                intersection_points,
                closest_distance: logic_result.closest_distance,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
