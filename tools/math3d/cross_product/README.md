# Cross Product Tool

Calculate the cross product of two 3D vectors with geometric analysis.

## Overview

This tool computes the cross product of two 3D vectors and provides additional geometric information including magnitude, parallelogram area, and parallel vector detection.

## API Endpoint

```
POST /cross-product
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
    "x": 0.0,
    "y": 1.0,
    "z": 0.0
  }
}
```

## Output

```json
{
  "cross_product": {
    "x": 0.0,
    "y": 0.0,
    "z": 1.0
  },
  "magnitude": 1.0,
  "area_parallelogram": 1.0,
  "are_parallel": false
}
```

## Example Usage

```bash
# Calculate cross product of perpendicular unit vectors
echo '{"vector1": {"x": 1.0, "y": 0.0, "z": 0.0}, "vector2": {"x": 0.0, "y": 1.0, "z": 0.0}}' | \
  ./curl.sh http://127.0.0.1:3000/cross-product
```

## Technical Details

- **Algorithm**: Standard 3D cross product (v1 × v2 = [y1z2-z1y2, z1x2-x1z2, x1y2-y1x2])
- **Magnitude**: Euclidean norm of resulting vector
- **Parallel Detection**: Cross product magnitude < 1e-10
- **Geometric Properties**: Cross product magnitude equals parallelogram area
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Surface normal calculations and lighting
- **Physics Simulations**: Angular momentum and torque calculations
- **3D Modeling**: Perpendicular vector generation and surface analysis
- **Game Development**: Collision detection and physics engines