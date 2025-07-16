use ftl_sdk::tool;
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
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AABBRayInput {
    pub aabb: AABB,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AABBIntersectionResult {
    pub intersects: bool,
    pub closest_distance: Option<f64>,
    pub intersection_points: Vec<IntersectionPoint>,
}

#[cfg_attr(not(test), tool)]
pub fn ray_aabb_intersection(input: AABBRayInput) -> Result<AABBIntersectionResult, String> {
    // Convert JsonSchema types to logic types
    let logic_input = logic::AABBRayInput {
        aabb: logic::AABB {
            min: logic::Vector3 {
                x: input.aabb.min.x,
                y: input.aabb.min.y,
                z: input.aabb.min.z,
            },
            max: logic::Vector3 {
                x: input.aabb.max.x,
                y: input.aabb.max.y,
                z: input.aabb.max.z,
            },
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
    let logic_result = ray_aabb_intersection_logic(logic_input)?;

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

    Ok(AABBIntersectionResult {
        intersects: logic_result.intersects,
        closest_distance: logic_result.closest_distance,
        intersection_points,
    })
}