use ftl_sdk::{tool, ToolResponse};
use serde_json;
use basic_math_types::{TwoNumberInput, ArithmeticResult, SafeArithmeticResult, helpers};

// Import the pure functions from standardized basic math tools
use add_tool::add_pure;
use subtract_tool::subtract_pure;
use multiply_tool::multiply_pure;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct BasicMathRequest {
    /// The operation to perform
    pub operation: String,
    /// The operands for the operation
    pub operands: Vec<f64>,
}

#[tool]
pub fn basic_math_category(input: BasicMathRequest) -> ToolResponse {
    let result = match input.operation.as_str() {
        "add" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&SafeArithmeticResult::error(
                    "add", 
                    input.operands.clone(), 
                    "Add operation requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let result = add_pure(input.operands[0], input.operands[1]);
            SafeArithmeticResult::success("add", result, input.operands.clone())
        }
        "subtract" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&SafeArithmeticResult::error(
                    "subtract", 
                    input.operands.clone(), 
                    "Subtract operation requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let result = subtract_pure(input.operands[0], input.operands[1]);
            SafeArithmeticResult::success("subtract", result, input.operands.clone())
        }
        "multiply" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&SafeArithmeticResult::error(
                    "multiply", 
                    input.operands.clone(), 
                    "Multiply operation requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let result = multiply_pure(input.operands[0], input.operands[1]);
            SafeArithmeticResult::success("multiply", result, input.operands.clone())
        }
        _ => {
            return ToolResponse::text(serde_json::to_string(&SafeArithmeticResult::error(
                &input.operation, 
                input.operands.clone(), 
                format!("Unknown operation: {}", input.operation)
            )).unwrap());
        }
    };

    ToolResponse::text(serde_json::to_string(&result).unwrap())
}