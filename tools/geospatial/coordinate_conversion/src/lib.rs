#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};
use schemars::JsonSchema;

mod logic;
use logic::{DecimalDegreesInput as LogicInput, convert_to_dms};

#[derive(serde::Deserialize, JsonSchema)]
struct DecimalDegreesInput {
    /// Latitude in decimal degrees
    latitude: f64,
    /// Longitude in decimal degrees
    longitude: f64,
}

impl From<DecimalDegreesInput> for LogicInput {
    fn from(input: DecimalDegreesInput) -> Self {
        LogicInput {
            latitude: input.latitude,
            longitude: input.longitude,
        }
    }
}

/// Convert decimal degrees to degrees, minutes, seconds (DMS) format
#[cfg_attr(not(test), tool)]
fn coordinate_conversion(input: DecimalDegreesInput) -> ToolResponse {
    match convert_to_dms(input.latitude, input.longitude) {
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