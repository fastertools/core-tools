use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

use ftl_sdk::ToolResponse;

// Re-export types from logic module
pub use logic::{PythagoreanInput as LogicInput, PythagoreanResult as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PythagoreanInput {
    /// First leg of right triangle
    pub a: f64,
    /// Second leg of right triangle
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct PythagoreanResult {
    /// The calculated hypotenuse
    pub hypotenuse: f64,
    /// First leg (input a)
    pub leg_a: f64,
    /// Second leg (input b)
    pub leg_b: f64,
    /// Square of first leg
    pub a_squared: f64,
    /// Square of second leg
    pub b_squared: f64,
    /// Sum of squares
    pub sum_of_squares: f64,
}

// Helper structs for calling other tools
#[derive(Serialize)]
struct SingleNumberInput {
    value: f64,
}

#[derive(Serialize)]
struct TwoNumberInput {
    a: f64,
    b: f64,
}

#[derive(Deserialize)]
struct ArithmeticResult {
    result: f64,
    operation: String,
    inputs: Vec<f64>,
}

#[derive(Deserialize)]
struct SquareRootResult {
    result: f64,
    is_valid: bool,
    error: Option<String>,
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

/// Calculate the hypotenuse of a right triangle using the Pythagorean theorem: c = sqrt(a² + b²)
/// This demonstrates tool composition by calling other tools via Spin's local chaining pattern
#[cfg_attr(not(test), tool)]
pub async fn pythagorean(input: PythagoreanInput) -> ToolResponse {
    use spin_sdk::http::{Method, Request};
    
    // Step 1: Square first leg (a²) by calling /square
    let square_input = SingleNumberInput { value: input.a };
    let request_body = match serde_json::to_string(&square_input) {
        Ok(body) => body,
        Err(e) => return ToolResponse::text(format!("Error: Failed to serialize square input: {}", e))
    };
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://square.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Error calling square tool: {:?}", e))
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse response body: {}", e))
    };
    
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse square response wrapper: {}", e))
    };
    
    let square_result: ArithmeticResult = match serde_json::from_str(&wrapper.content[0].text) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse square result: {}", e))
    };
    
    let a_squared = square_result.result;
    
    // Step 2: Square second leg (b²) by calling /square
    let square_input = SingleNumberInput { value: input.b };
    let request_body = match serde_json::to_string(&square_input) {
        Ok(body) => body,
        Err(e) => return ToolResponse::text(format!("Error: Failed to serialize square input: {}", e))
    };
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://square.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Error calling square tool: {:?}", e))
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse response body: {}", e))
    };
    
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse square response wrapper: {}", e))
    };
    
    let square_result: ArithmeticResult = match serde_json::from_str(&wrapper.content[0].text) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse square result: {}", e))
    };
    
    let b_squared = square_result.result;
    
    // Step 3: Add the squares (a² + b²) by calling /add
    let add_input = TwoNumberInput { a: a_squared, b: b_squared };
    let request_body = match serde_json::to_string(&add_input) {
        Ok(body) => body,
        Err(e) => return ToolResponse::text(format!("Error: Failed to serialize add input: {}", e))
    };
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://add.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Error calling add tool: {:?}", e))
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse response body: {}", e))
    };
    
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse add response wrapper: {}", e))
    };
    
    let add_result: ArithmeticResult = match serde_json::from_str(&wrapper.content[0].text) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse add result: {}", e))
    };
    
    let sum_of_squares = add_result.result;
    
    // Step 4: Take square root (sqrt(a² + b²)) by calling /sqrt
    let sqrt_input = SingleNumberInput { value: sum_of_squares };
    let request_body = match serde_json::to_string(&sqrt_input) {
        Ok(body) => body,
        Err(e) => return ToolResponse::text(format!("Error: Failed to serialize sqrt input: {}", e))
    };
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://sqrt.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Error calling sqrt tool: {:?}", e))
    };
    
    let body_bytes = response.into_body();
    let body = match String::from_utf8(body_bytes) {
        Ok(b) => b,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse response body: {}", e))
    };
    
    let wrapper: ToolResponseWrapper = match serde_json::from_str(&body) {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse sqrt response wrapper: {}", e))
    };
    
    let sqrt_result: SquareRootResult = match serde_json::from_str(&wrapper.content[0].text) {
        Ok(result) => result,
        Err(e) => return ToolResponse::text(format!("Error: Failed to parse sqrt result: {}", e))
    };
    
    if !sqrt_result.is_valid {
        return ToolResponse::text(format!("Error: {}", sqrt_result.error.unwrap_or("Invalid sqrt result".to_string())));
    }
    
    let hypotenuse = sqrt_result.result;
    
    let result = PythagoreanResult {
        hypotenuse,
        leg_a: input.a,
        leg_b: input.b,
        a_squared,
        b_squared,
        sum_of_squares,
    };
    
    ToolResponse::text(serde_json::to_string(&result).unwrap())
}