use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::*;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereSphereInput {
    pub sphere1: Sphere,
    pub sphere2: Sphere,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereSphereResult {
    pub intersects: bool,
    pub intersection_type: String,
    pub distance_between_centers: f64,
    pub intersection_circle: Option<IntersectionCircle>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionCircle {
    pub center: Vector3,
    pub radius: f64,
    pub normal: Vector3,
}

#[cfg_attr(not(test), tool)]
pub fn sphere_sphere_intersection(input: SphereSphereInput) -> ToolResponse {
    // Convert JsonSchema types to logic types
    let logic_input = logic::SphereSphereInput {
        sphere1: logic::Sphere {
            center: logic::Vector3 {
                x: input.sphere1.center.x,
                y: input.sphere1.center.y,
                z: input.sphere1.center.z,
            },
            radius: input.sphere1.radius,
        },
        sphere2: logic::Sphere {
            center: logic::Vector3 {
                x: input.sphere2.center.x,
                y: input.sphere2.center.y,
                z: input.sphere2.center.z,
            },
            radius: input.sphere2.radius,
        },
    };

    // Call business logic
    match sphere_sphere_intersection_logic(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to JsonSchema types
            let intersection_circle =
                logic_result
                    .intersection_circle
                    .map(|circle| IntersectionCircle {
                        center: Vector3 {
                            x: circle.center.x,
                            y: circle.center.y,
                            z: circle.center.z,
                        },
                        radius: circle.radius,
                        normal: Vector3 {
                            x: circle.normal.x,
                            y: circle.normal.y,
                            z: circle.normal.z,
                        },
                    });

            let result = SphereSphereResult {
                intersects: logic_result.intersects,
                intersection_type: logic_result.intersection_type,
                distance_between_centers: logic_result.distance_between_centers,
                intersection_circle,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
