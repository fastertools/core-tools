# LLM Standard Library Implementation Plan

## Overview

This document provides a comprehensive implementation plan for the LLM Standard Library tools, following existing Core Tools patterns and conventions. The plan is based on analysis of existing tools and focuses on providing essential computational capabilities that LLMs lack.

## Core Implementation Patterns (From Existing Code)

### 1. Directory Structure
```
tools/
├── [category]/
│   └── [tool_name]/
│       ├── Cargo.toml
│       └── src/
│           ├── lib.rs
│           └── logic.rs
```

### 2. Cargo.toml Pattern
```toml
[package]
name = "[tool_name]_tool"  # Note: package name uses underscore
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
ftl-sdk = { version = "0.2.3", features = ["macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"
# Add specific dependencies here (e.g., chrono = "0.4")

[target.'cfg(target_arch = "wasm32")'.dependencies]
spin-sdk = "4.0"
```

### 3. lib.rs Pattern
```rust
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

mod logic;

#[cfg(not(test))]
use ftl_sdk::tool;

// Define input/output types with JsonSchema
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolInput {
    /// Documentation for field
    pub field: Type,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolOutput {
    pub result: Type,
}

#[cfg_attr(not(test), tool)]
pub fn tool_name(input: ToolInput) -> Result<ToolOutput, String> {
    // Call logic module
    logic::perform_operation(input)
}
```

### 4. Composite Tool Pattern (Calling Other Tools)
```rust
#[cfg_attr(not(test), tool)]
async fn composite_tool(input: Input) -> ToolResponse {
    use spin_sdk::http::{Method, Request};
    
    // Call another tool
    let request = Request::builder()
        .method(Method::Post)
        .uri("http://[tool-name].spin.internal")
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&data).unwrap().into_bytes())
        .build();
    
    let response = spin_sdk::http::send(request).await?;
    // Process response...
}
```

### 5. spin.toml Registration
```toml
[[trigger.http]]
route = "/[tool-name]"
component = "[tool-name]"

[component.[tool-name]]
source = "target/wasm32-wasip1/release/[tool_name]_tool.wasm"
allowed_outbound_hosts = []  # Or ["http://other-tool.spin.internal"] for composites
[component.[tool-name].build]
command = "cargo build --target wasm32-wasip1 --release"
workdir = "tools/[category]/[tool_name]"
watch = ["tools/[category]/[tool_name]/src/**/*.rs", "tools/[category]/[tool_name]/Cargo.toml"]
```

## New Tool Categories & Implementation Order

### Phase 1: Foundation Tools (No Dependencies)

#### 1. Missing Basic Math Tools
**Location**: `tools/basic_math/`
- `subtract` - Basic subtraction
- `divide` - Division with zero handling
- `modulo` - Modulo operation
- `power` - Exponentiation

**Dependencies**: None (pure Rust)
**Implementation Note**: Follow existing add/multiply patterns

#### 2. Identifiers & Random
**Location**: `tools/identifiers/`
- `uuid_generator` - Generate UUIDs
  - Dependencies: `uuid = "1.0"`
  - Implementation: Use uuid::Uuid::new_v4()
- `random_integer` - Generate random integers
  - Dependencies: `rand = "0.8"`
  - Implementation: Range-based generation
- `random_string` - Generate random strings
  - Dependencies: `rand = "0.8"`
  - Implementation: Alphanumeric with length

#### 3. Encoding Tools
**Location**: `tools/encoding/`
- `base64_encoder` - Encode to Base64
  - Dependencies: `base64 = "0.21"`
- `base64_decoder` - Decode from Base64
  - Dependencies: `base64 = "0.21"`
- `url_encoder` - URL encode strings
  - Dependencies: `percent-encoding = "2.3"`
- `url_decoder` - URL decode strings
  - Dependencies: `percent-encoding = "2.3"`
- `hex_encoder` - Convert to hex
  - Dependencies: `hex = "0.4"`
