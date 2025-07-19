use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{
    Line3D as LogicLine3D, MultipleLineIntersectionResult, MultipleLinesInput as LogicInput,
    Vector3D as LogicVector3D, multiple_line_intersection_logic,
};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Line3D {
    pub point: Vector3D,     // A point on the line
    pub direction: Vector3D, // Direction vector of the line
}

#[derive(Deserialize, JsonSchema)]
pub struct MultipleLinesInput {
    pub lines: Vec<Line3D>,
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

impl From<Line3D> for LogicLine3D {
    fn from(line: Line3D) -> Self {
        LogicLine3D {
            point: line.point.into(),
            direction: line.direction.into(),
        }
    }
}

impl From<MultipleLinesInput> for LogicInput {
    fn from(input: MultipleLinesInput) -> Self {
        LogicInput {
            lines: input.lines.into_iter().map(|line| line.into()).collect(),
        }
    }
}

#[cfg_attr(not(test), tool)]
pub fn multiple_line_intersection(input: MultipleLinesInput) -> ToolResponse {
    match multiple_line_intersection_logic(input.into()) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
