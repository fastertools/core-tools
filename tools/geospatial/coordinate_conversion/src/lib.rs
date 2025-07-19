use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{DecimalDegreesInput as LogicInput, convert_to_dms};

#[derive(Deserialize, JsonSchema)]
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

#[derive(Serialize, JsonSchema, Debug)]
struct DMSCoordinate {
    degrees: i32,
    minutes: i32,
    seconds: f64,
    direction: String,
}

#[derive(Serialize, JsonSchema)]
struct CoordinateConversionResult {
    /// Latitude in degrees, minutes, seconds format
    latitude: DMSCoordinate,
    /// Longitude in degrees, minutes, seconds format
    longitude: DMSCoordinate,
}

/// Convert decimal degrees to degrees, minutes, seconds (DMS) format
#[cfg_attr(not(test), tool)]
pub fn coordinate_conversion(input: DecimalDegreesInput) -> ToolResponse {
    let logic_input = LogicInput::from(input);

    match convert_to_dms(logic_input.latitude, logic_input.longitude) {
        Ok(result) => {
            let response = CoordinateConversionResult {
                latitude: DMSCoordinate {
                    degrees: result.latitude.degrees,
                    minutes: result.latitude.minutes,
                    seconds: result.latitude.seconds,
                    direction: result.latitude.direction,
                },
                longitude: DMSCoordinate {
                    degrees: result.longitude.degrees,
                    minutes: result.longitude.minutes,
                    seconds: result.longitude.seconds,
                    direction: result.longitude.direction,
                },
            };
            ftl_sdk::ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ftl_sdk::ToolResponse::text(format!("Error: {e}")),
    }
}
