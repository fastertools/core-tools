#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{Point as LogicPoint, NearestPointsInput as LogicInput, find_nearest_points};

#[derive(serde::Deserialize, JsonSchema)]
struct Point {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
    /// Optional identifier for the point
    id: Option<String>,
}

impl From<Point> for LogicPoint {
    fn from(p: Point) -> Self {
        LogicPoint { lat: p.lat, lon: p.lon, id: p.id }
    }
}

#[derive(serde::Deserialize, JsonSchema)]
struct NearestPointsInput {
    /// Point to search from
    query_point: Point,
    /// Points to search among
    candidate_points: Vec<Point>,
    /// Maximum number of results to return
    max_results: Option<usize>,
    /// Only return points within this distance (meters)
    max_distance_meters: Option<f64>,
}

impl From<NearestPointsInput> for LogicInput {
    fn from(input: NearestPointsInput) -> Self {
        LogicInput {
            query_point: input.query_point.into(),
            candidate_points: input.candidate_points.into_iter().map(|p| p.into()).collect(),
            max_results: input.max_results,
            max_distance_meters: input.max_distance_meters,
        }
    }
}

/// Find nearest points to a query location with distance and bearing
#[cfg_attr(not(test), tool)]
fn proximity_search(input: NearestPointsInput) -> ToolResponse {
    match find_nearest_points(input.query_point.into(), input.candidate_points.into_iter().map(|p| p.into()).collect(), input.max_results, input.max_distance_meters) {
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