# Tetrahedron Volume Tool

Calculate the volume of a tetrahedron defined by four 3D points.

## Overview

This tool computes the volume of a tetrahedron using the scalar triple product method, which provides exact volume calculation for any four non-coplanar points in 3D space.

## API Endpoint

```
POST /tetrahedron-volume
```

## Input

```json
{
  "point_a": {"x": 0.0, "y": 0.0, "z": 0.0},
  "point_b": {"x": 1.0, "y": 0.0, "z": 0.0},
  "point_c": {"x": 0.0, "y": 1.0, "z": 0.0},
  "point_d": {"x": 0.0, "y": 0.0, "z": 1.0}
}
```

## Output

```json
{
  "volume": 0.1667,
  "calculation_method": "Scalar triple product",
  "points": [
    {"x": 0.0, "y": 0.0, "z": 0.0},
    {"x": 1.0, "y": 0.0, "z": 0.0},
    {"x": 0.0, "y": 1.0, "z": 0.0},
    {"x": 0.0, "y": 0.0, "z": 1.0}
  ]
}
```

## Example Usage

```bash
# Calculate volume of unit tetrahedron
echo '{"point_a": {"x": 0.0, "y": 0.0, "z": 0.0}, "point_b": {"x": 1.0, "y": 0.0, "z": 0.0}, "point_c": {"x": 0.0, "y": 1.0, "z": 0.0}, "point_d": {"x": 0.0, "y": 0.0, "z": 1.0}}' | \
  ./curl.sh http://127.0.0.1:3000/tetrahedron-volume
```

## Technical Details

- **Algorithm**: Scalar triple product method (|AB · (AC × AD)|/6)
- **Formula**: Volume = |det(AB, AC, AD)|/6 where A is reference vertex
- **Accuracy**: Exact calculation for any non-coplanar points
- **Validation**: Handles coplanar points (returns volume = 0)
- **Performance**: Sub-millisecond response time

## Use Cases

- **3D Modeling**: Mesh volume calculations and quality analysis
- **Engineering**: Finite element analysis and structural calculations
- **Scientific Computing**: Numerical integration and spatial analysis
- **Game Development**: Collision detection and procedural geometry