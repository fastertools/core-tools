use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

// Re-export logic module types
mod logic;
pub use logic::*;

// FTL-compatible input type (flattened for HTTP interface)  
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoPointInputFlat {
    /// X coordinate of first point
    pub x1: f64,
    /// Y coordinate of first point
    pub y1: f64,
    /// X coordinate of second point
    pub x2: f64,
    /// Y coordinate of second point
    pub y2: f64,
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

// Helper function to convert flat input to structured input
fn flat_to_structured(input: TwoPointInputFlat) -> TwoPointInput {
    TwoPointInput {
        point1: Point2D { x: input.x1, y: input.y1 },
        point2: Point2D { x: input.x2, y: input.y2 },
    }
}

// Conditional pythagorean helper - decides HTTP vs pure based on feature
#[cfg(feature = "individual")]
async fn conditional_pythagorean(input: PythagoreanInput) -> Result<f64, String> {
    // Individual mode: HTTP call
    use spin_sdk::http::{Method, Request};
    
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize pythagorean input: {}", e))?;
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://pythagorean.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
        .map_err(|e| format!("Error calling pythagorean tool: {:?}", e))?;
    
    let body = String::from_utf8(response.into_body())
        .map_err(|e| format!("Failed to parse response body: {}", e))?;
    
    let wrapper: ToolResponseWrapper = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse pythagorean response wrapper: {}", e))?;
    
    let pyth_result: PythagoreanResult = serde_json::from_str(&wrapper.content[0].text)
        .map_err(|e| format!("Failed to parse pythagorean result: {}", e))?;
    
    Ok(pyth_result.hypotenuse)
}

#[cfg(feature = "library")]
async fn conditional_pythagorean(input: pythagorean_tool::PythagoreanInput) -> Result<f64, String> {
    // Library mode: Direct function call
    use pythagorean_tool::pythagorean_pure;
    
    let pyth_result = pythagorean_pure(input);
    Ok(pyth_result.hypotenuse)
}

// Core implementation - shared between both modes
async fn distance_2d_impl(input: TwoPointInputFlat) -> Result<DistanceResult, String> {
    let structured_input = flat_to_structured(input);
    
    // Validate input
    validate_input(&structured_input)?;
    
    // Calculate delta values
    let delta_x = structured_input.point2.x - structured_input.point1.x;
    let delta_y = structured_input.point2.y - structured_input.point1.y;
    
    // Get distance via conditional pythagorean (HTTP or pure based on feature)
    #[cfg(feature = "individual")]
    let pyth_input = PythagoreanInput { a: delta_x, b: delta_y };
    #[cfg(feature = "library")]
    let pyth_input = pythagorean_tool::PythagoreanInput { a: delta_x, b: delta_y };
    
    let distance = conditional_pythagorean(pyth_input).await?;
    
    // Build calculation steps for traceability
    let mut calculation_steps = Vec::new();
    calculation_steps.push("Step 1: Calculate differences".to_string());
    calculation_steps.push(format!("Δx = {} - {} = {}", structured_input.point2.x, structured_input.point1.x, delta_x));
    calculation_steps.push(format!("Δy = {} - {} = {}", structured_input.point2.y, structured_input.point1.y, delta_y));
    
    #[cfg(feature = "individual")]
    calculation_steps.push("Step 2: Call pythagorean tool via HTTP".to_string());
    #[cfg(feature = "library")]
    calculation_steps.push("Step 2: Call pythagorean_pure function directly".to_string());
    
    calculation_steps.push(format!("distance = pythagorean({}, {}) = {}", delta_x, delta_y, distance));
    
    // Format result using pure business logic
    Ok(format_distance_result(structured_input, distance, calculation_steps))
}

// Library mode: Primary export for pure function usage
#[cfg(feature = "library")]
pub async fn distance_2d(input: TwoPointInputFlat) -> Result<DistanceResult, String> {
    distance_2d_impl(input).await
}

// Individual mode: HTTP-based tool handler
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub async fn distance_2d(input: TwoPointInputFlat) -> ToolResponse {
    match distance_2d_impl(input).await {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}