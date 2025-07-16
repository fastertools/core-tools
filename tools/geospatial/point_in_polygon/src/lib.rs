#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{Point as LogicPoint, PointInPolygonInput as LogicInput, point_in_polygon_check};

#[derive(serde::Deserialize, JsonSchema)]
struct Point {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

impl From<Point> for LogicPoint {
    fn from(p: Point) -> Self {
        LogicPoint { lat: p.lat, lon: p.lon }
    }
}

#[derive(serde::Deserialize, JsonSchema)]
struct PointInPolygonInput {
    /// Point to test
    point: Point,
    /// Polygon vertices
    polygon: Vec<Point>,
}

impl From<PointInPolygonInput> for LogicInput {
    fn from(input: PointInPolygonInput) -> Self {
        LogicInput {
            point: input.point.into(),
            polygon: input.polygon.into_iter().map(|p| p.into()).collect(),
        }
    }
}

/// Check if a point is inside a polygon using ray casting algorithm
#[cfg_attr(not(test), tool)]
fn point_in_polygon(input: PointInPolygonInput) -> ToolResponse {
    match point_in_polygon_check(input.point.into(), input.polygon.into_iter().map(|p| p.into()).collect()) {
        Ok(result) => {
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        }
        Err(e) => {
            ToolResponse::text(format!("Error: {}", e))
        }
    }
}

#[cfg(test)]
pub struct ToolResponse;

#[cfg(test)]
impl ToolResponse {
    pub fn text(_: String) -> Self { ToolResponse }
}