- `hex_decoder` - Convert from hex
  - Dependencies: `hex = "0.4"`

#### 4. String Operations
**Location**: `tools/string/`
- `string_case_converter` - Convert between cases
  - Dependencies: `heck = "0.4"` (for case conversions)
  - Input: text, target_case (snake, camel, pascal, kebab)
- `string_trimmer` - Trim/pad strings
  - Dependencies: None
  - Input: text, operation (trim, ltrim, rtrim, pad_left, pad_right)
- `string_splitter` - Split strings
  - Dependencies: None
  - Input: text, delimiter, limit (optional)

#### 5. Basic DateTime Tools
**Location**: `tools/datetime/`
- `current_datetime` - Get current time
  - Dependencies: `chrono = { version = "0.4", features = ["serde"] }`
  - Input: timezone (optional, default UTC)
  - Output: ISO timestamp, unix timestamp, components
- `timestamp_converter` - Convert timestamp formats
  - Dependencies: `chrono = "0.4"`
  - Input: timestamp, source_format, target_format
- `date_parser` - Parse date strings
  - Dependencies: `chrono = "0.4"`, `dateparser = "2.0"`
  - Input: date_string, format_hint (optional)

#### 6. Data Format Tools
**Location**: `tools/data_formats/`
- `json_parser` - Parse JSON safely
  - Dependencies: None (use serde_json)
  - Features: Error recovery, partial parsing
- `json_validator` - Validate JSON schema
  - Dependencies: `jsonschema = "0.17"`
- `json_formatter` - Format JSON
  - Dependencies: None
  - Input: json_string, pretty (bool), indent_size

### Phase 2: Composite Tools (Depend on Phase 1)

#### 1. Math Composites
**Location**: `tools/basic_math/`
- `percentage_calculator` - Uses multiply, divide
  - Calculates percentages, percentage change
- `ratio_calculator` - Uses divide, modulo
  - Simplifies ratios, calculates proportions

#### 2. DateTime Composites
**Location**: `tools/datetime/`
- `date_arithmetic` - Uses current_datetime, basic math
  - Add/subtract time periods
  - allowed_outbound_hosts: ["http://current-datetime.spin.internal", "http://add.spin.internal"]
- `timezone_converter` - Uses current_datetime, timestamp_converter
  - Convert between timezones
  - allowed_outbound_hosts: ["http://current-datetime.spin.internal", "http://timestamp-converter.spin.internal"]
- `duration_calculator` - Uses date_parser, subtract
  - Calculate time between dates
  - allowed_outbound_hosts: ["http://date-parser.spin.internal", "http://subtract.spin.internal"]

#### 3. String Composites
**Location**: `tools/string/`
- `text_normalizer` - Uses trimmer, case_converter
  - Comprehensive text cleaning
  - allowed_outbound_hosts: ["http://string-trimmer.spin.internal", "http://string-case-converter.spin.internal"]
- `slug_generator` - Uses normalizer, url_encoder
  - Generate URL-safe slugs
  - allowed_outbound_hosts: ["http://text-normalizer.spin.internal", "http://url-encoder.spin.internal"]

#### 4. Identifier Composites
**Location**: `tools/identifiers/`
- `session_id_generator` - Uses uuid_generator, current_datetime
  - Generate session IDs with timestamp
  - allowed_outbound_hosts: ["http://uuid-generator.spin.internal", "http://current-datetime.spin.internal"]
- `secure_token_generator` - Uses random_string, base64_encoder
  - Generate secure tokens
  - allowed_outbound_hosts: ["http://random-string.spin.internal", "http://base64-encoder.spin.internal"]

#### 5. Data Processing Composites
**Location**: `tools/data_formats/`
- `json_transformer` - Uses json_parser, json_formatter
  - Parse, transform, and format JSON
  - allowed_outbound_hosts: ["http://json-parser.spin.internal", "http://json-formatter.spin.internal"]
- `api_response_handler` - Uses json_parser, json_validator
  - Parse and validate API responses
  - allowed_outbound_hosts: ["http://json-parser.spin.internal", "http://json-validator.spin.internal"]

