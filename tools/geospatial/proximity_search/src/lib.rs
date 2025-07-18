use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(not(test))]
use ftl_sdk::ToolResponse;

mod logic;
use logic::{Point as LogicPoint, NearestPointsInput as LogicInput, find_nearest_points};

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
        LogicPoint { lat: p.lat, lon: p.lon, id: p.id }
    }
}

#[derive(Deserialize, JsonSchema)]
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

#[derive(Serialize, JsonSchema)]
struct NearestPointResult {
    /// The found point
    point: Point,
    /// Distance in meters
    distance_meters: f64,
    /// Bearing in degrees
    bearing_degrees: f64,
}

#[derive(Serialize, JsonSchema)]
struct NearestPointsResult {
    /// Original query point
    query_point: Point,
    /// Found nearest points
    nearest_points: Vec<NearestPointResult>,
    /// Total candidates examined
    total_candidates: usize,
    /// Number of results returned
    results_returned: usize,
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
#[cfg_attr(not(test), ftl_sdk::tool)]
fn proximity_search(input: NearestPointsInput) -> ToolResponse {
    let logic_input = LogicInput::from(input);
    
    match find_nearest_points(logic_input.query_point, logic_input.candidate_points, logic_input.max_results, logic_input.max_distance_meters) {
        Ok(result) => {
            let response = NearestPointsResult {
                query_point: Point {
                    lat: result.query_point.lat,
                    lon: result.query_point.lon,
                    id: result.query_point.id,
                },
                nearest_points: result.nearest_points.into_iter().map(|np| NearestPointResult {
                    point: Point {
                        lat: np.point.lat,
                        lon: np.point.lon,
                        id: np.point.id,
                    },
                    distance_meters: np.distance_meters,
                    bearing_degrees: np.bearing_degrees,
                }).collect(),
                total_candidates: result.total_candidates,
                results_returned: result.results_returned,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap_or_else(|_| "Error serializing result".to_string()))
        },
        Err(error) => ToolResponse::text(error),
    }
}

