// Core modules - keeping the existing functionality
pub mod common;
pub mod coordinate_utils;
pub mod geofencing;
pub mod geospatial;
pub mod math_3d;
pub mod statistics;

// FTL tools module - contains the tool definitions using ftl-sdk
// pub mod ftl_tools; // Disabled - using individual tool files instead

// Individual tool modules organized by category
// pub mod tools; // Disabled - can only have one tool per WASM module

// Tool modules - only one active at build time
#[cfg(feature = "distance")]
pub mod single_tool;

// Bearing tool implementation
#[cfg(feature = "bearing")]
pub use bearing_impl::*;

#[cfg(feature = "bearing")]
mod bearing_impl {
    use ftl_sdk::{tool, ToolResponse};
    use serde::Deserialize;
    use schemars::JsonSchema;

    #[derive(Deserialize, JsonSchema)]
    struct BearingInput {
        /// Latitude of the first point
        lat1: f64,
        /// Longitude of the first point
        lon1: f64,
        /// Latitude of the second point
        lat2: f64,
        /// Longitude of the second point
        lon2: f64,
    }

    /// Calculate bearing from first point to second point in degrees
    #[tool]
    fn bearing(input: BearingInput) -> ToolResponse {
        use crate::geospatial::bearing::{BearingInput as InternalInput, get_bearing};
        
        let internal_input = InternalInput {
            lat1: input.lat1,
            lon1: input.lon1,
            lat2: input.lat2,
            lon2: input.lon2,
        };
        
        let result = get_bearing(internal_input);
        ToolResponse::text(serde_json::to_string(&result).unwrap())
    }
}

#[cfg(feature = "dot_product")]
pub mod dot_product_tool;