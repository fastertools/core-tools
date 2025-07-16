# Line Intersection Tool

Determine intersection relationships between two 3D lines with comprehensive geometric analysis.

## Overview

This tool analyzes two 3D lines and determines their relationship: intersecting, parallel, coincident, or skew. It provides closest points, minimum distance, and parametric values for all cases.

## API Endpoint

```
POST /line-intersection
```

## Input

```json
{
  "line1": {
    "point": {"x": 0.0, "y": 0.0, "z": 0.0},
    "direction": {"x": 1.0, "y": 0.0, "z": 0.0}
  },
  "line2": {
    "point": {"x": 0.0, "y": 1.0, "z": 0.0},
    "direction": {"x": 0.0, "y": 0.0, "z": 1.0}
  }
}
```

## Output

```json
{
  "intersection_type": "skew",
  "intersects": false,
  "intersection_point": null,
  "closest_point_line1": {"x": 0.0, "y": 0.0, "z": 0.0},
  "closest_point_line2": {"x": 0.0, "y": 1.0, "z": 0.0},
  "minimum_distance": 1.0,
  "parameter_line1": 0.0,
  "parameter_line2": 0.0,
  "are_parallel": false,
  "are_skew": true,
  "are_coincident": false
}
```

## Example Usage

```bash
# Check intersection of two lines
echo '{"line1": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}, "line2": {"point": {"x": 0.0, "y": 0.0, "z": 1.0}, "direction": {"x": 0.0, "y": 1.0, "z": 0.0}}}' | \
  ./curl.sh http://127.0.0.1:3000/line-intersection
```

## Technical Details

- **Algorithm**: Parametric line analysis with distance minimization
- **Intersection Types**: Intersecting, parallel, coincident, skew
- **Tolerance**: 1e-10 for intersection and parallel detection
- **Closest Points**: Computed for all line configurations
- **Parameters**: Parametric values where closest points occur
- **Performance**: Sub-millisecond response time

## Use Cases

- **CAD Systems**: Feature intersection and constraint solving
- **Computer Graphics**: Ray-object intersection testing
- **Robotics**: Path planning and collision avoidance
- **Engineering**: Structural analysis and geometric modeling