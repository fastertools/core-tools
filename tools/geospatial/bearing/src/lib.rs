use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;
use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{BearingInput as LogicInput, BearingResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BearingInput {
    /// Latitude of the starting point
    pub lat1: f64,
    /// Longitude of the starting point
    pub lon1: f64,
    /// Latitude of the destination point
    pub lat2: f64,
    /// Longitude of the destination point
    pub lon2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct BearingResult {
    pub bearing_degrees: f64,
    pub bearing_radians: f64,
    pub compass_direction: String,
}

#[cfg_attr(not(test), tool)]
pub fn bearing(input: BearingInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        lat1: input.lat1,
        lon1: input.lon1,
        lat2: input.lat2,
        lon2: input.lon2,
    };

    // Call logic implementation
    match logic::calculate_bearing_between_points(logic_input) {
        Ok(result) => {
            let response = BearingResult {
                bearing_degrees: result.bearing_degrees,
                bearing_radians: result.bearing_radians,
                compass_direction: result.compass_direction,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}
