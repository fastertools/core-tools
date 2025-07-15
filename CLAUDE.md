# LLM Augmentation Tooling Project

This project is a container for building tools and functionality to augment LLMs (Large Language Models) by providing capabilities that cover gaps in their abilities.

## Project Overview

- **Purpose**: Develop tooling to enhance LLM capabilities
- **Current Status**: Initial development phase - building tools while sorting hosting/deployment
- **Architecture**: Each tool follows a standard call pattern for easy integration

## Design Principles

1. **Standardized Interface**: All tools implement a consistent call pattern
2. **Modularity**: Tools can be easily dropped into the system once high-level architecture is finalized
3. **Gap Coverage**: Focus on functionality that addresses specific LLM limitations

## Development Notes

- This project serves as a development container during the planning phase
- Tools are being built with future integration in mind
- Standard call patterns ensure compatibility when deployment strategy is determined
- **Development Pattern**: Don't use curl directly. Add/remove curl commands as necessary to a single curl.sh file
- **Memory Techniques**:
  - Use the sequential thinking tool often when planning work

## Current Implementation Status

### Completed Tool Categories

#### **Geospatial Tools Suite** (First Major Category - Complete)
- **13+ API endpoints** implemented covering core geospatial analysis needs
- **3-tier modular architecture**: geospatial/, coordinate_utils/, geofencing/
- **Production-ready**: Comprehensive error handling, validation, and testing completed

#### **3D Mathematics Suite** (Second Major Category - Complete)
- **13+ API endpoints** implemented for comprehensive 3D mathematical operations
- **5-module architecture**: math_3d/ with vector_ops.rs, line_intersection.rs, plane_operations.rs, transformations.rs, volume_calculations.rs
- **Vector Operations**: Dot product, cross product, magnitude, angle calculations
- **Line Intersection**: 3D line-line intersection detection (intersecting, parallel, skew, coincident)
- **Advanced Features**: Line-plane intersection, plane-plane intersection, point-plane distance
- **3D Transformations**: Rotation matrices (X/Y/Z axis, arbitrary axis), quaternions, coordinate conversions
- **Volume Calculations**: Tetrahedron, sphere, cylinder, AABB, pyramid, convex hull volumes

### Technology Stack Established
- **Framework**: Spin (WebAssembly serverless) - provides fast, secure execution
- **Language**: Rust - chosen for performance and memory safety
- **API**: RESTful JSON with standardized error responses
- **Build**: `spin build` / **Run**: `spin up --listen 127.0.0.1:3000`

### Key Architectural Decisions
1. **Modular File Structure**: Each tool category in separate folder (geospatial/, geofencing/, etc.)
2. **Individual Tool Files**: Each specific tool in its own .rs file for maintainability
3. **Shared Utilities**: Common validation and error handling in common.rs
4. **Consistent Patterns**: All endpoints follow same input validation → processing → response pattern

### Performance Benchmarks Achieved
- **Distance Calculations**: 99.8% accuracy using Haversine formula
- **Geofencing**: 200K-500K operations/second for point-in-polygon checks
- **API Response**: Sub-millisecond for simple operations, <100ms for complex batch processing
- **Accuracy**: Meter-level precision for all spatial calculations

### API Endpoint Categories Implemented
1. **Core Geospatial** (`/distance`, `/bearing`, `/polygon/area`)
2. **Coordinate Utils** (`/convert/*`, `/validate`) 
3. **Geofencing** (`/geofence/*`, `/buffer/*`, `/proximity/*`)
4. **3D Vector Operations** (`/3d/dot-product`, `/3d/cross-product`, `/3d/vector-magnitude`, `/3d/vector-angle`)
5. **3D Line Operations** (`/3d/line-intersection`, `/3d/segment-intersection`, `/3d/multi-line-intersection`)
6. **3D Plane Operations** (`/3d/line-plane`, `/3d/plane-plane`, `/3d/point-plane-distance`)
7. **3D Transformations** (`/3d/rotation-matrix`, `/3d/rotation-arbitrary`, `/3d/quaternion-*`, `/3d/matrix-vector`, `/3d/coordinate-convert`)
8. **3D Volume Calculations** (`/3d/volume/tetrahedron`, `/3d/volume/sphere`, `/3d/volume/cylinder`, `/3d/volume/aabb`, `/3d/volume/pyramid`, `/3d/volume/convex-hull`)

### Testing Commands Reference

**IMPORTANT: NEVER use curl directly. Always use the curl.sh script for all API testing.**
**IMPORTANT: NEVER use spin directly. Always use the test_server script for server management.**

```bash
# Build and start test server (DO NOT use spin build/up directly)
./test_server

# Run all tests (DO NOT use curl directly - use this script)
./curl.sh

# Stop test server (DO NOT use pkill spin directly)
./test_server stop

# Restart test server
./test_server restart

# View server logs
tail -f spin_*.log
```

