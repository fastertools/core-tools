use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Standard input for operations requiring a single number
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SingleNumberInput {
    /// The number to operate on
    pub value: f64,
}

/// Standard input for operations requiring two numbers
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TwoNumberInput {
    /// First number
    pub a: f64,
    /// Second number
    pub b: f64,
}

/// Standard input for 2D point operations
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

/// Standard output for basic math operations
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ArithmeticResult {
    /// The result of the operation
    pub result: f64,
    /// The operation that was performed
    pub operation: String,
    /// The input values that were used
    pub inputs: Vec<f64>,
}

/// Standard output for operations that can fail
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct SafeArithmeticResult {
    /// The result of the operation (if successful)
    pub result: Option<f64>,
    /// The operation that was performed
    pub operation: String,
    /// The input values that were used
    pub inputs: Vec<f64>,
    /// Whether the operation was successful
    pub success: bool,
    /// Error message if the operation failed
    pub error: Option<String>,
}

impl ArithmeticResult {
    /// Create a new successful result
    pub fn success(operation: &str, result: f64, inputs: Vec<f64>) -> Self {
        Self {
            result,
            operation: operation.to_string(),
            inputs,
        }
    }
}

impl SafeArithmeticResult {
    /// Create a new successful result
    pub fn success(operation: &str, result: f64, inputs: Vec<f64>) -> Self {
        Self {
            result: Some(result),
            operation: operation.to_string(),
            inputs,
            success: true,
            error: None,
        }
    }

    /// Create a new failed result
    pub fn error(operation: &str, inputs: Vec<f64>, error: String) -> Self {
        Self {
            result: None,
            operation: operation.to_string(),
            inputs,
            success: false,
            error: Some(error),
        }
    }
}

/// Pure function signatures for library mode
pub trait BasicMathOperation {
    type Input;
    type Output;

    fn execute(input: Self::Input) -> Self::Output;
}

/// Helper functions for common operations
pub mod helpers {
    use super::*;

    /// Convert SingleNumberInput to f64
    pub fn single_to_f64(input: SingleNumberInput) -> f64 {
        input.value
    }

    /// Convert TwoNumberInput to (f64, f64)
    pub fn two_to_tuple(input: TwoNumberInput) -> (f64, f64) {
        (input.a, input.b)
    }

    /// Convert TwoPointInput to (f64, f64, f64, f64)
    pub fn points_to_tuple(input: TwoPointInput) -> (f64, f64, f64, f64) {
        (input.x1, input.y1, input.x2, input.y2)
    }

    /// Create ArithmeticResult from single input
    pub fn single_result(operation: &str, input: f64, result: f64) -> ArithmeticResult {
        ArithmeticResult::success(operation, result, vec![input])
    }

    /// Create ArithmeticResult from two inputs
    pub fn two_result(operation: &str, a: f64, b: f64, result: f64) -> ArithmeticResult {
        ArithmeticResult::success(operation, result, vec![a, b])
    }

    /// Create ArithmeticResult from four inputs (2D points)
    pub fn points_result(
        operation: &str,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        result: f64,
    ) -> ArithmeticResult {
        ArithmeticResult::success(operation, result, vec![x1, y1, x2, y2])
    }
}
