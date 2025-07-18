use ftl_sdk::{tool, ToolResponse};
use serde_json;

// Import ALL dual-mode basic math tools - using library mode
use add_tool::{add, TwoNumberInput as AddInput};
use subtract_tool::{subtract, TwoNumberInput as SubtractInput};
use multiply_tool::{multiply, TwoNumberInput as MultiplyInput};
use divide_tool::{divide, TwoNumberInput as DivideInput};
use power_tool::{power, TwoNumberInput as PowerInput};
use sqrt_tool::{sqrt, SingleNumberInput as SqrtInput};
use square_tool::{square, SingleNumberInput as SquareInput};
use remainder_tool::{remainder, TwoNumberInput as RemainderInput};
use modulus_tool::{modulus, TwoNumberInput as ModulusInput};
use distance_2d_tool::{distance_2d, TwoPointInputFlat as DistanceInput};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
pub struct BasicMathRequest {
    /// The operation to perform
    pub operation: String,
    /// The operands for the operation
    pub operands: Vec<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CategoryResult {
    pub operation: String,
    pub operands: Vec<f64>,
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

impl CategoryResult {
    fn success(operation: &str, operands: Vec<f64>, result: serde_json::Value) -> Self {
        CategoryResult {
            operation: operation.to_string(),
            operands,
            success: true,
            result: Some(result),
            error: None,
        }
    }
    
    fn error(operation: &str, operands: Vec<f64>, error: String) -> Self {
        CategoryResult {
            operation: operation.to_string(),
            operands,
            success: false,
            result: None,
            error: Some(error),
        }
    }
}

#[tool]
pub async fn basic_math_category(input: BasicMathRequest) -> ToolResponse {
    let result = match input.operation.as_str() {
        // Two-number operations
        "add" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "add", input.operands, "Add requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let add_input = AddInput { a: input.operands[0], b: input.operands[1] };
            match add(add_input) {
                Ok(result) => CategoryResult::success("add", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("add", input.operands, e),
            }
        }
        "subtract" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "subtract", input.operands, "Subtract requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let subtract_input = SubtractInput { a: input.operands[0], b: input.operands[1] };
            match subtract(subtract_input) {
                Ok(result) => CategoryResult::success("subtract", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("subtract", input.operands, e),
            }
        }
        "multiply" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "multiply", input.operands, "Multiply requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let multiply_input = MultiplyInput { a: input.operands[0], b: input.operands[1] };
            match multiply(multiply_input) {
                Ok(result) => CategoryResult::success("multiply", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("multiply", input.operands, e),
            }
        }
        "divide" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "divide", input.operands, "Divide requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let divide_input = DivideInput { a: input.operands[0], b: input.operands[1] };
            match divide(divide_input) {
                Ok(result) => CategoryResult::success("divide", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("divide", input.operands, e),
            }
        }
        "power" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "power", input.operands, "Power requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let power_input = PowerInput { a: input.operands[0], b: input.operands[1] };
            match power(power_input) {
                Ok(result) => CategoryResult::success("power", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("power", input.operands, e),
            }
        }
        "remainder" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "remainder", input.operands, "Remainder requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let remainder_input = RemainderInput { a: input.operands[0], b: input.operands[1] };
            match remainder(remainder_input) {
                Ok(result) => CategoryResult::success("remainder", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("remainder", input.operands, e),
            }
        }
        "modulus" => {
            if input.operands.len() != 2 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "modulus", input.operands, "Modulus requires exactly 2 operands".to_string()
                )).unwrap());
            }
            let modulus_input = ModulusInput { a: input.operands[0], b: input.operands[1] };
            match modulus(modulus_input) {
                Ok(result) => CategoryResult::success("modulus", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("modulus", input.operands, e),
            }
        }
        
        // Single-number operations
        "sqrt" => {
            if input.operands.len() != 1 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "sqrt", input.operands, "Sqrt requires exactly 1 operand".to_string()
                )).unwrap());
            }
            let sqrt_input = SqrtInput { value: input.operands[0] };
            match sqrt(sqrt_input) {
                Ok(result) => CategoryResult::success("sqrt", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("sqrt", input.operands, e),
            }
        }
        "square" => {
            if input.operands.len() != 1 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "square", input.operands, "Square requires exactly 1 operand".to_string()
                )).unwrap());
            }
            let square_input = SquareInput { value: input.operands[0] };
            match square(square_input) {
                Ok(result) => CategoryResult::success("square", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("square", input.operands, e),
            }
        }
        
        // Four-number operations (coordinate-based)
        "distance_2d" => {
            if input.operands.len() != 4 {
                return ToolResponse::text(serde_json::to_string(&CategoryResult::error(
                    "distance_2d", input.operands, "Distance_2d requires exactly 4 operands (x1, y1, x2, y2)".to_string()
                )).unwrap());
            }
            let distance_input = DistanceInput { 
                x1: input.operands[0], 
                y1: input.operands[1], 
                x2: input.operands[2], 
                y2: input.operands[3] 
            };
            match distance_2d(distance_input).await {
                Ok(result) => CategoryResult::success("distance_2d", input.operands, serde_json::to_value(result).unwrap()),
                Err(e) => CategoryResult::error("distance_2d", input.operands, e),
            }
        }
        
        _ => {
            CategoryResult::error(
                &input.operation, 
                input.operands, 
                format!("Unknown operation: {}. Supported: add, subtract, multiply, divide, power, remainder, modulus, sqrt, square, distance_2d", input.operation)
            )
        }
    };

    ToolResponse::text(serde_json::to_string(&result).unwrap())
}