use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(not(test))]
use ftl_sdk::tool;

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
/// Simplified version for debugging - calculates directly without HTTP calls
#[cfg_attr(not(test), tool)]
pub fn distance_2d(input: TwoPointInput) -> Result<DistanceResult, String> {
    // Step 1: Calculate differences
    let delta_x = input.x2 - input.x1;
    let delta_y = input.y2 - input.y1;
    
    // Step 2: Calculate distance directly: sqrt(delta_x^2 + delta_y^2)
    let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
    
    Ok(DistanceResult {
        distance,
        point1: Point2D { x: input.x1, y: input.y1 },
        point2: Point2D { x: input.x2, y: input.y2 },
        delta_x,
        delta_y,
    })
}