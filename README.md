# Core Tools - Precision Computation APIs for LLM Applications

[![Build Status](https://github.com/fastertools/core-tools/workflows/Build%20and%20Test/badge.svg)](https://github.com/fastertools/core-tools/actions)
[![Tests](https://github.com/fastertools/core-tools/workflows/Tests/badge.svg)](https://github.com/fastertools/core-tools/actions)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rustup.rs/)

A collection of high-performance WebAssembly microservices that provide precise computational capabilities for applications using Large Language Models. Built with Rust for reliability and speed.

## Why Core Tools?

Large Language Models excel at understanding and generating text, but they often struggle with precise mathematical computations. Core Tools bridges this gap by providing:

- **Accurate Calculations**: Battle-tested algorithms for geospatial, 3D math, and statistical operations
- **Microservice Architecture**: 84 independent tools that can be composed for complex workflows
- **Near-Native Performance**: WebAssembly execution with sub-millisecond response times
- **Simple Integration**: RESTful JSON APIs that work with any programming language
- **Production Ready**: Comprehensive error handling, input validation, and test coverage

## Key Features

- **🎯 Precision**: Validated accuracy for mission-critical calculations
- **⚡ Performance**: WebAssembly provides near-native execution speed
- **🔧 Composability**: Combine atomic tools to build complex operations
- **🛡️ Reliability**: Comprehensive error handling and input validation
- **📦 Easy Deployment**: Single binary with all tools included

## Project Structure

```
core-tools/
├── tools/                     # 84 computational microservices
│   ├── geospatial/           # Location & mapping (11 tools)
│   ├── math3d/               # 3D operations (20 tools)
│   ├── statistics/           # Data analysis (12 tools)
│   ├── basic_math/           # Core calculations (25 tools)
│   └── utilities/            # Helper functions (16 tools)
├── docs/                     # API documentation
├── tests/                    # Comprehensive test suite
├── spin.toml                 # WebAssembly configuration
└── Makefile                  # Build automation
```

## Technology Stack

- **Language**: Rust (for performance and reliability)
- **Runtime**: WebAssembly via [Spin Framework](https://spin.fermyon.dev/)
- **Architecture**: Microservice pattern - each tool is an independent component
- **API Design**: RESTful JSON with consistent error handling
- **SDK**: FTL SDK for tool development

## Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Spin CLI](https://spin.fermyon.dev/quickstart/)
- FTL SDK (included in tool dependencies)

### Building Tools

#### Option 1: Using Make (Recommended)
```bash
# Build all tools
make build

# Build only changed tools (faster for development)
make build-changed

# Build in debug mode
make build-debug

# Clean build artifacts
make clean

# Show all available commands
make help
```

#### Option 2: Using Build Script Directly
```bash
# Build all tools in parallel
./build_all.sh build

# Build only tools that changed since main branch
./build_all.sh changed

# Build with more parallel jobs (default: 4)
./build_all.sh --jobs 8 build

# Show all available options
./build_all.sh help
```

### Running the Server
```bash
# Start the development server
./test_server

# The server will be available at http://127.0.0.1:3000
# Individual tools available at http://127.0.0.1:3000/[tool-name]

# Stop the server
./test_server stop

# Test the API (use the testing script, not curl directly)
./curl.sh
```

### Testing Individual Tools
```bash
# Test a specific geospatial tool
echo '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}' | \
  ./curl.sh http://127.0.0.1:3000/distance

# Test a 3D math operation
echo '{"vector1": {"x": 1.0, "y": 2.0, "z": 3.0}, "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}}' | \
  ./curl.sh http://127.0.0.1:3000/dot-product

# Test statistical analysis
echo '{"data": [1.5, 2.3, 3.1, 4.7, 5.2, 6.8, 7.1, 8.9, 9.4, 10.6]}' | \
  ./curl.sh http://127.0.0.1:3000/descriptive-statistics
```

## Available Tool Categories

### Geospatial & Mapping (11 tools)
Professional-grade GPS calculations, spatial analysis, geofencing, and coordinate conversions. Perfect for logistics, mapping, and location-based services.

### 3D Mathematics (20 tools)
Comprehensive vector operations, geometric intersections, transformations, and volume calculations. Essential for CAD, game development, and engineering applications.

### Statistical Analysis (12 tools)
Full suite of descriptive statistics, correlation analysis, regression modeling, and distribution testing. Ideal for data science and research applications.

### Mathematical Operations (25 tools)
Fundamental arithmetic through advanced calculations including matrix operations, trigonometry, and numerical methods.

### Utility Functions (16 tools)
Practical tools for encoding/decoding, validation, string manipulation, hashing, and data format processing.

## Real-World Examples

### Example 1: Building a Delivery Route Optimizer

```javascript
// Calculate optimal delivery routes using Core Tools APIs
async function optimizeDeliveryRoute(warehouse, deliveries) {
  const distances = [];
  
  // Calculate distance from warehouse to each delivery
  for (const delivery of deliveries) {
    const response = await fetch('http://localhost:3000/distance', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        lat1: warehouse.lat, lon1: warehouse.lon,
        lat2: delivery.lat, lon2: delivery.lon
      })
    });
    
    const result = await response.json();
    distances.push({ 
      delivery: delivery.id, 
      distance_km: result.distance_km 
    });
  }
  
  // Sort by distance for simple nearest-neighbor routing
  return distances.sort((a, b) => a.distance_km - b.distance_km);
}
```

### Example 2: Engineering Analysis for Structural Design

```python
import requests
import numpy as np

def analyze_beam_connection(beam1_vector, beam2_vector):
    """Analyze the connection between two structural beams"""
    
    # Calculate angle between beams
    angle_response = requests.post(
        'http://localhost:3000/vector-angle',
        json={
            'vector1': {'x': beam1_vector[0], 'y': beam1_vector[1], 'z': beam1_vector[2]},
            'vector2': {'x': beam2_vector[0], 'y': beam2_vector[1], 'z': beam2_vector[2]}
        }
    )
    angle_data = angle_response.json()
    
    # Check if angle is within structural limits (e.g., 45-135 degrees)
    if 45 <= angle_data['angle_degrees'] <= 135:
        return {
            'status': 'valid',
            'angle': angle_data['angle_degrees'],
            'message': 'Connection angle within acceptable range'
        }
    else:
        return {
            'status': 'warning',
            'angle': angle_data['angle_degrees'],
            'message': 'Connection angle may require additional support'
        }
```

### Example 3: Quality Control in Manufacturing

```rust
// Automated quality control using statistical analysis
async fn check_production_quality(measurements: Vec<f64>) -> QualityReport {
    let client = reqwest::Client::new();
    
    // Get comprehensive statistics
    let stats_response = client.post("http://localhost:3000/descriptive-statistics")
        .json(&serde_json::json!({ "data": measurements }))
        .send()
        .await?
        .json::<DescriptiveStats>()
        .await?;
    
    // Test for normal distribution
    let normality_response = client.post("http://localhost:3000/test-normality")
        .json(&serde_json::json!({ "data": measurements, "alpha": 0.05 }))
        .send()
        .await?
        .json::<NormalityTest>()
        .await?;
    
    QualityReport {
        mean: stats_response.mean,
        std_dev: stats_response.std_dev,
        within_spec: stats_response.std_dev < 0.2, // Example specification
        normally_distributed: normality_response.is_normal,
        action_required: stats_response.std_dev > 0.2 || !normality_response.is_normal
    }
}

## Development

### Architecture Principles
1. **One Tool, One Component**: Each computational tool is a standalone WASM component
2. **Microservice Pattern**: Tools are independently deployable and scalable
3. **Standardized Interfaces**: Consistent JSON input/output across all tools
4. **Composability**: Tools can be combined for complex workflows
5. **Performance**: WebAssembly provides near-native performance

### Development Workflow

#### Setting Up Development Environment
```bash
# Set up everything needed for development
make dev-setup

# Check project statistics
make stats

# Generate documentation
make docs
```

#### Building and Testing
```bash
# Build only changed tools (fast iteration)
make build-changed

# Run tests on the built tools
make test

# Build and package for release
make package
```

### Adding New Tools
1. Create new directory in appropriate category: `tools/[category]/[tool-name]/`
2. Set up Cargo.toml with FTL SDK dependencies
3. Implement tool logic in `src/lib.rs` using `#[tool]` attribute
4. Add endpoint configuration to root `spin.toml`
5. Test using `./curl.sh`
6. Build and verify: `make build-changed`

### Testing

#### Comprehensive Test Suite
```bash
# Build all tools and validate
make build-all              # Build all 84 tools to WASM

# Unit testing
cargo test                  # Run all unit tests

# HTTP endpoint testing
./test_server              # Start development server
./curl_comprehensive.sh    # Comprehensive HTTP endpoint testing (ALL 84 tools)
./test_server stop         # Stop server

# Validation commands
make test                  # Complete validation pipeline
```

#### Testing Methodology
The project includes a **3-tier validation system**:
1. **Build Validation**: All 84 tools compile to WebAssembly without errors
2. **Unit Test Validation**: Comprehensive unit test coverage for all tools  
3. **HTTP Endpoint Validation**: End-to-end testing via HTTP requests using `curl.sh`
4. **Integration Testing**: Complex operations like `vector_analysis` composition patterns

### Continuous Integration

The project includes automated CI/CD pipelines:

- **Pull Request Testing**: Automatically tests only changed tools
- **Build and Publish**: Builds tools and publishes to GitHub Container Registry
- **Smart Change Detection**: Only rebuilds tools that have actually changed
- **Parallel Building**: Efficiently builds tools in parallel batches

#### GitHub Actions Workflows
- `.github/workflows/build-and-publish.yml` - Main build and publish pipeline
- `.github/workflows/test-pr.yml` - PR validation and testing

## Documentation

### Detailed Guides by Category
- **[Geospatial Tools Guide](./docs/GEOSPATIAL.md)** - Complete reference for GPS and spatial calculations
- **[3D Mathematics Guide](./docs/3D_MATHEMATICS.md)** - Vector operations, transformations, and geometry
- **[Statistical Analysis Guide](./docs/STATISTICS.md)** - Statistics, correlation, and regression methods

## Who Uses Core Tools?

Core Tools is designed for developers building LLM-powered applications that need reliable computational capabilities:

- **AI Application Developers**: Enhance chatbots and assistants with precise calculations
- **Engineering Teams**: Add CAD calculations and 3D math to LLM workflows  
- **Data Scientists**: Integrate statistical analysis into AI pipelines
- **Logistics Companies**: Build route optimization into conversational interfaces
- **Research Teams**: Combine LLM reasoning with accurate mathematical analysis

## Contributing

We welcome contributions! Core Tools is designed to be extended with new computational capabilities.

### Quick Start for Contributors

```bash
# 1. Fork and clone the repository
git clone https://github.com/YOUR-USERNAME/core-tools.git
cd core-tools

# 2. Set up development environment
make dev-setup

# 3. Create a new branch
git checkout -b feature/your-new-tool

# 4. Build and test
make build-changed  # Builds only modified tools
make test          # Run the test suite
```

### Adding a New Tool

1. **Choose the right category** for your tool (or propose a new one)
2. **Create the tool structure**:
   ```bash
   mkdir -p tools/[category]/[your-tool-name]
   cd tools/[category]/[your-tool-name]
   cargo init --lib
   ```

3. **Implement your tool** using the FTL SDK pattern:
   ```rust
   use ftl_sdk::tool;
   
   #[tool]
   async fn your_tool_name(input: YourInput) -> YourOutput {
       // Your implementation here
   }
   ```

4. **Add to spin.toml** to register the HTTP endpoint
5. **Test thoroughly** with comprehensive test cases
6. **Submit a PR** with a clear description of what your tool does

### Code Standards

- **Error Handling**: All tools must handle errors gracefully
- **Validation**: Validate all inputs before processing
- **Documentation**: Include clear documentation and examples
- **Testing**: Provide comprehensive test coverage
- **Performance**: Keep response times under 100ms when possible

### Review Process

All PRs go through:
1. Automated testing (only changed tools are tested)
2. Code review for quality and consistency
3. Performance validation
4. Documentation review

We aim to review PRs within 48 hours. Small, focused PRs are easier to review and merge quickly.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Spin](https://spin.fermyon.dev/) - The WebAssembly framework for building microservices
- Powered by [Rust](https://www.rust-lang.org/) - For performance and reliability
- Uses [FTL SDK](https://github.com/fastertools/ftl-sdk) - For seamless tool development

---

**Core Tools** - Precision computation APIs that make LLM applications more capable.

*Questions? Issues? Contributions? We'd love to hear from you!*