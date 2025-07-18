use basic_math_types::{TwoNumberInput, ArithmeticResult, SafeArithmeticResult, helpers};

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

#[cfg(feature = "individual")]
use serde_json;

mod logic;

// Re-export standardized types for external use
pub use basic_math_types;

// Individual component mode - FTL tool
#[cfg(feature = "individual")]
#[cfg_attr(not(test), tool)]
pub fn subtract(input: TwoNumberInput) -> ToolResponse {
    let (a, b) = helpers::two_to_tuple(input);
    let result = a - b;
    let response = helpers::two_result("subtract", a, b, result);
    ToolResponse::text(serde_json::to_string(&response).unwrap())
}

// Library mode - pure function for category use
#[cfg(feature = "library")]
pub fn subtract_pure(a: f64, b: f64) -> f64 {
    a - b
}

// Library mode - structured function for category use
#[cfg(feature = "library")]
pub fn subtract_structured(input: TwoNumberInput) -> ArithmeticResult {
    let (a, b) = helpers::two_to_tuple(input);
    let result = a - b;
    helpers::two_result("subtract", a, b, result)
}