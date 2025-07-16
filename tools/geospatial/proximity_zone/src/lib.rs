#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{Point as LogicPoint, ProximityZoneInput as LogicInput, proximity_zone_analysis};

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
struct ProximityZoneInput {
    /// Center of the proximity zone
    center: Point,
    /// Radius of the zone in meters
    radius_meters: f64,
    /// Points to analyze
    candidate_points: Vec<Point>,
}

impl From<ProximityZoneInput> for LogicInput {
    fn from(input: ProximityZoneInput) -> Self {
        LogicInput {
            center: input.center.into(),
            radius_meters: input.radius_meters,
            candidate_points: input.candidate_points.into_iter().map(|p| p.into()).collect(),
        }
    }
}

/// Analyze points within a proximity zone and provide detailed statistics
#[cfg_attr(not(test), tool)]
fn proximity_zone(input: ProximityZoneInput) -> ToolResponse {
    match proximity_zone_analysis(input.center.into(), input.radius_meters, input.candidate_points.into_iter().map(|p| p.into()).collect()) {
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