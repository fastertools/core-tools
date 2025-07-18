use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};

use logic::{line_intersection_logic, LineIntersectionInput as LogicInput, LineIntersectionResult, Line3D as LogicLine3D, Vector3D as LogicVector3D};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    /// X component of the vector
    pub x: f64,
    /// Y component of the vector
    pub y: f64,
    /// Z component of the vector
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Line3D {
    /// A point on the line
    pub point: Vector3D,
    /// Direction vector of the line
    pub direction: Vector3D,
}

#[derive(Deserialize, JsonSchema)]
pub struct LineIntersectionInput {
    /// First 3D line
    pub line1: Line3D,
    /// Second 3D line
    pub line2: Line3D,
}

impl From<Vector3D> for LogicVector3D {
    fn from(v: Vector3D) -> Self {
        LogicVector3D { x: v.x, y: v.y, z: v.z }
    }
}

impl From<Line3D> for LogicLine3D {
    fn from(line: Line3D) -> Self {
        LogicLine3D {
            point: line.point.into(),
            direction: line.direction.into(),
        }
    }
}

impl From<LineIntersectionInput> for LogicInput {
    fn from(input: LineIntersectionInput) -> Self {
        LogicInput {
            line1: input.line1.into(),
            line2: input.line2.into(),
        }
    }
}

/// Find intersection of two 3D lines
#[cfg_attr(not(test), tool)]
pub fn line_intersection(input: LineIntersectionInput) -> ToolResponse {
    match line_intersection_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}