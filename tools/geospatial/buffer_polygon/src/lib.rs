use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{CircularBufferInput as LogicInput, Point as LogicPoint, create_circular_buffer};

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug)]
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

#[derive(Deserialize, JsonSchema)]
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

#[derive(Serialize, JsonSchema)]
struct BufferPolygonResult {
    /// The resulting buffer polygon as a sequence of points
    buffer_polygon: Vec<Point>,
    /// Area of the buffer polygon in square meters
    area_square_meters: f64,
    /// Perimeter of the buffer polygon in meters
    perimeter_meters: f64,
    /// Algorithm used for buffer calculation
    algorithm_used: String,
}

/// Create circular buffer around a point using geodesic calculations
#[cfg_attr(not(test), ftl_sdk::tool)]
fn buffer_polygon(input: CircularBufferInput) -> ftl_sdk::ToolResponse {
    let logic_input = LogicInput::from(input);
    
    match create_circular_buffer(logic_input.center, logic_input.radius_meters, logic_input.num_points) {
        Ok(result) => {
            let response = BufferPolygonResult {
                buffer_polygon: result.buffer_polygon.into_iter().map(|p| Point { lat: p.lat, lon: p.lon }).collect(),
                area_square_meters: result.area_square_meters,
                perimeter_meters: result.perimeter_meters,
                algorithm_used: result.algorithm_used,
            };
            ftl_sdk::ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ftl_sdk::ToolResponse::text(format!("Error: {}", e))
    }
}

