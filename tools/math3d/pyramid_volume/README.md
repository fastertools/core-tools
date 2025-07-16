# Pyramid Volume Tool

Calculate the volume of a pyramid with an arbitrary polygonal base and apex point.

## Overview

This tool computes the volume of a pyramid given a polygonal base and apex point using the formula V = (1/3) × base_area × height, with advanced polygon area calculation.

## API Endpoint

```
POST /pyramid-volume
```

## Input

```json
{
  "base_points": [
    {"x": 0.0, "y": 0.0, "z": 0.0},
    {"x": 2.0, "y": 0.0, "z": 0.0},
    {"x": 2.0, "y": 2.0, "z": 0.0},
    {"x": 0.0, "y": 2.0, "z": 0.0}
  ],
  "apex": {"x": 1.0, "y": 1.0, "z": 3.0}
}
```

## Output

```json
{
  "volume": 4.0,
  "calculation_method": "Pyramid formula: (1/3) × base_area × height",
  "base_area": 4.0,
  "height": 3.0,
  "base_points": [
    {"x": 0.0, "y": 0.0, "z": 0.0},
    {"x": 2.0, "y": 0.0, "z": 0.0},
    {"x": 2.0, "y": 2.0, "z": 0.0},
    {"x": 0.0, "y": 2.0, "z": 0.0}
  ],
  "apex": {"x": 1.0, "y": 1.0, "z": 3.0}
}
```

## Example Usage

```bash
# Calculate volume of square pyramid
echo '{"base_points": [{"x": 0.0, "y": 0.0, "z": 0.0}, {"x": 2.0, "y": 0.0, "z": 0.0}, {"x": 2.0, "y": 2.0, "z": 0.0}, {"x": 0.0, "y": 2.0, "z": 0.0}], "apex": {"x": 1.0, "y": 1.0, "z": 3.0}}' | \
  ./curl.sh http://127.0.0.1:3000/pyramid-volume
```

## Technical Details

- **Algorithm**: (1/3) × base_area × height formula
- **Base Area**: 3D polygon area using projection and shoelace formula
- **Height Calculation**: Point-to-plane distance from apex to base
- **Polygon Support**: Any convex or concave polygon base
- **3D Geometry**: Handles arbitrary orientations in 3D space
- **Performance**: Linear time complexity for base area calculation

## Use Cases

- **Architecture**: Building volume calculations and space analysis
- **Engineering**: Structural component volume estimation
- **3D Modeling**: Procedural geometry and mesh analysis
- **Manufacturing**: Material volume and cost estimation