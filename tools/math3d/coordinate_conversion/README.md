# Coordinate Conversion Tool

Convert between different 3D coordinate systems: Cartesian, spherical, and cylindrical.

## Overview

This tool provides bidirectional conversion between Cartesian, spherical, and cylindrical coordinate systems, essential for 3D mathematics and physics applications.

## API Endpoint

```
POST /coordinate-conversion
```

## Input

```json
{
  "from_type": "cartesian",
  "to_type": "spherical",
  "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}
}
```

## Output

```json
{
  "original": {"x": 1.0, "y": 1.0, "z": 1.0},
  "converted": {"x": 1.732, "y": 0.7854, "z": 0.9553},
  "from_type": "cartesian",
  "to_type": "spherical"
}
```

## Example Usage

```bash
# Convert Cartesian to spherical coordinates
echo '{"from_type": "cartesian", "to_type": "spherical", "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}}' | \
  ./curl.sh http://127.0.0.1:3000/coordinate-conversion
```

## Technical Details

- **Supported Conversions**: Cartesian ↔ Spherical, Cartesian ↔ Cylindrical
- **Spherical Format**: (radius, theta, phi) in radians
- **Cylindrical Format**: (radius, theta, z) in radians
- **Angle Ranges**: θ ∈ [-π, π], φ ∈ [0, π]
- **Mathematical Convention**: Physics convention (θ=azimuth, φ=polar)
- **Performance**: Sub-millisecond response time

## Use Cases

- **Physics Simulations**: Field calculations and particle tracking
- **Computer Graphics**: Camera systems and spherical mapping
- **Engineering**: Antenna patterns and fluid dynamics
- **Scientific Computing**: Data analysis and coordinate transformations