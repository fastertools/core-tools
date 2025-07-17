use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Re-export types from logic module
pub use logic::{TwoPointInput as LogicInput, DistanceResult as LogicOutput, Point2D as LogicPoint2D};

// Define wrapper types with JsonSchema for FTL-SDK
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

// Helper structs for calling pythagorean tool
#[derive(Serialize)]
struct PythagoreanInput {
    a: f64,
    b: f64,
}

#[derive(Deserialize)]
struct PythagoreanResult {
    hypotenuse: f64,
    // Only parse the field we need to avoid deserialization issues
}

#[derive(Deserialize)]
struct OkResponse<T> {
    #[serde(rename = "Ok")]
    ok: T,
}

/// Calculate the distance between two 2D points using the Pythagorean theorem
/// This demonstrates tool composition by calling the pythagorean tool via Spin's local chaining pattern
#[cfg_attr(not(test), tool)]
pub async fn distance_2d(input: TwoPointInput) -> Result<DistanceResult, String> {
    use spin_sdk::http::{Method, Request};
    
    // Step 1: Calculate differences
    let delta_x = input.x2 - input.x1;
    let delta_y = input.y2 - input.y1;
    
    // Step 2: Call pythagorean tool via HTTP
    let pyth_input = PythagoreanInput { a: delta_x, b: delta_y };
    let request_body = serde_json::to_string(&pyth_input)
        .map_err(|e| format!("Failed to serialize pythagorean input: {}. Input: a={}, b={}", e, delta_x, delta_y))?;
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://pythagorean.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
        .map_err(|e| format!("Error calling pythagorean tool: {:?}", e))?;
    
    let body_bytes = response.into_body();
    let body = String::from_utf8(body_bytes)
        .map_err(|e| format!("Failed to parse response body: {}", e))?;
    
    // First, let's try to parse the direct response without Ok wrapper
    let pyth_result: PythagoreanResult = if let Ok(ok_response) = serde_json::from_str::<OkResponse<PythagoreanResult>>(&body) {
        ok_response.ok
    } else {
        // If that fails, try parsing the body directly
        serde_json::from_str(&body)
            .map_err(|e| format!("Failed to parse pythagorean result both ways. Error: {}. Response body: {}", e, body))?
    };
    
    let distance = pyth_result.hypotenuse;
    
    Ok(DistanceResult {
        distance,
        point1: Point2D { x: input.x1, y: input.y1 },
        point2: Point2D { x: input.x2, y: input.y2 },
        delta_x,
        delta_y,
    })
}