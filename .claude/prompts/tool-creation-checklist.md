# Core-Tools: Tool Creation Checklist

## Pre-Creation Queries
```bash
# 1. Find implementation pattern
@memory-core-tools search "FTL-SDK Tool Implementation Pattern"
@memory-core-tools search "Tool Creation Quick Reference"

# 2. Find similar tool to copy
@memory-core-tools search "[similar functionality] tool"
ls tools/[category]/
```

## Tool Creation Steps

### 1. Copy Template Tool
```bash
# Choose most similar existing tool
cp -r tools/geospatial/polygon_area tools/geospatial/new_tool_name
```

### 2. Update Cargo.toml
- [ ] Change package name (keep _tool suffix)
- [ ] Verify edition = "2024"
- [ ] Verify ftl-sdk version matches template

### 3. Create Logic Module
- [ ] Create src/logic.rs with pure Rust implementation
- [ ] Add comprehensive unit tests
- [ ] Use #[cfg(test)] for test module

### 4. Create WASM Wrapper
- [ ] Update src/lib.rs with #[tool] function
- [ ] Map between logic types and wrapper types
- [ ] Add JsonSchema derives to wrapper types

### 5. Update Project Files
- [ ] Add to main Cargo.toml exclude array
- [ ] Update spin.toml tool_components
- [ ] Add HTTP trigger with hyphenated route
- [ ] Add component configuration

### 6. Build and Test
```bash
cd tools/[category]/[tool_name]
cargo test  # Run unit tests first
cargo build --target wasm32-wasip1 --release
cd ../../..
```

### 7. Update curl.sh
- [ ] Add success case test
- [ ] Add error case test
- [ ] Keep tests minimal (detailed tests in unit tests)

### 8. Server and Integration Test
```bash
./test_server restart  # CRITICAL: Restart to load new WASM
./curl.sh  # Run all tests including new tool
```

### 9. Commit
```bash
git add .
git commit -m "Add [tool_name] tool to [category] - [brief description]

- Implements [algorithm/functionality]
- Follows FTL-SDK pattern with unit tests
- Tests passing via curl.sh

🤖 Generated with [Claude Code](https://claude.ai/code)

Co-Authored-By: Claude <noreply@anthropic.com>"
```

## Common Pitfalls
- Using wrong Cargo.toml pattern (check memory first!)
- Forgetting _tool suffix in package name  
- Not restarting server before testing
- Using curl directly instead of ./curl.sh
- Missing JsonSchema derive on types

## Quick Validation
```bash
# Check WASM built correctly
ls tools/[category]/[tool_name]/target/wasm32-wasip1/release/*.wasm

# Verify registration
grep -A5 "new-tool-name" spin.toml
```