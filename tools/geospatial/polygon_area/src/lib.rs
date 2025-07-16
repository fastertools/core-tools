#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{Coordinate as LogicCoordinate, PolygonInput as LogicInput, get_polygon_area};

#[derive(serde::Deserialize, JsonSchema)]
struct Coordinate {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

impl From<Coordinate> for LogicCoordinate {
    fn from(c: Coordinate) -> Self {
        LogicCoordinate { lat: c.lat, lon: c.lon }
    }
}

#[derive(serde::Deserialize, JsonSchema)]
struct PolygonInput {
    /// Array of coordinates defining the polygon
    coordinates: Vec<Coordinate>,
}

impl From<PolygonInput> for LogicInput {
    fn from(input: PolygonInput) -> Self {
        LogicInput {
            coordinates: input.coordinates.into_iter().map(|c| c.into()).collect(),
        }
    }
}

/// Calculate area of a GPS polygon
#[cfg_attr(not(test), tool)]
fn polygon_area(input: PolygonInput) -> ToolResponse {
    match get_polygon_area(input.coordinates.into_iter().map(|c| c.into()).collect()) {
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