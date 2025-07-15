# Core Tools Migration Context - Session Handoff

## Overview
This document captures the current state of the Core Tools migration project for seamless continuation in a new session.

## Project Summary
We are migrating a monolithic Spin-based API architecture to individual FTL-SDK tools following the Model Context Protocol (MCP) pattern. Each tool is now a separate WASM module using the FTL-SDK `#[tool]` decorator.

## Critical Project Rules (MUST FOLLOW)
1. **NEVER use curl directly** - Always use `./curl.sh` script for testing
2. **NEVER use spin commands directly** - Always use `./test_server` script for server management  
3. **ALWAYS commit major milestones immediately** with detailed commit messages
4. **Rust edition 2024** is required everywhere in the project
5. **No wasm32 target config blocks** in tool Cargo.toml files - tools only compile to WASM
6. **Component names in spin.toml** must use hyphens, not underscores, and cannot have numbers after hyphens

## Current Status: SEGMENT 8 - Math3D Function Migrations
**Progress**: 18 of 23 functions completed (78%)
**Location**: Migrating from `src/math_3d/` to `tools/math3d/`

### Completed Categories:
1. **✅ Plane Operations** (3 tools) - Commit a5d0194
   - line_plane_intersection
   - plane_plane_intersection  
   - point_plane_distance

2. **✅ Transformations** (9 tools) - Commit a2fc1a4
   - rotation_matrix (X, Y, Z axis rotations)
   - arbitrary_rotation (Rodrigues formula)
   - quaternion_from_axis_angle
   - quaternion_multiply
   - quaternion_slerp (spherical linear interpolation)
   - matrix_vector_multiply (3D matrix-vector operations)
   - coordinate_conversion (cartesian ↔ spherical ↔ cylindrical)

3. **✅ Volume Calculations** (5 tools) - Commit 89587d6
   - tetrahedron_volume (scalar triple product)
   - sphere_volume (4/3πr³ formula)
   - cylinder_volume (πr²h with base_center, axis)
   - aabb_volume (axis-aligned bounding box)
   - pyramid_volume (base area + height calculation)

4. **🔄 Primitives** (1 of 4 completed) - Commit 09e7276
   - ✅ sphere_ray_intersection (ray-sphere with entry/exit points)
   - ⏳ sphere_sphere_intersection (TODO NEXT)
   - ⏳ cylinder_ray_intersection
   - ⏳ ray_aabb_intersection (note: called aabb_ray in original)

5. **⏳ Distance Operations** (0 of 3 tools) - Not started
   - point-to-line distance
   - point-to-plane distance  
   - vector projections

## Tool Creation Step-by-Step Process

### 1. Create Directory Structure
```bash
mkdir -p tools/math3d/[tool_name]/src
```

### 2. Create Cargo.toml
```toml
[package]
name = "[tool_name]_tool"
version = "0.1.0"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
ftl-sdk = { version = "0.2.3", features = ["macros"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
schemars = "0.8"
spin-sdk = "4.0"
```

### 3. Create src/lib.rs
```rust
use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

// Define input/output types with ALL derives
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[tool]
pub fn tool_name(input: InputType) -> Result<OutputType, String> {
    // Implementation from src/math_3d/[source_file].rs
    // Convert ErrorResponse to String errors
}
```

### 4. Update Main Cargo.toml
Add tool to the `exclude` array:
```toml
exclude = [..., "tools/math3d/[tool_name]"]
```

### 5. Update spin.toml
Add to `tool_components`:
```toml
tool_components = { default = "...,tool-name-with-hyphens" }
```

Add trigger and component:
```toml
[[trigger.http]]
route = "/tool-name-with-hyphens"
component = "tool-name-with-hyphens"

[component.tool-name-with-hyphens]
source = "tools/math3d/[tool_name]/target/wasm32-wasip1/release/[tool_name]_tool.wasm"
allowed_outbound_hosts = []
[component.tool-name-with-hyphens.build]
command = "cargo build --target wasm32-wasip1 --release"
workdir = "tools/math3d/[tool_name]"
watch = ["tools/math3d/[tool_name]/src/**/*.rs", "tools/math3d/[tool_name]/Cargo.toml"]
```

### 6. Build Tool
```bash
cd tools/math3d/[tool_name]
cargo build --target wasm32-wasip1 --release
cd ../../..
```

### 7. Update curl.sh
Add test cases for:
- Success case (normal operation)
- Edge case (e.g., ray misses sphere)
- Error case (e.g., negative radius)

### 8. Test and Commit
```bash
./test_server restart
./curl.sh
git add [files]
git commit -m "Descriptive message following pattern"
```

## Next Tool to Implement: sphere_sphere_intersection

### Source Location
`src/math_3d/primitives.rs` - function `handle_sphere_sphere_intersection`

### Expected Functionality
- Detects if two spheres intersect
- Returns intersection type: separate, one_inside_other, external_tangent, internal_tangent, intersecting
- For intersecting spheres, calculates the intersection circle

### Test Cases to Add
1. Two separate spheres
2. Two intersecting spheres  
3. One sphere inside another
4. Two spheres just touching (tangent)
5. Error case: negative radius

## Important File Locations
- **Source files**: `/Users/coreyryan/data/mashh/core-tools/src/math_3d/`
- **New tools**: `/Users/coreyryan/data/mashh/core-tools/tools/math3d/`
- **Test script**: `/Users/coreyryan/data/mashh/core-tools/curl.sh`
- **Server script**: `/Users/coreyryan/data/mashh/core-tools/test_server`
- **Progress tracking**: `/Users/coreyryan/data/mashh/core-tools/optimized-segments.md`
- **Project rules**: `/Users/coreyryan/data/mashh/core-tools/CLAUDE.md`

## Recent Commit History
```
09e7276 Add sphere_ray_intersection tool - First Math3D primitive operation
89587d6 Complete Math3D volume calculations - All 5 volume calculation tools migrated
a2fc1a4 Complete Math3D transformations - All 9 transformation tools migrated
f1b5e99 Add quaternion_from_axis_angle and quaternion_multiply tools
ddc27a1 Add arbitrary rotation tool and update project to Rust 2024
674835f Add rotation matrix tool to Math3D transformations
a5d0194 feat: Migrate 3D plane operations from old architecture
```

## Current Git Status
- Branch: feat/core-tools
- 10 commits ahead of origin/feat/core-tools
- Main branch for PRs: main

## Common Issues and Solutions
1. **Port 3000 in use**: `lsof -ti:3000 | xargs kill -9` before restart
2. **TOML parsing errors**: Check component names for numbers after hyphens
3. **Build failures**: Ensure Rust edition 2024 in all Cargo.toml files
4. **Test failures**: Verify JSON structure matches exactly (especially for complex types like cylinder)

## Session Handoff Instructions
1. Read this file and CLAUDE.md for project rules
2. Check `git status` and recent commits
3. Continue with sphere_sphere_intersection tool
4. Follow the 8-step process above exactly
5. Test thoroughly with curl.sh
6. Commit after each tool completion
7. Update memory and segments after major milestones

The project is progressing smoothly. Each tool maintains mathematical accuracy while following the established FTL-SDK pattern. Good luck!