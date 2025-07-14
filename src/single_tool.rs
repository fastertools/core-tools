use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct DistanceInput {
    /// Latitude of the first point
    lat1: f64,
    /// Longitude of the first point
    lon1: f64,
    /// Latitude of the second point
    lat2: f64,
    /// Longitude of the second point
    lon2: f64,
}

/// Calculate distance between two GPS coordinates using Haversine formula
#[tool]
fn distance(input: DistanceInput) -> ToolResponse {
    use crate::geospatial::distance::{CoordinateInput, calculate_distance};
    
    let internal_input = CoordinateInput {
        lat1: input.lat1,
        lon1: input.lon1,
        lat2: input.lat2,
        lon2: input.lon2,
    };
    
    let result = calculate_distance(internal_input);
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}