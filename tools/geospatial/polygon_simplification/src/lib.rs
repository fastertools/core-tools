use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{
    Point as LogicPoint, PolygonSimplificationInput as LogicInput, polygon_simplification_logic,
};

#[derive(serde::Deserialize, JsonSchema)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

impl From<Point> for LogicPoint {
    fn from(p: Point) -> Self {
        LogicPoint {
            lat: p.lat,
            lon: p.lon,
        }
    }
}

#[derive(serde::Deserialize, JsonSchema)]
pub struct PolygonSimplificationInput {
    pub polygon: Vec<Point>,
    pub tolerance_meters: f64,
    pub algorithm: Option<String>, // "douglas_peucker" or "visvalingam" (default: douglas_peucker)
}

impl From<PolygonSimplificationInput> for LogicInput {
    fn from(input: PolygonSimplificationInput) -> Self {
        LogicInput {
            polygon: input.polygon.into_iter().map(|p| p.into()).collect(),
            tolerance_meters: input.tolerance_meters,
            algorithm: input.algorithm,
        }
    }
}

#[cfg_attr(not(test), ftl_sdk::tool)]
pub fn polygon_simplification(input: PolygonSimplificationInput) -> ToolResponse {
    match polygon_simplification_logic(input.into()) {
        Ok(result) => ToolResponse::text(
            serde_json::to_string(&result)
                .unwrap_or_else(|_| "Error serializing result".to_string()),
        ),
        Err(error) => ToolResponse::text(error),
    }
}
