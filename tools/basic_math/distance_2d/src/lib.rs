use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::{tool, ToolResponse};

// For testing, we need a dummy ToolResponse
#[cfg(test)]
pub struct ToolResponse;
#[cfg(test)]
impl ToolResponse {
    pub fn text(_text: String) -> Self { ToolResponse }
}

// Re-export types from logic module
pub use logic::{TwoPointInput as LogicInput, DistanceResult as LogicOutput, Point2D as LogicPoint2D};

#[derive(Deserialize, Serialize, JsonSchema)]
struct Point2D {
    /// X coordinate
    x: f64,
    /// Y coordinate
    y: f64,
}

#[derive(Deserialize, JsonSchema)]
struct TwoPointInput {
    /// First point
    point1: Point2D,
    /// Second point
    point2: Point2D,
}

#[derive(Serialize)]
struct DistanceResult {
    distance: f64,
    point1: Point2D,
    point2: Point2D,
    delta_x: f64,
    delta_y: f64,
    calculation_steps: Vec<String>,
    note: String,
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
    #[allow(dead_code)]
    leg_a: f64,
    #[allow(dead_code)]
    leg_b: f64,
    #[allow(dead_code)]
    calculation_steps: Vec<String>,
    #[allow(dead_code)]
    tool_calls: Vec<String>,
}

#[derive(Deserialize)]
struct ToolResponseWrapper {
    content: Vec<ContentItem>,
}

#[derive(Deserialize)]
struct ContentItem {
    #[serde(rename = "type")]
    #[allow(dead_code)]
    content_type: String,
    text: String,
}

/// Calculate the distance between two 2D points using the Pythagorean theorem
/// This demonstrates tool composition by calling the pythagorean tool via Spin's local chaining pattern
#[cfg_attr(not(test), tool)]
async fn distance_2d(input: TwoPointInput) -> ToolResponse {
    use spin_sdk::http::{Method, Request};
    
    let mut calculation_steps = Vec::new();
    
    // Step 1: Calculate differences
    let delta_x = input.point2.x - input.point1.x;
    let delta_y = input.point2.y - input.point1.y;
    calculation_steps.push("Step 1: Calculate differences".to_string());
    calculation_steps.push(format!("Δx = {} - {} = {}", input.point2.x, input.point1.x, delta_x));
    calculation_steps.push(format!("Δy = {} - {} = {}", input.point2.y, input.point1.y, delta_y));
    
    // Step 2: Call pythagorean tool via HTTP
    calculation_steps.push("Step 2: Apply Pythagorean theorem to Δx and Δy via HTTP call".to_string());
    calculation_steps.push(format!("Calling POST http://pythagorean.spin.internal with a: {}, b: {}", delta_x, delta_y));
    
    let pyth_input = PythagoreanInput { a: delta_x, b: delta_y };
    let request_body = serde_json::to_string(&pyth_input).unwrap();
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://pythagorean.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error calling pythagorean tool: {:?}", e)),
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Failed to parse response body: {}", e)),
    };
    
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(w) => w,
        Err(e) => return ToolResponse::text(format!("Failed to parse tool response: {}", e)),
    };
    
    let content = match wrapper.content.get(0) {
        Some(c) => c,
        None => return ToolResponse::text("No content in tool response".to_string()),
    };
    
    let pyth_result: PythagoreanResult = match serde_json::from_str(&content.text) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Failed to parse pythagorean result: {}", e)),
    };
    
    let distance = pyth_result.hypotenuse;
    calculation_steps.push(format!("Result: Distance = {}", distance));
    
    let response = DistanceResult {
        distance,
        point1: input.point1,
        point2: input.point2,
        delta_x,
        delta_y,
        calculation_steps,
        note: "This uses Spin's local service chaining to call http://pythagorean.spin.internal".to_string(),
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}