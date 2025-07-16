#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{CircularBufferInput as LogicInput, Point as LogicPoint, create_circular_buffer};

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
struct CircularBufferInput {
    /// Center point for the buffer
    center: Point,
    /// Buffer radius in meters
    radius_meters: f64,
    /// Number of points to approximate circle (8-360, default 32)
    num_points: Option<usize>,
}

impl From<CircularBufferInput> for LogicInput {
    fn from(input: CircularBufferInput) -> Self {
        LogicInput {
            center: input.center.into(),
            radius_meters: input.radius_meters,
            num_points: input.num_points,
        }
    }
}

/// Create circular buffer around a point using geodesic calculations
#[cfg_attr(not(test), tool)]
fn buffer_polygon(input: CircularBufferInput) -> ToolResponse {
    match create_circular_buffer(input.center.into(), input.radius_meters, input.num_points) {
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