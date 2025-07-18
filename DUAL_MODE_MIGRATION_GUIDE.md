# Dual-Mode Architecture Migration Guide

## Executive Summary

**Problem**: Tools currently have separate implementations for individual (WASM components) and library (pure functions) modes, leading to code duplication and maintenance overhead.

**Solution**: Single codebase with conditional exports that adapt behavior based on compile-time feature flags.

**Benefits**:
- Eliminates code duplication
- Ensures consistent behavior across modes
- Simplifies maintenance and testing
- Provides clean API with single function names

## Architecture Overview

### Dual-Mode Concept

Tools compile in two modes:
- **Individual Mode**: WASM components with HTTP handlers for Spin deployment
- **Library Mode**: Pure functions for direct import as Rust dependencies

### Key Innovation: Conditional Exports

Instead of separate functions (`tool_name`, `tool_name_pure`, `tool_name_handler`), we use a single function name that adapts based on compile features:

```rust
// Library mode: Pure function
#[cfg(feature = "library")]
pub fn tool_name(input: StructInput) -> Result<StructOutput, String>

// Individual mode: HTTP handler  
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub fn tool_name(input: StructInput) -> ToolResponse
```

## Pattern Reference

### Pattern A: No External Dependencies (Example: divide)

For tools that don't call other tools:

**File Structure:**
```
tool_name/
├── Cargo.toml          # Feature flags and dependencies
├── src/
│   ├── lib.rs          # I/O handling, conditional exports
│   └── logic.rs        # Pure business logic, comprehensive tests
```

**lib.rs Template:**
```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[cfg(feature = "individual")]
use ftl_sdk::{tool, ToolResponse};

// Re-export logic module types
mod logic;
pub use logic::*;

// FTL-compatible input type (with JsonSchema for HTTP interface)
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolInput {
    /// First parameter description
    pub a: f64,
    /// Second parameter description  
    pub b: f64,
}

// Core implementation - shared between both modes
fn tool_name_impl(input: ToolInput) -> Result<ToolOutput, String> {
    // Convert to logic types
    let logic_input = logic::ToolInput {
        a: input.a,
        b: input.b,
    };
    
    // Call pure business logic
    logic::tool_operation(logic_input)
}

// Library mode: Primary export for pure function usage
#[cfg(feature = "library")]
pub fn tool_name(input: ToolInput) -> Result<ToolOutput, String> {
    tool_name_impl(input)
}

// Individual mode: HTTP-based tool handler
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub fn tool_name(input: ToolInput) -> ToolResponse {
    match tool_name_impl(input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}
```

**logic.rs Template:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub a: f64,
    pub b: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub result: f64,
    pub operation: String,
    pub inputs: Vec<f64>,
}

