use ftl_sdk::ToolResponse;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{DistanceInput as LogicInput, DistanceResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistanceInput {
    /// Latitude of the first point
    pub lat1: f64,
    /// Longitude of the first point
    pub lon1: f64,
    /// Latitude of the second point
    pub lat2: f64,
    /// Longitude of the second point
    pub lon2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct DistanceResult {
    pub distance_km: f64,
    pub distance_miles: f64,
    pub distance_nautical_miles: f64,
}

#[cfg_attr(not(test), tool)]
pub fn distance(input: DistanceInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        lat1: input.lat1,
        lon1: input.lon1,
        lat2: input.lat2,
        lon2: input.lon2,
    };

    // Call logic implementation
    let result = match logic::calculate_distance_between_points(logic_input) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error calculating distance: {}", e)),
    };

    // Convert back to wrapper types
    let output = DistanceResult {
        distance_km: result.distance_km,
        distance_miles: result.distance_miles,
        distance_nautical_miles: result.distance_nautical_miles,
    };

    ToolResponse::text(
        serde_json::to_string(&output).unwrap_or_else(|_| "Error serializing result".to_string()),
    )
}
