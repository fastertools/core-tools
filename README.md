# Core Tools - Fast WASM Functions for Real-Time LLM Computation

[![CI](https://github.com/fastertools/core-tools/workflows/CI/badge.svg)](https://github.com/fastertools/core-tools/actions/workflows/ci.yml)
[![Release](https://github.com/fastertools/core-tools/workflows/Release/badge.svg)](https://github.com/fastertools/core-tools/actions/workflows/release.yml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rustup.rs/)

A collection of lightning-fast WebAssembly functions designed for real-time computation in LLM applications. Perfect for MCP (Model Context Protocol) servers and any system requiring sub-millisecond mathematical precision alongside AI reasoning.

## Why Core Tools?

Large Language Models excel at understanding and generating text, but they often struggle with precise mathematical computations. Core Tools solves this by providing:

- **Real-Time Performance**: WASM functions with sub-millisecond response times for LLM interactions
- **MCP-Ready**: Designed to plug directly into Model Context Protocol servers
- **84 Precision Functions**: From GPS calculations to 3D math, all optimized for speed
- **Minimal Cold Starts**: WebAssembly design minimizes startup latency for responsive AI
- **Validated Accuracy**: Well-tested implementations with comprehensive error handling

## Key Features

- **⚡ Sub-millisecond Response**: WASM execution ensures real-time LLM augmentation
- **🎯 Precision Math**: Accurate calculations that LLMs can't reliably perform
- **🔌 MCP Compatible**: Ready to integrate with Model Context Protocol servers
- **🚀 Zero Setup**: Single binary deployment with all 84 functions included
- **🛡️ Error Handling**: Comprehensive validation and error messages for reliable operation

## Project Structure

```
core-tools/
├── tools/                     # 84 WASM computation functions
│   ├── geospatial/           # GPS & mapping (11 functions)
│   ├── math3d/               # 3D operations (20 functions)
│   ├── statistics/           # Data analysis (12 functions)
│   ├── basic_math/           # Core math (25 functions)
│   └── utilities/            # Helpers (16 functions)
├── docs/                     # API documentation
├── tests/                    # Test suite
├── spin.toml                 # WASM runtime config
└── Makefile                  # Build automation
```

## Technology Stack

- **Language**: Rust (compiled to WebAssembly for maximum speed)
- **Runtime**: [Spin Framework](https://spin.fermyon.dev/) (optimized WASM runtime)
- **Architecture**: Individual WASM functions with HTTP endpoints
- **Integration**: JSON APIs designed for LLM/MCP server integration
- **Performance**: Sub-millisecond execution for real-time AI applications

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

## Available Functions

### Geospatial & Mapping
Instant GPS calculations, spatial analysis, and geofencing - all with sub-millisecond response for real-time LLM interactions with location data.

### 3D Mathematics
Vector operations, geometric intersections, and transformations that execute faster than an LLM can generate the next token.

### Statistical Analysis
Real-time statistics, correlations, and regression analysis to augment LLM data interpretation with precise calculations.

### Mathematical Operations
Core arithmetic through advanced math - providing the computational precision LLMs lack, instantly.

### Utility Functions
Fast encoding, validation, and data processing functions to handle formats and transformations in real-time.

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
```

## Development

### Architecture Principles
1. **One Function, One Purpose**: Each tool is a focused WASM function for a specific calculation
2. **Real-Time First**: Optimized for sub-millisecond response in LLM conversations
3. **MCP-Ready Design**: JSON interfaces compatible with Model Context Protocol
4. **Composable Functions**: Chain simple functions to build complex computations
5. **Low Latency**: WASM execution minimizes overhead for fast response times

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

### Adding New Functions
1. Create directory for your function: `tools/[category]/[function-name]/`
2. Set up Cargo.toml with FTL SDK dependencies
3. Implement the WASM function in `src/lib.rs` using `#[tool]` attribute
4. Register HTTP endpoint in `spin.toml`
5. Test with `./curl.sh` for sub-millisecond response
6. Build and verify: `make build-changed`

#### Testing Methodology
The project includes a **3-tier validation system**:
1. **Build Validation**: All 84 tools compile to WebAssembly without errors
2. **Unit Test Validation**: Comprehensive unit test coverage for all tools  
3. **HTTP Endpoint Validation**: End-to-end testing via HTTP requests using `curl.sh`
4. **Integration Testing**: Complex operations like `vector_analysis` composition patterns

## Documentation

### Detailed Guides by Category
- **[Geospatial Tools Guide](./docs/GEOSPATIAL.md)** - Complete reference for GPS and spatial calculations
- **[3D Mathematics Guide](./docs/3D_MATHEMATICS.md)** - Vector operations, transformations, and geometry
- **[Statistical Analysis Guide](./docs/STATISTICS.md)** - Statistics, correlation, and regression methods

## Perfect For

### MCP Server Developers
Plug these functions directly into your Model Context Protocol server for instant mathematical capabilities in your AI applications.

### LLM Application Builders  
- **Conversational AI**: Add real-time calculations to chatbots with minimal latency
- **AI Assistants**: Augment reasoning with precise computational results
- **Voice Interfaces**: Sub-millisecond math for responsive voice applications

### Real-Time Systems
- **Live Data Analysis**: Statistical calculations that keep pace with streaming data
- **Interactive 3D**: Instant geometric computations for AR/VR with AI
- **Location Services**: GPS calculations fast enough for real-time navigation

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

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Spin](https://spin.fermyon.dev/) - The WebAssembly runtime for instant function execution
- Powered by [Rust](https://www.rust-lang.org/) - Compiled to WASM for maximum performance
- Uses [FTL SDK](https://github.com/fastertools/ftl-sdk) - For rapid function development
- Designed for [MCP](https://modelcontextprotocol.io/) - Ready for Model Context Protocol integration

---