All curl commands are centralized in curl.sh. To add new tests, edit curl.sh - never run curl commands directly.
All server management is done via test_server script. NEVER run spin commands directly.

### Development Patterns Established
- **New Tool Creation**: Add to appropriate folder, implement standard input/output structs with serde
- **Coordinate Validation**: Use `common::validate_coordinates()` for all GPS inputs
- **Error Handling**: Return `ErrorResponse` struct with descriptive messages
- **Route Addition**: Add endpoint match case in lib.rs `handle_tool()` function
- **Testing**: NEVER use curl directly - always use ./curl.sh script for all API testing
- **Server Management**: NEVER use spin directly - always use ./test_server script for build/start/stop operations
- **Parallel Development**: Use git worktrees for implementing multiple features simultaneously
- **Mathematical Accuracy**: Validate all 3D operations against known mathematical results
- **API Consistency**: Maintain consistent input/output patterns across all endpoints

### Recently Completed - 3D Mathematics Extension
**Successfully Implemented Using Git Worktree Parallel Development**:

✅ **3D Transformations** (`/3d/transform`) - **COMPLETED**
   - ✅ Rotation matrices around X, Y, Z axes or arbitrary axes
   - ✅ Quaternion operations (creation, multiplication, rotation, SLERP interpolation)
   - ✅ Coordinate conversions (Cartesian ↔ Spherical ↔ Cylindrical)
   - ✅ Matrix operations (3x3 and 4x4 matrix multiplication, inversion, determinants)
   - ✅ Matrix-vector transformations
   - **Use Cases**: 3D graphics, robotics, animation, CAD transformations

✅ **3D Volume Calculations** (`/3d/volume`) - **COMPLETED**
   - ✅ Tetrahedron volume from 4 points (scalar triple product)
   - ✅ Convex hull volume using triangulation
   - ✅ 3D bounding box (AABB) volume calculations
   - ✅ Sphere and cylinder volume calculations
   - ✅ Pyramid volume calculations
   - **Use Cases**: CAD, manufacturing, physics simulations, 3D modeling

### Immediate Next Tools - 3D Mathematics Extension
**Priority Order** (most to least immediate):

1. **3D Distance Operations** (`/3d/distance`)
   - Point-to-line distance (extend current line intersection work)
   - Point-to-plane distance (expose existing implementation)
   - 3D projections (orthogonal and perspective) onto planes
   - Vector projections (scalar and vector projections)
   - **Use Cases**: Computer graphics, collision detection, proximity analysis

2. **3D Geometric Primitives** (`/3d/primitives`)
   - Sphere operations (sphere-line, sphere-sphere, sphere-plane intersection)
   - Cylinder operations (line-cylinder, cylinder-cylinder intersection)
   - 3D ray operations (ray-sphere, ray-cylinder, ray-box intersections)
   - **Use Cases**: Ray tracing, collision detection, 3D picking, game engines

### Development Achievements
- **Total API Endpoints**: 25+ endpoints across geospatial and 3D mathematics
- **Parallel Development Success**: Successfully used git worktrees to implement two major feature sets simultaneously
- **Mathematical Accuracy**: All 3D operations validated against reference implementations
- **API Consistency**: Maintained consistent patterns across all new endpoints
- **Production Ready**: All features include comprehensive error handling and validation

### Future Tool Categories
Comprehensive roadmap documented in `TOOL_IDEAS.md`:
- **3D Distance Operations**: Point-to-line, point-to-plane, projection operations
- **3D Geometric Primitives**: Sphere, cylinder, ray intersection algorithms
- **Data Processing**: CSV/JSON parsing, statistical analysis, array operations
- **Time/Date**: Timezone conversions, calendar operations, duration parsing
- **Text Analysis**: Advanced tokenization, string algorithms, pattern matching
- **Network/Web**: URL operations, data validation, encoding/decoding

## Development Reminders

**CRITICAL: NEVER use curl or spin commands directly. Always use scripts:**
- For API testing: Use ./curl.sh (NEVER use curl directly)
- For server management: Use ./test_server (NEVER use spin build/up/down directly)

**CRITICAL: ALWAYS establish operational orientation:**
- Verify location: `pwd` should show `/Users/coreyryan/data/mashh/core-tools`
- Verify tools exist: `curl.sh`, `test_server`, `spin.toml`
- Never run operational commands from subdirectories

**CRITICAL: ALWAYS commit major milestones immediately:**
- When completing segments or major tool groups, commit with detailed messages
- Include test evidence and completion status in commit messages
- Use 🤖 Generated with [Claude Code] tag and Co-Authored-By: Claude
- Update memory and segments files BEFORE committing
- Don't forget to commit - major work should be preserved in git history
```