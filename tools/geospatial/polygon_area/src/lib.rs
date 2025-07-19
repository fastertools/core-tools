use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{Coordinate as LogicCoordinate, PolygonInput as LogicInput, get_polygon_area};

#[derive(Deserialize, JsonSchema)]
struct Coordinate {
    /// Latitude in decimal degrees
    lat: f64,
    /// Longitude in decimal degrees
    lon: f64,
}

impl From<Coordinate> for LogicCoordinate {
    fn from(c: Coordinate) -> Self {
        LogicCoordinate {
            lat: c.lat,
            lon: c.lon,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
struct PolygonInput {
    /// Array of coordinates defining the polygon
    coordinates: Vec<Coordinate>,
}

#[derive(Serialize, JsonSchema)]
struct PolygonAreaResult {
    /// Area in square meters
    area_square_meters: f64,
    /// Area in square kilometers
    area_square_kilometers: f64,
    /// Area in square miles
    area_square_miles: f64,
    /// Area in hectares
    area_hectares: f64,
    /// Area in acres
    area_acres: f64,
}

impl From<PolygonInput> for LogicInput {
    fn from(input: PolygonInput) -> Self {
        LogicInput {
            coordinates: input.coordinates.into_iter().map(|c| c.into()).collect(),
        }
    }
}

/// Calculate area of a GPS polygon
#[cfg_attr(not(test), ftl_sdk::tool)]
fn polygon_area(input: PolygonInput) -> ToolResponse {
    let logic_input = LogicInput::from(input);

    let result = match get_polygon_area(logic_input.coordinates) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error calculating polygon area: {e}")),
    };

    let output = PolygonAreaResult {
        area_square_meters: result.area_square_meters,
        area_square_kilometers: result.area_square_kilometers,
        area_square_miles: result.area_square_miles,
        area_hectares: result.area_hectares,
        area_acres: result.area_acres,
    };

    ToolResponse::text(
        serde_json::to_string(&output).unwrap_or_else(|_| "Error serializing result".to_string()),
    )
}
