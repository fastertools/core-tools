# Vector Angle Tool

Calculate the angle between two 3D vectors with geometric relationship detection.

## Overview

This tool computes the angle between two 3D vectors in both radians and degrees, including cosine values and detection of perpendicular or parallel relationships.

## API Endpoint

```
POST /vector-angle
```

## Input

```json
{
  "vector1": {
    "x": 1.0,
    "y": 0.0,
    "z": 0.0
  },
  "vector2": {
    "x": 1.0,
    "y": 1.0,
    "z": 0.0
  }
}
```

## Output

```json
{
  "angle_radians": 0.7854,
  "angle_degrees": 45.0,
  "cos_angle": 0.7071,
  "vector1_magnitude": 1.0,
  "vector2_magnitude": 1.4142,
  "is_perpendicular": false,
  "is_parallel": false
}
```

## Example Usage

```bash
# Calculate angle between two vectors
echo '{"vector1": {"x": 1.0, "y": 0.0, "z": 0.0}, "vector2": {"x": 0.0, "y": 1.0, "z": 0.0}}' | \
  ./curl.sh http://127.0.0.1:3000/vector-angle
```

## Technical Details

- **Algorithm**: Inverse cosine of normalized dot product
- **Range**: 0 to π radians (0 to 180 degrees)
- **Numerical Stability**: Clamping to handle floating-point precision
- **Perpendicular Detection**: Angle within 1e-10 of π/2 radians
- **Parallel Detection**: Angle within 1e-10 of 0 or π radians
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Lighting angle calculations and surface orientation
- **Physics Simulations**: Force vector analysis and momentum calculations
- **3D Modeling**: Geometric relationship detection and alignment
- **Game Development**: AI vision cones and directional mechanics