use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{Point as LogicPoint, ProximityZoneInput as LogicInput, proximity_zone_analysis};

#[derive(Deserialize, Serialize, JsonSchema)]
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
        LogicPoint {
            lat: p.lat,
            lon: p.lon,
            id: p.id,
        }
    }
}

#[derive(Deserialize, JsonSchema)]
struct ProximityZoneInput {
    /// Center of the proximity zone
    center: Point,
    /// Radius of the zone in meters
    radius_meters: f64,
    /// Points to analyze
    candidate_points: Vec<Point>,
}

#[derive(Serialize, JsonSchema)]
struct NearestPointResult {
    /// The point
    point: Point,
    /// Distance in meters
    distance_meters: f64,
    /// Bearing in degrees
    bearing_degrees: f64,
}

#[derive(Serialize, JsonSchema)]
struct ProximityZoneSummary {
    /// Total number of points
    total_points: usize,
    /// Points inside the zone
    points_inside: usize,
    /// Points outside the zone
    points_outside: usize,
    /// Average distance of points inside
    average_distance_inside: f64,
    /// Distance to closest point
    closest_point_distance: f64,
    /// Distance to farthest point
    farthest_point_distance: f64,
}

#[derive(Serialize, JsonSchema)]
struct ProximityZoneResult {
    /// Center of the zone
    center: Point,
    /// Radius in meters
    radius_meters: f64,
    /// Points within the zone
    points_in_zone: Vec<NearestPointResult>,
    /// Points outside the zone
    points_outside_zone: Vec<NearestPointResult>,
    /// Summary statistics
    summary: ProximityZoneSummary,
}

impl From<ProximityZoneInput> for LogicInput {
    fn from(input: ProximityZoneInput) -> Self {
        LogicInput {
            center: input.center.into(),
            radius_meters: input.radius_meters,
            candidate_points: input
                .candidate_points
                .into_iter()
                .map(|p| p.into())
                .collect(),
        }
    }
}

/// Analyze points within a proximity zone and provide detailed statistics
#[cfg_attr(not(test), tool)]
pub fn proximity_zone(input: ProximityZoneInput) -> ToolResponse {
    let logic_input = LogicInput::from(input);

    match proximity_zone_analysis(
        logic_input.center,
        logic_input.radius_meters,
        logic_input.candidate_points,
    ) {
        Ok(result) => {
            let response = ProximityZoneResult {
                center: Point {
                    lat: result.center.lat,
                    lon: result.center.lon,
                    id: result.center.id,
                },
                radius_meters: result.radius_meters,
                points_in_zone: result
                    .points_in_zone
                    .into_iter()
                    .map(|np| NearestPointResult {
                        point: Point {
                            lat: np.point.lat,
                            lon: np.point.lon,
                            id: np.point.id,
                        },
                        distance_meters: np.distance_meters,
                        bearing_degrees: np.bearing_degrees,
                    })
                    .collect(),
                points_outside_zone: result
                    .points_outside_zone
                    .into_iter()
                    .map(|np| NearestPointResult {
                        point: Point {
                            lat: np.point.lat,
                            lon: np.point.lon,
                            id: np.point.id,
                        },
                        distance_meters: np.distance_meters,
                        bearing_degrees: np.bearing_degrees,
                    })
                    .collect(),
                summary: ProximityZoneSummary {
                    total_points: result.summary.total_points,
                    points_inside: result.summary.points_inside,
                    points_outside: result.summary.points_outside,
                    average_distance_inside: result.summary.average_distance_inside,
                    closest_point_distance: result.summary.closest_point_distance,
                    farthest_point_distance: result.summary.farthest_point_distance,
                },
            };
            ToolResponse::text(
                serde_json::to_string(&response)
                    .unwrap_or_else(|_| "Error serializing result".to_string()),
            )
        }
        Err(error) => ToolResponse::text(error),
    }
}
