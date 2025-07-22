# Core Tools - LLM Augmentation API Suite

[![Build Status](https://github.com/your-org/core-tools/workflows/Build%20and%20Test/badge.svg)](https://github.com/your-org/core-tools/actions)
[![Tests](https://github.com/your-org/core-tools/workflows/Tests/badge.svg)](https://github.com/your-org/core-tools/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://rustup.rs/)

Production-ready computational APIs that fill critical gaps in Large Language Model capabilities for mathematical computation, spatial analysis, and data processing.

## 🌟 Overview

**Solve Real Problems with Precision**

Core Tools provides battle-tested APIs for tasks LLMs struggle with:

- **📍 GPS & Mapping**: Calculate distances, bearings, geofencing, and spatial relationships with professional-grade accuracy
- **🧮 3D Mathematics**: Vector operations, geometric intersections, and coordinate transformations for engineering applications  
- **📊 Statistical Analysis**: Comprehensive statistics, correlation analysis, and regression modeling
- **🔢 Mathematical Operations**: Reliable arithmetic, advanced calculations, and data processing primitives
- **🛠️ Utility Functions**: Encoding, validation, string manipulation, and data format processing

**Built for Production**: Each tool is a standalone WebAssembly microservice with standardized JSON APIs, comprehensive error handling, and validated accuracy against reference implementations.

### 🏗️ Modern Architecture

- **Microservice Design**: Each tool is an independent WebAssembly component with HTTP APIs
- **Composability**: Combine simple tools to build complex operations via HTTP composition
- **Performance**: WebAssembly provides near-native speed with sub-millisecond response times
- **Reliability**: Comprehensive error handling and validation in every tool

## 📁 Project Structure

```
core-tools/
├── tools/                           # Individual microservice tools
│   ├── geospatial/                  # GPS, mapping, spatial analysis
│   │   ├── distance/                # Haversine distance calculations
│   │   ├── bearing/                 # Bearing/heading calculations
│   │   ├── polygon_area/            # Polygon area calculations
│   │   ├── point_in_polygon/        # Geofencing operations
│   │   ├── coordinate_conversion/   # DMS ↔ Decimal conversion
│   │   ├── buffer_polygon/          # Buffer zone creation
│   │   ├── proximity_search/        # Proximity detection
│   │   └── polygon_simplification/  # Polygon simplification
│   ├── math3d/                      # 3D mathematics operations
│   │   ├── dot_product/             # Vector dot product
│   │   ├── cross_product/           # Vector cross product
│   │   ├── vector_magnitude/        # Vector magnitude calculation
│   │   ├── vector_angle/            # Angle between vectors
│   │   ├── line_intersection/       # 3D line intersection
│   │   ├── line_plane_intersection/ # Line-plane intersection
│   │   ├── plane_plane_intersection/# Plane-plane intersection
│   │   ├── rotation_matrix/         # 3D rotation matrices
│   │   ├── quaternion_*/            # Quaternion operations
│   │   ├── *_volume/                # 3D volume calculations
│   │   └── coordinate_conversion/   # 3D coordinate systems
│   ├── statistics/                  # Statistical analysis
│   │   ├── descriptive_statistics/  # Mean, std dev, skewness, etc.
│   │   ├── summary_statistics/      # Summary stats (5-number summary)
│   │   ├── pearson_correlation/     # Pearson correlation
│   │   ├── spearman_correlation/    # Spearman rank correlation
│   │   ├── correlation_matrix/      # Multi-variable correlation
│   │   ├── linear_regression/       # Linear regression analysis
│   │   ├── polynomial_regression/   # Polynomial regression
│   │   ├── histogram/               # Data distribution analysis
│   │   └── test_normality/          # Normality testing
│   └── basic_math/                  # Fundamental operations
│       ├── add/                     # Addition
│       ├── multiply/                # Multiplication
│       ├── square/                  # Square calculation
│       ├── sqrt/                    # Square root
│       ├── pythagorean/             # Pythagorean theorem
│       └── distance_2d/             # 2D distance calculation
├── spin.toml                        # Spin framework configuration
├── curl.sh                          # Testing script (use this, not curl directly)
├── test_server                      # Server management script
└── docs/                            # Category-specific documentation
```

### Technology Stack
- **Framework**: [Spin](https://spin.fermyon.dev/) (WebAssembly serverless)
- **Language**: Rust with FTL SDK
- **Architecture**: Individual microservice tools (1 tool = 1 WASM component)
- **API**: RESTful JSON with standardized error handling
- **Build**: Each tool builds independently to WebAssembly

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Spin CLI](https://spin.fermyon.dev/quickstart/) (optional for development)
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

## 🛠️ Tool Categories

### 📍 Geospatial & Mapping
GPS calculations, spatial analysis, geofencing, coordinate conversion, polygon operations

### 🧮 3D Mathematics
Vector operations, geometric intersections, transformations, volume calculations, ray tracing

### 📊 Statistical Analysis  
Descriptive statistics, correlation analysis, regression modeling, distribution testing

### 🔢 Mathematical Operations
Arithmetic, advanced calculations, trigonometry, data processing primitives

### 🔧 Utility Functions
Encoding/decoding, validation, string manipulation, data format processing, cryptography

## 🎯 Real-World Examples

### Fleet Management: Delivery Route Optimization
```bash
# Calculate distances between delivery stops
POST /distance
{
  "lat1": 40.7128, "lon1": -74.0060,  # NYC warehouse
  "lat2": 40.7831, "lon2": -73.9712   # Customer location
}
# Response: {"distance_km": 8.97, "distance_miles": 5.57}

# Check if delivery is within service area
POST /point-in-polygon
{
  "point": {"lat": 40.7831, "lon": -73.9712},
  "polygon": [/* service area coordinates */]
}
```

### Engineering: 3D CAD Calculations
```bash
# Calculate angle between structural beams
POST /vector-angle
{
  "vector1": {"x": 10.0, "y": 0.0, "z": 5.0},
  "vector2": {"x": 8.0, "y": 6.0, "z": 0.0}
}
# Response: {"angle_degrees": 67.38, "angle_radians": 1.176}

# Find intersection point for beam connections
POST /line-intersection
{
  "line1": {"point": {"x": 0, "y": 0, "z": 0}, "direction": {"x": 1, "y": 0, "z": 0}},
  "line2": {"point": {"x": 0, "y": 1, "z": 0}, "direction": {"x": 0, "y": 0, "z": 1}}
}
```

### Data Science: Quality Control Analysis
```bash
# Analyze manufacturing measurements for quality control
POST /descriptive-statistics
{
  "data": [24.1, 24.3, 23.9, 24.2, 24.0, 24.4, 23.8, 24.1]
}
# Response: {"mean": 24.1, "std_dev": 0.19, "within_tolerance": true}

# Test if measurements follow normal distribution
POST /test-normality
{
  "data": [/* measurement data */],
  "alpha": 0.05
}
```

## 🔧 Development

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

## 📖 Documentation

### Detailed Category Documentation
- **[📍 Geospatial Tools](./docs/GEOSPATIAL.md)** - GPS calculations, geofencing, spatial analysis
- **[🧮 3D Mathematics](./docs/3D_MATHEMATICS.md)** - Vector operations, transformations, 3D geometry
- **[📊 Statistical Analysis](./docs/STATISTICS.md)** - Descriptive stats, correlation, regression

## 🎯 Use Cases

**Augment LLM Capabilities**: Fill computational gaps with precise, reliable calculations

- **Fleet & Logistics**: Route optimization, delivery zones, distance calculations
- **Engineering & CAD**: 3D modeling, structural analysis, geometric calculations  
- **Data Science**: Statistical analysis, quality control, research validation
- **Web Applications**: Form validation, encoding/decoding, data processing
- **Financial**: Risk analysis, correlation studies, mathematical modeling

## 🤝 Contributing

### Development Setup
```bash
# Clone and set up the project
git clone https://github.com/your-org/core-tools.git
cd core-tools
make dev-setup
```

### Making Changes

#### For New Tools
1. Create tool directory: `tools/[category]/[tool-name]/`
2. Follow FTL SDK patterns from existing tools
3. Implement comprehensive error handling and validation
4. Add endpoint to `spin.toml`
5. Build and test: `make build-changed && make test`

#### For Existing Tools
1. Make your changes
2. Build only affected tools: `make build-changed`
3. Test your changes: `make test`
4. Verify with project tools: `./curl.sh [tool-name] [test-data]`

### Pull Request Process
1. **Create feature branch**: `git checkout -b feature/your-feature`
2. **Make changes**: Follow the development workflow above
3. **Test thoroughly**: `make test` should pass
4. **Commit changes**: Clear, descriptive commit messages
5. **Push and create PR**: GitHub Actions will automatically test only changed tools
6. **Review process**: Automated tests + manual review
7. **Merge**: Only after all tests pass and review approval

### Automated Testing
- **PR Testing**: Only tests tools that changed in your PR
- **Parallel Builds**: Efficient CI that scales with project size
- **Container Publishing**: Automatic publishing on merge to main
- **Smart Detection**: Avoids unnecessary rebuilds

### Guidelines
- Follow established FTL SDK patterns
- Implement comprehensive error handling and validation
- Add thorough testing using `./curl.sh`
- Update documentation as needed
- Maintain API consistency across tools
- Use `make build-changed` for fast iteration during development

## 📄 License

This project is designed to enhance Large Language Model capabilities with precise mathematical, spatial, and statistical analysis tools.

---

*Built with Rust, FTL SDK, and Spin for high-performance LLM augmentation.*