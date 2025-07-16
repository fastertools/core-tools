# Plane Plane Intersection Tool

Calculate the intersection between two 3D planes with comprehensive geometric analysis.

## Overview

This tool analyzes two 3D planes and determines their relationship: intersecting (forming a line), parallel, or coincident. It provides the intersection line, angles, and geometric classifications.

## API Endpoint

```
POST /plane-plane-intersection
```

## Input

```json
{
  "plane1": {
    "point": {"x": 0.0, "y": 0.0, "z": 0.0},
    "normal": {"x": 0.0, "y": 0.0, "z": 1.0}
  },
  "plane2": {
    "point": {"x": 0.0, "y": 0.0, "z": 1.0},
    "normal": {"x": 1.0, "y": 0.0, "z": 0.0}
  }
}
```

## Output

```json
{
  "intersection_type": "intersecting",
  "intersects": true,
  "intersection_line": {
    "point": {"x": 0.0, "y": 0.0, "z": 0.0},
    "direction": {"x": 0.0, "y": 1.0, "z": 0.0}
  },
  "are_parallel": false,
  "are_coincident": false,
  "angle_radians": 1.5708,
  "angle_degrees": 90.0
}
```

## Example Usage

```bash
# Find intersection of XZ plane and YZ plane
echo '{"plane1": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "normal": {"x": 0.0, "y": 0.0, "z": 1.0}}, "plane2": {"point": {"x": 0.0, "y": 0.0, "z": 1.0}, "normal": {"x": 1.0, "y": 0.0, "z": 0.0}}}' | \
  ./curl.sh http://127.0.0.1:3000/plane-plane-intersection
```

## Technical Details

- **Algorithm**: Cross product for intersection line direction
- **Intersection Types**: Intersecting, parallel, coincident
- **Line Calculation**: Finds optimal point on intersection line
- **Angle Computation**: Uses normalized normal vectors
- **Tolerance**: 1e-10 for parallel and coincident detection
- **Performance**: Sub-millisecond response time

## Use Cases

- **CAD Systems**: Feature intersection and geometric modeling
- **Computer Graphics**: Clipping plane calculations and CSG operations
- **Engineering**: Structural analysis and intersection detection
- **Architecture**: Building plane intersections and space analysis