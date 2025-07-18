// Pure Function Validation - Testing library mode dual-mode architecture
// Tests the distance_2d and divide tools' pure function implementations

use distance_2d_tool::{distance_2d, TwoPointInputFlat};
use divide_tool::{divide, TwoNumberInput as DivideTwoNumberInput};
use add_tool::{add, TwoNumberInput as AddTwoNumberInput};
use multiply_tool::{multiply, TwoNumberInput as MultiplyTwoNumberInput};
use subtract_tool::{subtract, TwoNumberInput as SubtractTwoNumberInput};
use power_tool::{power, TwoNumberInput as PowerTwoNumberInput};
use sqrt_tool::{sqrt, SingleNumberInput as SqrtSingleNumberInput};

#[tokio::main]
async fn main() {
    println!("=== Pure Function Validation - Dual-Mode Architecture ===");
    println!("Testing distance_2d, divide, add, multiply, subtract, power, and sqrt tools in library mode");
    println!("Date: {}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    println!();

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Test basic distance calculation (3-4-5 triangle)
    println!("--- Test: Basic Distance (3-4-5 triangle) ---");
    let input = TwoPointInputFlat {
        x1: 0.0,
        y1: 0.0,
        x2: 3.0,
        y2: 4.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: distance = {}", result.distance);
            println!("   Point1: ({}, {})", result.point1.x, result.point1.y);
            println!("   Point2: ({}, {})", result.point2.x, result.point2.y);
            println!("   Delta: ({}, {})", result.delta_x, result.delta_y);
            println!("   Note: {}", result.note);
            
            if result.distance == 5.0 && result.delta_x == 3.0 && result.delta_y == 4.0 {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected distance=5.0, delta_x=3.0, delta_y=4.0");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative coordinates
    println!("--- Test: Negative Coordinates ---");
    let input = TwoPointInputFlat {
        x1: -1.0,
        y1: -1.0,
        x2: 2.0,
        y2: 3.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: distance = {}", result.distance);
            println!("   Delta: ({}, {})", result.delta_x, result.delta_y);
            
            if result.distance == 5.0 && result.delta_x == 3.0 && result.delta_y == 4.0 {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected distance=5.0, delta_x=3.0, delta_y=4.0");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test same point (zero distance)
    println!("--- Test: Same Point (Zero Distance) ---");
    let input = TwoPointInputFlat {
        x1: 5.0,
        y1: 7.0,
        x2: 5.0,
        y2: 7.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: distance = {}", result.distance);
            
            if result.distance == 0.0 && result.delta_x == 0.0 && result.delta_y == 0.0 {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected distance=0.0, delta_x=0.0, delta_y=0.0");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test sqrt(2) diagonal
    println!("--- Test: Unit Square Diagonal (√2) ---");
    let input = TwoPointInputFlat {
        x1: 0.0,
        y1: 0.0,
        x2: 1.0,
        y2: 1.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: distance = {}", result.distance);
            println!("   Expected: √2 = {}", 2.0_f64.sqrt());
            let diff = (result.distance - 2.0_f64.sqrt()).abs();
            println!("   Difference: {}", diff);
            
            if diff < 1e-15 && result.delta_x == 1.0 && result.delta_y == 1.0 {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected √2 distance with delta_x=1.0, delta_y=1.0");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test calculation steps validation
    println!("--- Test: Calculation Steps Validation ---");
    let input = TwoPointInputFlat {
        x1: 0.0,
        y1: 0.0,
        x2: 3.0,
        y2: 4.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: {} calculation steps recorded", result.calculation_steps.len());
            
            let has_differences = result.calculation_steps.iter().any(|step| step.contains("Calculate differences"));
            let has_pythagorean = result.calculation_steps.iter().any(|step| step.contains("pythagorean_pure"));
            let sufficient_steps = result.calculation_steps.len() >= 4;
            
            println!("   Steps:");
            for (i, step) in result.calculation_steps.iter().enumerate() {
                println!("   {}. {}", i + 1, step);
            }
            
            if has_differences && has_pythagorean && sufficient_steps {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Missing required calculation steps");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test input validation (NaN values)
    println!("--- Test: Input Validation (NaN) ---");
    let input = TwoPointInputFlat {
        x1: f64::NAN,
        y1: 0.0,
        x2: 3.0,
        y2: 4.0,
    };
    
    match distance_2d(input).await {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("NaN") || e.contains("invalid") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention NaN or invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // Test large coordinates
    println!("--- Test: Large Coordinates ---");
    let input = TwoPointInputFlat {
        x1: 1000.0,
        y1: 2000.0,
        x2: 1003.0,
        y2: 2004.0,
    };
    
    match distance_2d(input).await {
        Ok(result) => {
            println!("✅ Success: distance = {}", result.distance);
            
            if result.distance == 5.0 && result.delta_x == 3.0 && result.delta_y == 4.0 {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected distance=5.0, delta_x=3.0, delta_y=4.0");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // === DIVIDE TOOL TESTS ===
    println!("=== DIVIDE TOOL TESTS ===");
    println!();

    // Test basic division
    println!("--- Test: Basic Division (10 ÷ 2) ---");
    let input = DivideTwoNumberInput { a: 10.0, b: 2.0 };
    
    match divide(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Operation: {}", result.operation);
            println!("   Inputs: {:?}", result.inputs);
            
            if result.result == 5.0 && result.operation == "division" && result.inputs == vec![10.0, 2.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=5.0, operation='division', inputs=[10.0, 2.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test division by zero
    println!("--- Test: Division by Zero ---");
    let input = DivideTwoNumberInput { a: 10.0, b: 0.0 };
    
    match divide(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with division by zero");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected division by zero");
            println!("   Error: {}", e);
            
            if e.contains("Division by zero") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention division by zero");
                tests_failed += 1;
            }
        }
    }
    println!();

    // Test negative numbers
    println!("--- Test: Negative Numbers (-10 ÷ -2) ---");
    let input = DivideTwoNumberInput { a: -10.0, b: -2.0 };
    
    match divide(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 5.0 && result.inputs == vec![-10.0, -2.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=5.0, inputs=[-10.0, -2.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test fraction result
    println!("--- Test: Fraction Result (7 ÷ 2) ---");
    let input = DivideTwoNumberInput { a: 7.0, b: 2.0 };
    
    match divide(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 3.5 && result.inputs == vec![7.0, 2.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=3.5, inputs=[7.0, 2.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = DivideTwoNumberInput { a: f64::NAN, b: 2.0 };
    
    match divide(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // === ADD TOOL TESTS ===
    println!("=== ADD TOOL TESTS ===");
    println!();

    // Test basic addition
    println!("--- Test: Basic Addition (5 + 3) ---");
    let input = AddTwoNumberInput { a: 5.0, b: 3.0 };
    
    match add(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Operation: {}", result.operation);
            println!("   Inputs: {:?}", result.inputs);
            
            if result.result == 8.0 && result.operation == "addition" && result.inputs == vec![5.0, 3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=8.0, operation='addition', inputs=[5.0, 3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative numbers
    println!("--- Test: Negative Numbers (-5 + -3) ---");
    let input = AddTwoNumberInput { a: -5.0, b: -3.0 };
    
    match add(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == -8.0 && result.inputs == vec![-5.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=-8.0, inputs=[-5.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test mixed signs
    println!("--- Test: Mixed Signs (10 + -3) ---");
    let input = AddTwoNumberInput { a: 10.0, b: -3.0 };
    
    match add(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 7.0 && result.inputs == vec![10.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=7.0, inputs=[10.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero addition
    println!("--- Test: Zero Addition (42 + 0) ---");
    let input = AddTwoNumberInput { a: 42.0, b: 0.0 };
    
    match add(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 42.0 && result.inputs == vec![42.0, 0.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=42.0, inputs=[42.0, 0.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = AddTwoNumberInput { a: f64::NAN, b: 3.0 };
    
    match add(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // === MULTIPLY TOOL TESTS ===
    println!("=== MULTIPLY TOOL TESTS ===");
    println!();

    // Test basic multiplication
    println!("--- Test: Basic Multiplication (6 × 7) ---");
    let input = MultiplyTwoNumberInput { a: 6.0, b: 7.0 };
    
    match multiply(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Operation: {}", result.operation);
            println!("   Inputs: {:?}", result.inputs);
            
            if result.result == 42.0 && result.operation == "multiplication" && result.inputs == vec![6.0, 7.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=42.0, operation='multiplication', inputs=[6.0, 7.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative numbers
    println!("--- Test: Negative Numbers (-4 × -5) ---");
    let input = MultiplyTwoNumberInput { a: -4.0, b: -5.0 };
    
    match multiply(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 20.0 && result.inputs == vec![-4.0, -5.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=20.0, inputs=[-4.0, -5.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test mixed signs
    println!("--- Test: Mixed Signs (8 × -3) ---");
    let input = MultiplyTwoNumberInput { a: 8.0, b: -3.0 };
    
    match multiply(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == -24.0 && result.inputs == vec![8.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=-24.0, inputs=[8.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero multiplication
    println!("--- Test: Zero Multiplication (42 × 0) ---");
    let input = MultiplyTwoNumberInput { a: 42.0, b: 0.0 };
    
    match multiply(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 0.0 && result.inputs == vec![42.0, 0.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=0.0, inputs=[42.0, 0.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = MultiplyTwoNumberInput { a: f64::NAN, b: 3.0 };
    
    match multiply(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // === SUBTRACT TOOL TESTS ===
    println!("=== SUBTRACT TOOL TESTS ===");
    println!();

    // Test basic subtraction
    println!("--- Test: Basic Subtraction (5 - 3) ---");
    let input = SubtractTwoNumberInput { a: 5.0, b: 3.0 };
    
    match subtract(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Operation: {}", result.operation);
            println!("   Inputs: {:?}", result.inputs);
            
            if result.result == 2.0 && result.operation == "subtraction" && result.inputs == vec![5.0, 3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=2.0, operation='subtraction', inputs=[5.0, 3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative numbers
    println!("--- Test: Negative Numbers (-5 - -3) ---");
    let input = SubtractTwoNumberInput { a: -5.0, b: -3.0 };
    
    match subtract(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == -2.0 && result.inputs == vec![-5.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=-2.0, inputs=[-5.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test mixed signs
    println!("--- Test: Mixed Signs (10 - -3) ---");
    let input = SubtractTwoNumberInput { a: 10.0, b: -3.0 };
    
    match subtract(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 13.0 && result.inputs == vec![10.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=13.0, inputs=[10.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero subtraction
    println!("--- Test: Zero Subtraction (42 - 0) ---");
    let input = SubtractTwoNumberInput { a: 42.0, b: 0.0 };
    
    match subtract(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 42.0 && result.inputs == vec![42.0, 0.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=42.0, inputs=[42.0, 0.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = SubtractTwoNumberInput { a: f64::NAN, b: 3.0 };
    
    match subtract(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // === POWER TOOL TESTS ===
    println!("=== POWER TOOL TESTS ===");
    println!();

    // Test basic exponentiation
    println!("--- Test: Basic Exponentiation (2^3) ---");
    let input = PowerTwoNumberInput { a: 2.0, b: 3.0 };
    
    match power(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Operation: {}", result.operation);
            println!("   Inputs: {:?}", result.inputs);
            
            if result.result == 8.0 && result.operation == "exponentiation" && result.inputs == vec![2.0, 3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=8.0, operation='exponentiation', inputs=[2.0, 3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test square operation
    println!("--- Test: Square (5^2) ---");
    let input = PowerTwoNumberInput { a: 5.0, b: 2.0 };
    
    match power(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 25.0 && result.inputs == vec![5.0, 2.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=25.0, inputs=[5.0, 2.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero exponent
    println!("--- Test: Zero Exponent (5^0) ---");
    let input = PowerTwoNumberInput { a: 5.0, b: 0.0 };
    
    match power(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 1.0 && result.inputs == vec![5.0, 0.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=1.0, inputs=[5.0, 0.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative exponent
    println!("--- Test: Negative Exponent (2^-3) ---");
    let input = PowerTwoNumberInput { a: 2.0, b: -3.0 };
    
    match power(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 0.125 && result.inputs == vec![2.0, -3.0] {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=0.125, inputs=[2.0, -3.0]");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero to zero (should error)
    println!("--- Test: Zero to Zero (0^0) ---");
    let input = PowerTwoNumberInput { a: 0.0, b: 0.0 };
    
    match power(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with 0^0");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected 0^0");
            println!("   Error: {}", e);
            
            if e.contains("undefined") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention undefined");
                tests_failed += 1;
            }
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = PowerTwoNumberInput { a: f64::NAN, b: 3.0 };
    
    match power(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    // === SQRT TOOL TESTS ===
    println!("=== SQRT TOOL TESTS ===");
    println!();

    // Test perfect square
    println!("--- Test: Perfect Square (√9) ---");
    let input = SqrtSingleNumberInput { value: 9.0 };
    
    match sqrt(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            println!("   Input: {}", result.input);
            println!("   Is Valid: {}", result.is_valid);
            println!("   Error: {:?}", result.error);
            
            if result.result == 3.0 && result.input == 9.0 && result.is_valid && result.error.is_none() {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=3.0, input=9.0, is_valid=true, error=None");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test zero
    println!("--- Test: Square Root of Zero (√0) ---");
    let input = SqrtSingleNumberInput { value: 0.0 };
    
    match sqrt(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 0.0 && result.input == 0.0 && result.is_valid {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=0.0, input=0.0, is_valid=true");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test non-perfect square
    println!("--- Test: Non-Perfect Square (√2) ---");
    let input = SqrtSingleNumberInput { value: 2.0 };
    
    match sqrt(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            let expected = 2.0_f64.sqrt();
            let diff = (result.result - expected).abs();
            
            if diff < 1e-15 && result.input == 2.0 && result.is_valid {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected √2, input=2.0, is_valid=true");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test negative number (should error in business logic)
    println!("--- Test: Negative Number (√-4) ---");
    let input = SqrtSingleNumberInput { value: -4.0 };
    
    match sqrt(input) {
        Ok(result) => {
            println!("✅ Success: Handled negative input correctly");
            println!("   Result: {}", result.result);
            println!("   Is Valid: {}", result.is_valid);
            println!("   Error: {:?}", result.error);
            
            if !result.is_valid && result.error.is_some() && result.result.is_nan() {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected is_valid=false, result=NaN, error=Some");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test decimal input
    println!("--- Test: Decimal Input (√6.25) ---");
    let input = SqrtSingleNumberInput { value: 6.25 };
    
    match sqrt(input) {
        Ok(result) => {
            println!("✅ Success: result = {}", result.result);
            
            if result.result == 2.5 && result.input == 6.25 && result.is_valid {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Expected result=2.5, input=6.25, is_valid=true");
                tests_failed += 1;
            }
        },
        Err(e) => {
            println!("❌ Error: {}", e);
            tests_failed += 1;
        }
    }
    println!();

    // Test NaN input validation
    println!("--- Test: Input Validation (NaN) ---");
    let input = SqrtSingleNumberInput { value: f64::NAN };
    
    match sqrt(input) {
        Ok(_) => {
            println!("❌ Error: Should have failed with NaN input");
            tests_failed += 1;
        },
        Err(e) => {
            println!("✅ Success: Correctly rejected NaN input");
            println!("   Error: {}", e);
            
            if e.contains("invalid") || e.contains("NaN") {
                tests_passed += 1;
            } else {
                println!("❌ Assertion failed: Error message should mention invalid values");
                tests_failed += 1;
            }
        }
    }
    println!();

    println!("=== SUMMARY ===");
    println!("Tests passed: {}", tests_passed);
    println!("Tests failed: {}", tests_failed);
    println!("Total tests: {}", tests_passed + tests_failed);
    println!();
    println!("This validates the dual-mode architecture library implementations:");
    println!("1. DISTANCE_2D: Pure functions with pythagorean dependency");
    println!("2. DIVIDE: Pure functions with no external dependencies"); 
    println!("3. ADD: Pure functions with no external dependencies");
    println!("4. MULTIPLY: Pure functions with no external dependencies");
    println!("5. SUBTRACT: Pure functions with no external dependencies");
    println!("6. POWER: Pure functions with no external dependencies");
    println!("7. SQRT: Pure functions with no external dependencies");
    println!("8. CONDITIONAL EXPORTS: Single function names in both modes");
    println!("9. MODULAR DESIGN: Business logic separation maintained");
    println!("10. INPUT VALIDATION: Proper error handling for invalid inputs");
    
    if tests_failed == 0 {
        println!("✅ All pure function tests passed!");
        std::process::exit(0);
    } else {
        println!("❌ Some tests failed!");
        std::process::exit(1);
    }
}
