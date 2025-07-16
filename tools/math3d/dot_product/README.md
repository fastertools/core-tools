# Dot Product Tool

Calculate the dot product of two 3D vectors along with geometric relationships.

## Overview

This tool computes the dot product of two 3D vectors and provides additional geometric analysis including angle calculations and vector relationship detection (parallel/perpendicular).

## API Endpoint

```
POST /dot-product
```

## Input

```json
{
  "vector1": {
    "x": 1.0,
    "y": 2.0,
    "z": 3.0
  },
  "vector2": {
    "x": 4.0,
    "y": 5.0,
    "z": 6.0
  }
}
```

## Output

```json
{
  "dot_product": 32.0,
  "angle_radians": 0.2257,
  "angle_degrees": 12.933,
  "are_perpendicular": false,
  "are_parallel": false
}
```

## Example Usage

```bash
# Calculate dot product of two vectors
echo '{"vector1": {"x": 1.0, "y": 0.0, "z": 0.0}, "vector2": {"x": 0.0, "y": 1.0, "z": 0.0}}' | \
  ./curl.sh http://127.0.0.1:3000/dot-product
```

## Technical Details

- **Algorithm**: Standard 3D dot product (v1·v2 = x1×x2 + y1×y2 + z1×z2)
- **Angle Calculation**: Uses inverse cosine of normalized dot product
- **Parallel Detection**: Cross product magnitude < 1e-10
- **Perpendicular Detection**: Dot product absolute value < 1e-10
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Lighting calculations and surface normal analysis
- **Physics Simulations**: Force and velocity projections
- **3D Modeling**: Vector alignment and orientation detection
- **Game Development**: Collision detection and movement calculations