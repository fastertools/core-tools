use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct PythagoreanInput {
    /// First leg of right triangle
    a: f64,
    /// Second leg of right triangle
    b: f64,
}

#[derive(Serialize)]
struct PythagoreanResult {
    hypotenuse: f64,
    leg_a: f64,
    leg_b: f64,
    calculation_steps: Vec<String>,
    tool_calls: Vec<String>,
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
    #[allow(dead_code)]
    operation: String,
    #[allow(dead_code)]
    inputs: Vec<f64>,
}

#[derive(Deserialize)]
struct SquareRootResult {
    result: f64,
    #[allow(dead_code)]
    input: f64,
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
    #[allow(dead_code)]
    content_type: String,
    text: String,
}

/// Calculate the hypotenuse of a right triangle using the Pythagorean theorem: c = sqrt(a² + b²)
/// This demonstrates tool composition by calling other tools via Spin's local chaining pattern
#[tool]
async fn pythagorean(input: PythagoreanInput) -> ToolResponse {
    use spin_sdk::http::{Method, Request};
    
    let mut calculation_steps = Vec::new();
    let mut tool_calls = Vec::new();
    
    // Step 1: Square first leg (a²) by calling /square
    calculation_steps.push(format!("Step 1: Square first leg: {}² = ?", input.a));
    tool_calls.push(format!("POST http://square.spin.internal with value: {}", input.a));
    
    let square_input = SingleNumberInput { value: input.a };
    let request_body = serde_json::to_string(&square_input).unwrap();
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://square.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error calling square tool: {:?}", e)),
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
    
    let square_result: ArithmeticResult = match serde_json::from_str(&content.text) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Failed to parse square result: {}", e)),
    };
    
    let a_squared = square_result.result;
    calculation_steps.push(format!("Result: {}² = {}", input.a, a_squared));
    
    // Step 2: Square second leg (b²) by calling /square
    calculation_steps.push(format!("Step 2: Square second leg: {}² = ?", input.b));
    tool_calls.push(format!("POST http://square.spin.internal with value: {}", input.b));
    
    let square_input = SingleNumberInput { value: input.b };
    let request_body = serde_json::to_string(&square_input).unwrap();
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://square.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error calling square tool: {:?}", e)),
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
    
    let square_result: ArithmeticResult = match serde_json::from_str(&content.text) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Failed to parse square result: {}", e)),
    };
    
    let b_squared = square_result.result;
    calculation_steps.push(format!("Result: {}² = {}", input.b, b_squared));
    
    // Step 3: Add the squares (a² + b²) by calling /add
    calculation_steps.push(format!("Step 3: Add squares: {} + {} = ?", a_squared, b_squared));
    tool_calls.push(format!("POST http://add.spin.internal with a: {}, b: {}", a_squared, b_squared));
    
    let add_input = TwoNumberInput { a: a_squared, b: b_squared };
    let request_body = serde_json::to_string(&add_input).unwrap();
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://add.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error calling add tool: {:?}", e)),
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
    
    let add_result: ArithmeticResult = match serde_json::from_str(&content.text) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Failed to parse add result: {}", e)),
    };
    
    let sum_of_squares = add_result.result;
    calculation_steps.push(format!("Result: {} + {} = {}", a_squared, b_squared, sum_of_squares));
    
    // Step 4: Take square root (sqrt(a² + b²)) by calling /sqrt
    calculation_steps.push(format!("Step 4: Take square root: sqrt({}) = ?", sum_of_squares));
    tool_calls.push(format!("POST http://sqrt.spin.internal with value: {}", sum_of_squares));
    
    let sqrt_input = SingleNumberInput { value: sum_of_squares };
    let request_body = serde_json::to_string(&sqrt_input).unwrap();
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://sqrt.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = match spin_sdk::http::send(request).await {
        Ok(resp) => resp,
        Err(e) => return ToolResponse::text(format!("Error calling sqrt tool: {:?}", e)),
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
    
    let sqrt_result: SquareRootResult = match serde_json::from_str(&content.text) {
        Ok(r) => r,
        Err(e) => return ToolResponse::text(format!("Failed to parse sqrt result: {}", e)),
    };
    
    if !sqrt_result.is_valid {
        return ToolResponse::text(format!("Error: {}", sqrt_result.error.unwrap_or("Invalid sqrt result".to_string())));
    }
    
    let hypotenuse = sqrt_result.result;
    calculation_steps.push(format!("Result: sqrt({}) = {}", sum_of_squares, hypotenuse));
    
    let response = PythagoreanResult {
        hypotenuse,
        leg_a: input.a,
        leg_b: input.b,
        calculation_steps,
        tool_calls,
    };
    
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}