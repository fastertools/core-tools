use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;
use serde::Deserialize;

mod logic;
use logic::{point_plane_distance_logic, PointPlaneInput as LogicInput, Vector3D as LogicVector3D, Plane3D as LogicPlane3D};

#[derive(Deserialize, JsonSchema, Clone, Debug)]
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
        LogicVector3D { x: v.x, y: v.y, z: v.z }
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

/// Calculate the distance from a point to a plane in 3D space
/// Returns both signed and unsigned distance, the closest point on the plane, and which side of the plane the point is on
#[cfg_attr(not(test), tool)]
fn point_plane_distance(input: PointPlaneInput) -> ToolResponse {
    match point_plane_distance_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(error) => ToolResponse::text(serde_json::to_string(&serde_json::json!({
            "error": error
        })).unwrap()),
    }
}