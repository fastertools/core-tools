use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct BearingInput {
    /// Latitude of the first point
    lat1: f64,
    /// Longitude of the first point
    lon1: f64,
    /// Latitude of the second point
    lat2: f64,
    /// Longitude of the second point
    lon2: f64,
}

/// Calculate bearing from first point to second point in degrees
#[tool]
fn bearing(input: BearingInput) -> ToolResponse {
    use crate::geospatial::bearing::{BearingInput as InternalInput, get_bearing};
    
    let internal_input = InternalInput {
        lat1: input.lat1,
        lon1: input.lon1,
        lat2: input.lat2,
        lon2: input.lon2,
    };
    
    let result = get_bearing(internal_input);
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}