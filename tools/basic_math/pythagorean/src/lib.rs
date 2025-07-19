use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(all(feature = "individual", not(test)))]
use ftl_sdk::tool;

#[cfg(feature = "individual")]
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


/// Calculate the hypotenuse of a right triangle using the Pythagorean theorem: c = sqrt(a² + b²)
#[cfg_attr(not(test), tool)]
pub fn pythagorean(input: PythagoreanInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        a: input.a,
        b: input.b,
    };
    
    // Call logic implementation
    match logic::calculate_pythagorean(logic_input) {
        Ok(result) => {
            // Calculate intermediate values for the wrapper type
            let a_squared = input.a * input.a;
            let b_squared = input.b * input.b;
            let sum_of_squares = a_squared + b_squared;
            
            // Convert back to wrapper types
            let response = PythagoreanResult {
                hypotenuse: result.hypotenuse,
                leg_a: result.leg_a,
                leg_b: result.leg_b,
                a_squared,
                b_squared,
                sum_of_squares,
            };
            ToolResponse::text(serde_json::to_string(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}

