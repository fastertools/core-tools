use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{Point as LogicPoint, PointInPolygonInput as LogicInput, point_in_polygon_check};

#[derive(Deserialize, JsonSchema)]
struct Point {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

impl From<Point> for LogicPoint {
    fn from(p: Point) -> Self {
        LogicPoint {
            lat: p.lat,
            lon: p.lon,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
struct PointInPolygonInput {
    /// Point to test
    point: Point,
    /// Polygon vertices
    polygon: Vec<Point>,
}

#[derive(Serialize, JsonSchema)]
struct PointInPolygonResult {
    /// Whether the point is inside the polygon
    is_inside: bool,
    /// Algorithm used for calculation
    algorithm_used: String,
    /// Whether the point is on the boundary
    on_boundary: bool,
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
#[cfg_attr(not(test), ftl_sdk::tool)]
fn point_in_polygon(input: PointInPolygonInput) -> ToolResponse {
    let logic_input = LogicInput::from(input);

    let result = match point_in_polygon_check(logic_input.point, logic_input.polygon) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error checking point in polygon: {}", e)),
    };

    let output = PointInPolygonResult {
        is_inside: result.is_inside,
        algorithm_used: result.algorithm_used,
        on_boundary: result.on_boundary,
    };

    ToolResponse::text(
        serde_json::to_string(&output).unwrap_or_else(|_| "Error serializing result".to_string()),
    )
}
