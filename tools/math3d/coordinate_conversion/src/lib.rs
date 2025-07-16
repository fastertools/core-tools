use ftl_sdk::ToolResponse;
use schemars::JsonSchema;

mod logic;
use logic::{CoordinateConversionInput, coordinate_conversion_logic};

#[derive(serde::Deserialize, JsonSchema)]
struct ToolInput {
    from_type: String,
    to_type: String,
    coordinates: logic::Vector3D,
}

#[derive(serde::Serialize)]
struct ToolResponse_ {
    original: logic::Vector3D,
    converted: logic::Vector3D,
    from_type: String,
    to_type: String,
}

#[cfg_attr(not(test), ftl_sdk::tool)]
fn coordinate_conversion(input: ToolInput) -> ToolResponse {
    let logic_input = CoordinateConversionInput {
        from_type: input.from_type,
        to_type: input.to_type,
        coordinates: input.coordinates,
    };
    
    match coordinate_conversion_logic(logic_input) {
        Ok(output) => {
            let response = ToolResponse_ {
                original: output.original,
                converted: output.converted,
                from_type: output.from_type,
                to_type: output.to_type,
            };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}