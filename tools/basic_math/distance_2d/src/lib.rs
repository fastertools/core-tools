use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
use ftl_sdk::ToolResponse;

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
struct ToolResponseWrapper {
    content: Vec<ContentItem>,
}

#[derive(Deserialize)]
struct ContentItem {
    #[serde(rename = "type")]
    item_type: String,
    text: String,
}

/// Calculate the distance between two 2D points using the Pythagorean theorem
/// This demonstrates tool composition by calling the pythagorean tool via Spin's local chaining pattern
#[cfg(all(feature = "individual", not(test)))]
#[cfg_attr(not(test), tool)]
pub async fn distance_2d(input: TwoPointInput) -> ToolResponse {
    use spin_sdk::http::{Method, Request};
    
    // Step 1: Calculate differences
    let delta_x = input.x2 - input.x1;
    let delta_y = input.y2 - input.y1;
    
    // Step 2: Call pythagorean tool via HTTP
    let pyth_input = PythagoreanInput { a: delta_x, b: delta_y };
    let request_body = match serde_json::to_string(&pyth_input) {
        Ok(body) => body,
        Err(e) => return ToolResponse::text(format!("Error: Failed to serialize pythagorean input: {}. Input: a={}, b={}", e, delta_x, delta_y))
    };
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://pythagorean.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Error calling pythagorean tool: {:?}", e))
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse response body: {}", e))
    };
    
    // Parse the ToolResponse format
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse pythagorean response wrapper: {}", e))
    };
    
    let pyth_result: PythagoreanResult = match serde_json::from_str(&wrapper.content[0].text) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse pythagorean result: {}", e))
    };
    
    let distance = pyth_result.hypotenuse;
    
    let result = DistanceResult {
        distance,
        point1: Point2D { x: input.x1, y: input.y1 },
        point2: Point2D { x: input.x2, y: input.y2 },
        delta_x,
        delta_y,
    };
    
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}

// Library mode - pure function for category use with direct calculation
#[cfg(feature = "library")]
pub fn distance_2d_pure(input: TwoPointInput) -> DistanceResult {
    // Step 1: Calculate differences
    let delta_x = input.x2 - input.x1;
    let delta_y = input.y2 - input.y1;
    
    // Step 2: Calculate distance directly using Pythagorean theorem - no HTTP!
    let distance = (delta_x * delta_x + delta_y * delta_y).sqrt();
    
    DistanceResult {
        distance,
        point1: Point2D { x: input.x1, y: input.y1 },
        point2: Point2D { x: input.x2, y: input.y2 },
        delta_x,
        delta_y,
    }
}