use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;
use logic::*;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
pub struct Line3D {
    /// A point on the line
    pub point: Vector3D,
    /// Direction vector of the line
    pub direction: Vector3D,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
pub struct Plane3D {
    /// A point on the plane
    pub point: Vector3D,
    /// Normal vector to the plane
    pub normal: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
pub struct LinePlaneInput {
    /// The line to test for intersection
    pub line: Line3D,
    /// The plane to test against
    pub plane: Plane3D,
}

#[derive(Serialize, JsonSchema)]
pub struct LinePlaneIntersectionResult {
    /// Type of intersection: "point", "line_in_plane", or "no_intersection"
    pub intersection_type: String,
    /// Whether the line intersects the plane
    pub intersects: bool,
    /// The intersection point if it exists
    pub intersection_point: Option<Vector3D>,
    /// Parameter t where intersection occurs (line_point = line.point + t * line.direction)
    pub parameter: Option<f64>,
    /// Whether the line is parallel to the plane
    pub line_is_parallel: bool,
    /// Whether the line lies entirely in the plane
    pub line_is_in_plane: bool,
    /// Distance from line to plane (0 if intersecting)
    pub distance_to_plane: f64,
}

/// Calculate the intersection between a 3D line and a plane
/// Returns detailed information about the intersection including type, point, and geometric relationships
#[cfg_attr(not(test), ftl_sdk::tool)]
pub fn line_plane_intersection(input: LinePlaneInput) -> ftl_sdk::ToolResponse {
    // Convert JsonSchema types to logic types
    let logic_input = logic::LinePlaneInput {
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
        plane: logic::Plane3D {
            point: logic::Vector3D {
                x: input.plane.point.x,
                y: input.plane.point.y,
                z: input.plane.point.z,
            },
            normal: logic::Vector3D {
                x: input.plane.normal.x,
                y: input.plane.normal.y,
                z: input.plane.normal.z,
            },
        },
    };

    // Call business logic
    match line_plane_intersection_logic(logic_input) {
        Ok(logic_result) => {
            // Convert logic types back to JsonSchema types
            let result = LinePlaneIntersectionResult {
                intersection_type: logic_result.intersection_type,
                intersects: logic_result.intersects,
                intersection_point: logic_result.intersection_point.map(|point| Vector3D {
                    x: point.x,
                    y: point.y,
                    z: point.z,
                }),
                parameter: logic_result.parameter,
                line_is_parallel: logic_result.line_is_parallel,
                line_is_in_plane: logic_result.line_is_in_plane,
                distance_to_plane: logic_result.distance_to_plane,
            };
            ftl_sdk::ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ftl_sdk::ToolResponse::text(format!("Error: {}", e))
    }
}