use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{
    Plane3D as LogicPlane3D, PointPlaneInput as LogicInput, Vector3D as LogicVector3D,
    point_plane_distance_logic,
};

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Deserialize, JsonSchema, Clone, Debug)]
struct Plane3D {
    /// A point on the plane
    point: Vector3D,
    /// Normal vector to the plane
    normal: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
struct PointPlaneInput {
    /// The point to measure distance from
    point: Vector3D,
    /// The plane to measure distance to
    plane: Plane3D,
}

impl From<Vector3D> for LogicVector3D {
    fn from(v: Vector3D) -> Self {
        LogicVector3D {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl From<Plane3D> for LogicPlane3D {
    fn from(p: Plane3D) -> Self {
        LogicPlane3D {
            point: p.point.into(),
            normal: p.normal.into(),
        }
    }
}

impl From<PointPlaneInput> for LogicInput {
    fn from(input: PointPlaneInput) -> Self {
        LogicInput {
            point: input.point.into(),
            plane: input.plane.into(),
        }
    }
}

#[derive(Serialize, JsonSchema)]
struct PointPlaneResult {
    /// Absolute distance from point to plane
    distance: f64,
    /// Signed distance (positive if point is on the side of normal, negative otherwise)
    signed_distance: f64,
    /// Closest point on the plane to the given point
    closest_point_on_plane: Vector3D,
    /// Whether the point lies exactly on the plane
    is_on_plane: bool,
    /// Which side of the plane the point is on
    side_of_plane: String,
}

/// Calculate the distance from a point to a plane in 3D space
/// Returns both signed and unsigned distance, the closest point on the plane, and which side of the plane the point is on
#[cfg_attr(not(test), tool)]
pub fn point_plane_distance(input: PointPlaneInput) -> ToolResponse {
    match point_plane_distance_logic(input.into()) {
        Ok(logic_result) => {
            let result = PointPlaneResult {
                distance: logic_result.distance,
                signed_distance: logic_result.signed_distance,
                closest_point_on_plane: Vector3D {
                    x: logic_result.closest_point_on_plane.x,
                    y: logic_result.closest_point_on_plane.y,
                    z: logic_result.closest_point_on_plane.z,
                },
                is_on_plane: logic_result.is_on_plane,
                side_of_plane: logic_result.side_of_plane,
            };
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
