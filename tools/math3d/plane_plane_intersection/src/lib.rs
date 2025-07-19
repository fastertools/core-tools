use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;

mod logic;
use logic::{PlanePlaneIntersectionInput, plane_plane_intersection_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    /// First plane
    plane1: logic::Plane3D,
    /// Second plane
    plane2: logic::Plane3D,
}

#[derive(serde::Serialize, JsonSchema)]
struct ToolOutput {
    /// Type of intersection: "intersecting", "parallel", or "coincident"
    intersection_type: String,
    /// Whether the planes intersect
    intersects: bool,
    /// The intersection line if planes intersect
    intersection_line: Option<logic::Line3D>,
    /// Whether the planes are parallel
    are_parallel: bool,
    /// Whether the planes are coincident (same plane)
    are_coincident: bool,
    /// Angle between planes in radians
    angle_radians: f64,
    /// Angle between planes in degrees
    angle_degrees: f64,
}

/// Calculate the intersection between two 3D planes
/// Returns detailed information about the intersection including the line of intersection if it exists
#[cfg_attr(not(test), tool)]
pub fn plane_plane_intersection(input: ToolInput) -> ToolResponse {
    let logic_input = PlanePlaneIntersectionInput {
        plane1: input.plane1,
        plane2: input.plane2,
    };

    match plane_plane_intersection_logic(logic_input) {
        Ok(output) => {
            let result = ToolOutput {
                intersection_type: output.intersection_type,
                intersects: output.intersects,
                intersection_line: output.intersection_line,
                are_parallel: output.are_parallel,
                are_coincident: output.are_coincident,
                angle_radians: output.angle_radians,
                angle_degrees: output.angle_degrees,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
