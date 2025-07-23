# Test Coverage Analysis

This document describes how to use and understand the test coverage analysis tools for the Core Tools project.

## Overview

The project uses `cargo-llvm-cov` for comprehensive test coverage analysis, providing source-based coverage tracking at AST-level precision. This enables detailed insight into which parts of the codebase are exercised by tests.

## Quick Start

### Generate HTML Coverage Report
```bash
cargo llvm-cov --html
```

This generates an interactive HTML report at `target/llvm-cov/html/index.html` showing:
- Line-by-line coverage visualization
- Function and region coverage statistics
- Interactive source code browsing

### Generate Summary Report
```bash
cargo llvm-cov --summary-only
```

Shows a detailed table with coverage statistics for each file.

### Generate LCOV Report (for CI)
```bash
cargo llvm-cov --lcov --output-path target/llvm-cov/coverage.lcov
```

Creates an LCOV format report compatible with CI services like Codecov, Coveralls, and GitHub Actions.

## Current Coverage Status

The Core Tools project demonstrates excellent test coverage with comprehensive unit tests across all 84 tools:

### Key Metrics
- **Logic Module Coverage**: Near 100% for most computational tools
- **Total Test Cases**: 1000+ individual test cases
- **Error Handling**: Comprehensive validation testing (NaN, infinite values, edge cases)
- **Mathematical Accuracy**: Precision testing for floating-point operations

### Tool Categories with High Coverage
- **Basic Math**: 99%+ coverage across all arithmetic operations
- **Geospatial**: Excellent coverage for GPS calculations and spatial analysis
- **3D Mathematics**: Comprehensive testing of vector operations and geometric calculations
- **Statistics**: Strong coverage for statistical analysis and correlation functions
- **Encoding/Decoding**: Complete testing of Base64, hex, and URL encoding

### Areas for Improvement
- **FTL SDK Wrapper Code**: Currently shows 0% coverage (expected - these are interface layers)
- **Vector Analysis Composite Tool**: Has some uncovered logic paths
- **Error Handling Edge Cases**: Some specific validation scenarios need additional tests

## Understanding Coverage Reports

### HTML Report Navigation
1. Open `target/llvm-cov/html/index.html` in a browser
2. Click on any file to see line-by-line coverage
3. **Green lines**: Covered by tests
4. **Red lines**: Not covered by tests
5. **Yellow lines**: Partially covered

### Coverage Types
- **Line Coverage**: Percentage of executable lines covered
- **Function Coverage**: Percentage of functions called by tests
- **Region Coverage**: LLVM's fine-grained coverage regions
- **Branch Coverage**: Coverage of conditional branches (when available)

## Development Workflow

### Before Writing Tests
```bash
# Generate baseline to understand current structure
cargo llvm-cov --html
```

### During Development
```bash
# Quick summary for specific changes
cargo llvm-cov --summary-only

# Focus on specific tool
cargo test -p tool_name && cargo llvm-cov --summary-only
```

### CI Integration
```bash
# Generate LCOV report for CI
cargo llvm-cov --lcov --output-path coverage.lcov
```

## Best Practices

### Test Strategy
1. **Start with Core Logic**: Focus on computational algorithms first
2. **Edge Case Testing**: Test NaN, infinite, boundary values
3. **Error Path Coverage**: Ensure validation errors are tested
4. **Precision Testing**: Verify mathematical accuracy

### Coverage Goals
- **Logic Modules**: Target 95%+ coverage
- **Interface Layers**: Coverage not critical (thin wrappers)
- **Error Handling**: 100% coverage of validation paths
- **Integration Points**: Test tool composition patterns

### Common Patterns in Core Tools
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_functionality() {
        // Test core logic path
    }
    
    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
    }
    
    #[test]
    fn test_error_conditions() {
        // Test validation and error handling
    }
    
    #[test]
    fn test_precision() {
        // Test mathematical accuracy
    }
}
```

## Performance Notes

- **Compilation Overhead**: ~4-5 seconds vs 0.3 seconds normal compilation
- **Build Separation**: Uses `target/llvm-cov-target/` to avoid contaminating normal builds
- **Profile Storage**: Coverage data stored as `.profraw` files during test execution

## Tool Configuration

### .gitignore Coverage
The `target/` directory is already ignored, which includes all coverage artifacts:
- `target/llvm-cov-target/` - Instrumented builds
- `target/llvm-cov/` - Coverage reports
- `*.profraw` files - Runtime coverage data

### CI Integration Example
```yaml
- name: Generate Coverage
  run: cargo llvm-cov --lcov --output-path coverage.lcov

- name: Upload to Codecov
  uses: codecov/codecov-action@v3
  with:
    file: coverage.lcov
```

## Troubleshooting

### Common Issues
1. **Missing llvm-tools**: Run `rustup component add llvm-tools-preview`
2. **Permission Errors**: Ensure write access to `target/` directory
3. **Memory Usage**: Large codebases may need increased memory limits

### Debugging Coverage
```bash
# Verbose output for debugging
LLVM_PROFILE_FILE=app-%p-%m.profraw cargo test

# Manual profile processing
llvm-profdata merge -sparse *.profraw -o merged.profdata
llvm-cov show target/debug/deps/tool_name-* -instr-profile=merged.profdata
```

## Resources

- [LLVM Coverage Documentation](https://llvm.org/docs/CommandGuide/llvm-cov.html)
- [Clang Source-Based Coverage](https://clang.llvm.org/docs/SourceBasedCodeCoverage.html)
- [cargo-llvm-cov GitHub](https://github.com/taiki-e/cargo-llvm-cov)

---

*Coverage analysis enables systematic testing improvement and quality assurance across the entire Core Tools mathematical computing platform.*