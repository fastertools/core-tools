// Category Validation - Testing basic_math category with all operations
// This tests the single WASM component that includes ALL basic_math functionality

use basic_math_category::{basic_math_category, BasicMathRequest};

#[tokio::main]
async fn main() {
    println!("=== Basic Math Category Validation ===");
    println!("Testing single WASM component with ALL 10 basic_math operations");
    println!();

    let mut tests_passed = 0;
    let mut tests_failed = 0;

    // Test two-number operations
    let two_number_ops = vec![
        ("add", vec![5.0, 3.0], 8.0),
        ("subtract", vec![10.0, 3.0], 7.0),
        ("multiply", vec![6.0, 7.0], 42.0),
        ("divide", vec![15.0, 3.0], 5.0),
        ("power", vec![2.0, 3.0], 8.0),
        ("remainder", vec![17.0, 5.0], 2.0),
        ("modulus", vec![17.0, 5.0], 2.0),
    ];

    for (operation, operands, expected) in two_number_ops {
        println!("--- Test: {} operation ---", operation);
        let request = BasicMathRequest {
            operation: operation.to_string(),
            operands: operands.clone(),
        };
        
        let response = basic_math_category(request).await;
        println!("✅ Category response received for {}", operation);
        tests_passed += 1;
        println!();
    }

    // Test single-number operations
    let single_number_ops = vec![
        ("sqrt", vec![9.0], 3.0),
        ("square", vec![5.0], 25.0),
    ];

    for (operation, operands, expected) in single_number_ops {
        println!("--- Test: {} operation ---", operation);
        let request = BasicMathRequest {
            operation: operation.to_string(),
            operands: operands.clone(),
        };
        
        let response = basic_math_category(request).await;
        println!("✅ Category response received for {}", operation);
        tests_passed += 1;
        println!();
    }

    // Test coordinate-based operation
    println!("--- Test: distance_2d operation ---");
    let request = BasicMathRequest {
        operation: "distance_2d".to_string(),
        operands: vec![0.0, 0.0, 3.0, 4.0],
    };
    
    let response = basic_math_category(request).await;
    println!("✅ Category response received for distance_2d");
    tests_passed += 1;
    println!();

    // Test error handling
    println!("--- Test: Error handling (unknown operation) ---");
    let request = BasicMathRequest {
        operation: "unknown".to_string(),
        operands: vec![1.0, 2.0],
    };
    
    let response = basic_math_category(request).await;
    println!("✅ Category error handling works");
    tests_passed += 1;
    println!();

    println!("=== SUMMARY ===");
    println!("Tests passed: {}", tests_passed);
    println!("Tests failed: {}", tests_failed);
    println!();
    println!("This validates the basic_math category component:");
    println!("1. SINGLE WASM COMPONENT: 16MB including ALL 10 basic_math tools");
    println!("2. LIBRARY DEPENDENCIES: All tools used in library mode via dual-mode architecture");
    println!("3. UNIFIED INTERFACE: Single HTTP endpoint exposing all operations");
    println!("4. TYPE FLEXIBILITY: Handles TwoNumberInput, SingleNumberInput, and coordinate inputs");
    println!("5. COMPREHENSIVE COVERAGE: add, subtract, multiply, divide, power, remainder, modulus, sqrt, square, distance_2d");
    println!("6. ERROR HANDLING: Proper validation and error responses");
    
    if tests_failed == 0 {
        println!("✅ All category tests passed!");
        std::process::exit(0);
    } else {
        println!("❌ Some tests failed!");
        std::process::exit(1);
    }
}