pub fn tool_operation(input: ToolInput) -> Result<ToolOutput, String> {
    // Input validation
    if input.a.is_nan() || input.a.is_infinite() ||
       input.b.is_nan() || input.b.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    // Business logic specific validation (e.g., division by zero)
    // ... tool-specific validation ...
    
    let result = /* tool-specific calculation */;
    
    Ok(ToolOutput {
        result,
        operation: "operation_name".to_string(),
        inputs: vec![input.a, input.b],
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_operation() {
        let input = ToolInput { a: 10.0, b: 2.0 };
        let result = tool_operation(input).unwrap();
        // Assert expected results
    }
    
    #[test]
    fn test_error_conditions() {
        let input = ToolInput { a: f64::NAN, b: 2.0 };
        let result = tool_operation(input);
        assert!(result.is_err());
    }
    
    // Additional comprehensive tests...
}
```

**Cargo.toml Template:**
```toml
[package]
name = "tool_name_tool"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib", "rlib"]

[features]
default = ["individual"]
individual = ["ftl-sdk/macros", "spin-sdk"]
library = []

[dependencies]
ftl-sdk = { version = "0.2.3", features = ["macros"], optional = true }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"
spin-sdk = { version = "4.0", optional = true }
```

### Pattern B: With External Dependencies (Example: distance_2d)

For tools that call other tools:

**Additional Requirements:**
1. Conditional helper functions for HTTP vs pure calls
2. Different struct types for different modes
3. HTTP parsing for ToolResponseWrapper/ContentItem
4. Dependency management in Cargo.toml

**lib.rs Additions for Dependencies:**
```rust
// Helper structs for calling dependency tool
#[derive(Serialize)]
struct DependencyInput {
    a: f64,
    b: f64,
}

#[derive(Deserialize)]
struct DependencyResult {
    result_field: f64,
}

// HTTP response parsing structs
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

// Conditional dependency helper - decides HTTP vs pure based on feature
#[cfg(feature = "individual")]
async fn conditional_dependency(input: DependencyInput) -> Result<f64, String> {
    // Individual mode: HTTP call
    use spin_sdk::http::{Method, Request};
    
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize dependency input: {}", e))?;
    
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://dependency-tool.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();
    
    let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
        .map_err(|e| format!("Error calling dependency tool: {:?}", e))?;
    
    let body = String::from_utf8(response.into_body())
        .map_err(|e| format!("Failed to parse response body: {}", e))?;
    
    let wrapper: ToolResponseWrapper = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse dependency response wrapper: {}", e))?;
    
    let dep_result: DependencyResult = serde_json::from_str(&wrapper.content[0].text)
        .map_err(|e| format!("Failed to parse dependency result: {}", e))?;
    
    Ok(dep_result.result_field)
}

#[cfg(feature = "library")]
async fn conditional_dependency(input: dependency_tool::DependencyInput) -> Result<f64, String> {
    // Library mode: Direct function call
    use dependency_tool::dependency_pure;
    
    let dep_result = dependency_pure(input);
    Ok(dep_result.result_field)
}
```

**Cargo.toml Additions for Dependencies:**
```toml
[features]
default = ["individual"]
individual = ["ftl-sdk/macros", "spin-sdk"]
library = ["dependency_tool"]

# Library mode dependency
dependency_tool = { path = "../dependency", default-features = false, features = ["library"], optional = true }
```

## Migration Checklist

### Pre-Migration Assessment
- [ ] Identify if tool has external dependencies (Pattern A vs Pattern B)
- [ ] Review current tool structure and functionality
- [ ] Check existing tests for coverage requirements

### File Modifications

#### 1. Update Cargo.toml
- [ ] Add proper feature flags (`individual`, `library`)
- [ ] Set default to `["individual"]`
- [ ] Make ftl-sdk and spin-sdk optional with individual feature
- [ ] Add dependency_tool with library feature if needed (Pattern B only)

#### 2. Rewrite lib.rs
- [ ] Add conditional imports for ftl_sdk
- [ ] Create FTL-compatible input struct with JsonSchema
- [ ] Implement shared `tool_name_impl` function
- [ ] Add conditional exports for both modes
- [ ] For Pattern B: Add conditional dependency helpers
- [ ] Remove any duplicate function definitions

#### 3. Review logic.rs
- [ ] Ensure no conditional compilation (#[cfg]) in business logic
- [ ] Verify comprehensive test coverage
- [ ] Confirm input validation and error handling
- [ ] Ensure struct-based parameters throughout

### Testing Requirements (MANDATORY)

#### 1. WASM Build Validation
```bash
cd tools/basic_math/tool_name
cargo build --target wasm32-wasip1
```
- [ ] Build succeeds without errors
- [ ] No compilation warnings related to conditional features

#### 2. Library Mode Testing
- [ ] Add tool to `pure_validation/Cargo.toml`:
  ```toml
  tool_name_tool = { path = "../tools/basic_math/tool_name", default-features = false, features = ["library"] }
  ```
- [ ] Add comprehensive tests to `pure_validation/src/main.rs`
- [ ] Run tests: `cd pure_validation && cargo run`
- [ ] All tests must pass (minimum 3-5 test cases covering happy path and error conditions)

#### 3. Individual Mode Testing  
- [ ] Add HTTP endpoint tests to `http_validation.sh`
- [ ] Test same scenarios as library mode
- [ ] Run tests: `./http_validation.sh`
- [ ] All HTTP requests return 200 with expected JSON content

#### 4. Integration Testing (Pattern B Only)
- [ ] Verify proper dependency integration
- [ ] Test both HTTP and pure dependency calls
- [ ] Validate error propagation from dependencies

## Testing Protocol

### Library Mode Test Template

**CRITICAL: Type Conflict Resolution**

When adding multiple tools to `pure_validation/Cargo.toml`, tools may export the same struct names (e.g., `TwoNumberInput`). Resolve with type aliases:

```rust
// In pure_validation/src/main.rs
use tool1::{tool_function as tool1_func, TwoNumberInput as Tool1Input};
use tool2::{tool_function as tool2_func, TwoNumberInput as Tool2Input};
```

**Test Template:**
```rust
// Test basic functionality
println!("--- Test: Basic Operation ---");
let input = ToolInput { a: 10.0, b: 2.0 };

match tool_name(input) {
    Ok(result) => {
        println!("✅ Success: result = {}", result.result);
        if result.result == expected_value {
            tests_passed += 1;
        } else {
            println!("❌ Assertion failed");
            tests_failed += 1;
        }
    },
    Err(e) => {
        println!("❌ Error: {}", e);
        tests_failed += 1;
    }
}

// Test error conditions  
println!("--- Test: Error Handling ---");
let input = ToolInput { a: f64::NAN, b: 2.0 };

match tool_name(input) {
    Ok(_) => {
        println!("❌ Error: Should have failed with invalid input");
        tests_failed += 1;
    },
    Err(e) => {
        println!("✅ Success: Correctly rejected invalid input");
        if e.contains("invalid") || e.contains("NaN") {
            tests_passed += 1;
        } else {
            tests_failed += 1;
        }
    }
}
```

### HTTP Test Template
```bash
# Test basic functionality
echo "--- Test: Basic Operation ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/tool-name -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": 2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"

# Verify response contains expected JSON structure
# Expected: {"content":[{"type":"text","text":"{\"result\":5.0,\"operation\":\"operation_name\",\"inputs\":[10.0,2.0]}"}]}
```

## Common Pitfalls and Solutions

### 1. Feature Flag Conflicts
**Problem**: Building with `--features library` still enables default `individual` feature.
**Solution**: Use `--no-default-features --features library` for library builds.
**Prevention**: Always test both build modes separately.

### 2. Conditional Compilation in Wrong Places  
**Problem**: Adding `#[cfg]` directives in `logic.rs` breaks business logic isolation.
**Solution**: Keep ALL conditional compilation in `lib.rs` only.
**Prevention**: `logic.rs` should compile identically in all modes.

### 3. Function Naming Conflicts
**Problem**: Multiple function names (`tool_name`, `tool_name_pure`, `tool_name_handler`).
**Solution**: Single function name with conditional exports.
**Prevention**: Always use the same function name for both modes.

### 4. Individual Parameters vs Structs
**Problem**: Functions taking `(a: f64, b: f64)` instead of struct.
**Solution**: Always use struct inputs like `ToolInput { a: f64, b: f64 }`.
**Prevention**: Review all function signatures before implementation.

### 5. Incomplete Testing
**Problem**: Only testing one mode or missing edge cases.
**Solution**: Follow mandatory testing checklist with all scenarios.
**Prevention**: Don't consider work done until all three test types pass.

### 6. Type Conflicts in Testing
**Problem**: Multiple tools export same struct names (e.g., `TwoNumberInput`).
**Solution**: Use type aliases in pure_validation imports: `TwoNumberInput as ToolInput`.
**Prevention**: Always use unique aliases when adding tools to testing projects.

### 7. HTTP Response Parsing (Pattern B)
**Problem**: Incorrect parsing of ToolResponseWrapper format.
**Solution**: Use exact parsing structure with ContentItem array.
**Prevention**: Test HTTP calls thoroughly with actual responses.

## Validation Criteria

### Tool Migration Complete When:
- [ ] WASM build succeeds: `cargo build --target wasm32-wasip1`
- [ ] Library tests pass: All pure_validation tests for the tool pass
- [ ] HTTP tests pass: All http_validation endpoint tests pass  
- [ ] Code follows pattern: Conditional exports, struct parameters, isolated business logic
- [ ] No regressions: Existing functionality preserved

### Documentation Complete When:
- [ ] All basic_math tools successfully migrated using this guide
- [ ] Guide tested through actual migration process
- [ ] Common issues documented with solutions
- [ ] Other sessions can follow guide mechanically without errors

## Migration Order

Recommended order for basic_math tools:

1. **add** - Simplest arithmetic, test document
2. **multiply** - Similar to add, validate pattern
3. **subtract** - Another basic operation
4. **power** - Slightly more complex single operation
5. **sqrt** - Single input operation
6. **square** - Single input operation  
7. **remainder** - Potential edge cases with modulo operations
8. **modulus** - Similar to remainder, finish with edge cases

## Success Metrics

- All basic_math tools migrated to dual-mode architecture
- Single function name per tool across both modes
- All tests pass for all tools in all modes
- Migration guide is battle-tested and reliable
- Other sessions can follow guide without interpretation

---

## Appendix: Example Migration Diff

### Before (lib.rs):
```rust
#[cfg(feature = "individual")]
#[cfg_attr(not(test), tool)]
pub fn divide(input: TwoNumberInput) -> ToolResponse {
    // Individual mode implementation
}

#[cfg(feature = "library")]
pub fn divide_pure(a: f64, b: f64) -> Result<f64, String> {
    // Separate library implementation
}
```

### After (lib.rs):
```rust
// Core implementation - shared between both modes
fn divide_impl(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    let logic_input = logic::TwoNumberInput { a: input.a, b: input.b };
    logic::divide_numbers(logic_input)
}

// Library mode: Primary export for pure function usage
#[cfg(feature = "library")]
pub fn divide(input: TwoNumberInput) -> Result<ArithmeticResult, String> {
    divide_impl(input)
}

// Individual mode: HTTP-based tool handler
#[cfg(feature = "individual")]
#[cfg_attr(not(feature = "library"), tool)]
pub fn divide(input: TwoNumberInput) -> ToolResponse {
    match divide_impl(input) {
        Ok(result) => ToolResponse::text(serde_json::to_string(&result).unwrap()),
        Err(e) => ToolResponse::text(format!("Error: {}", e))
    }
}
```

This migration guide ensures consistent, reliable dual-mode architecture across all tools while maintaining comprehensive testing and validation standards.