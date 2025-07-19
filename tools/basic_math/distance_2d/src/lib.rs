use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
use ftl_sdk::ToolResponse;

mod logic;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Point2D {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoPointInput {
    /// X coordinate of first point
    pub x1: f64,
    /// Y coordinate of first point
    pub y1: f64,
    /// X coordinate of second point
    pub x2: f64,
    /// Y coordinate of second point
    pub y2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistanceResult {
    /// The calculated distance
    pub distance: f64,
    /// First point
    pub point1: Point2D,
    /// Second point
    pub point2: Point2D,
    /// Difference in X coordinates
    pub delta_x: f64,
    /// Difference in Y coordinates
    pub delta_y: f64,
}


/// Calculate the distance between two 2D points using the Pythagorean theorem
#[cfg_attr(not(test), tool)]
pub fn distance_2d(input: TwoPointInput) -> ToolResponse {
    // Convert from flat coordinate input to logic types
    let logic_input = logic::TwoPointInput {
        point1: logic::Point2D { x: input.x1, y: input.y1 },
        point2: logic::Point2D { x: input.x2, y: input.y2 },
    };
    
    // Call logic implementation
    match logic::calculate_distance_2d(logic_input) {
        Ok(result) => {
            // Convert back to wrapper types
            let response = DistanceResult {
                distance: result.distance,
                point1: Point2D { x: result.point1.x, y: result.point1.y },
                point2: Point2D { x: result.point2.x, y: result.point2.y },
                delta_x: result.delta_x,
                delta_y: result.delta_y,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}