### Phase 3: Advanced Tools

#### 1. Number Formatting
**Location**: `tools/formatting/`
- `number_formatter` - Format numbers with locale
  - Dependencies: `num-format = "0.4"`
- `currency_formatter` - Format currency
  - Dependencies: `rust_decimal = "1.32"`

#### 2. Validation Tools
**Location**: `tools/validation/`
- `email_validator` - Validate emails
  - Dependencies: `email_address = "0.2"`
- `url_validator` - Validate URLs
  - Dependencies: `url = "2.4"`

#### 3. Hash Tools
**Location**: `tools/crypto/`
- `md5_hash` - Generate MD5
  - Dependencies: `md5 = "0.7"`
- `sha256_hash` - Generate SHA256
  - Dependencies: `sha2 = "0.10"`

## Implementation Guidelines

### Error Handling
```rust
pub fn tool_name(input: Input) -> Result<Output, String> {
    // Validate input
    if input.field.is_empty() {
        return Err("Field cannot be empty".to_string());
    }
    
    // Handle potential errors
    match risky_operation() {
        Ok(result) => Ok(Output { result }),
        Err(e) => Err(format!("Operation failed: {}", e))
    }
}
```

### Input Validation
- Always validate inputs before processing
- Provide clear error messages
- Include valid ranges/formats in errors

### Performance Considerations
- Target <1ms for basic operations
- Target <5ms for composite operations
- Avoid unnecessary allocations
- Use references where possible

### Testing Pattern
Create test scripts in `test_scripts/`:
```bash
#!/bin/bash
# test_datetime_tools.sh

echo "Testing current_datetime..."
curl -X POST http://localhost:3000/current-datetime \
  -H "Content-Type: application/json" \
  -d '{"timezone": "America/New_York"}'

echo "Testing date_arithmetic..."
curl -X POST http://localhost:3000/date-arithmetic \
  -H "Content-Type: application/json" \
  -d '{"date": "2025-07-16", "operation": "add", "amount": 7, "unit": "days"}'
```

## CI/CD Integration

### Update GitHub Actions
The existing CI/CD pipeline will automatically:
1. Detect new tools in PR
2. Build in parallel batches
3. Run tests
4. Publish to OCI registry

### Batch Assignment
Add new tools to build batches evenly:
- Current: 8 batches for 55 tools (~7 per batch)
- After Phase 1: ~80 tools (~10 per batch)
- May need to increase to 10 batches if memory issues

## Memory Updates Needed

### Project Memory Updates
```
1. Create entity: "LLM Standard Library Implementation"
   - Observations:
     - "ACTIVE: Implementing standard library tools for LLM computational gaps"
     - "PATTERN: Follow existing tool patterns with logic module separation"
     - "PATTERN: Composite tools use http://[tool].spin.internal for internal calls"
     - "LEARNING: Dependencies added to Cargo.toml [dependencies] section"

2. Update entity: "Core Tools Project"
   - Add observations:
     - "ACTIVE: Expanding to include LLM standard library tools"
     - "ARCHITECTURE: New categories - datetime, string, encoding, identifiers"
```

### Implementation Checklist Pattern
For each tool:
1. Create directory structure
2. Create Cargo.toml with dependencies
3. Implement lib.rs with FTL SDK pattern
4. Implement logic.rs with business logic
5. Add to spin.toml with proper route and component
6. Create test in test_scripts/
7. Test with ./test_server and ./curl.sh
8. Commit with descriptive message

## Next Steps

1. Review this plan for completeness
2. Start with Phase 1 basic tools (no dependencies on other new tools)
3. Implement missing math operations first (subtract, divide, modulo, power)
4. Then implement foundation tools in each category
5. Move to composite tools once basics are complete
6. Update documentation and memory as we progress

---

*This plan provides a complete roadmap for implementing the LLM Standard Library following Core Tools patterns and best practices.*