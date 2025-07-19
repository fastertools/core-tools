use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{
    LineSegmentInput as LogicInput, Vector3D as LogicVector3D, line_segment_intersection_logic,
};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct LineSegmentInput {
    pub segment1_start: Vector3D,
    pub segment1_end: Vector3D,
    pub segment2_start: Vector3D,
    pub segment2_end: Vector3D,
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

impl From<LineSegmentInput> for LogicInput {
    fn from(input: LineSegmentInput) -> Self {
        LogicInput {
            segment1_start: input.segment1_start.into(),
            segment1_end: input.segment1_end.into(),
            segment2_start: input.segment2_start.into(),
            segment2_end: input.segment2_end.into(),
        }
    }
}

#[cfg_attr(not(test), tool)]
pub fn line_segment_intersection(input: LineSegmentInput) -> ToolResponse {
    match line_segment_intersection_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {e}")),
    }
}
