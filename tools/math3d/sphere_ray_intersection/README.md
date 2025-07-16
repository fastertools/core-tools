# Sphere Ray Intersection Tool

Calculate intersection points between a ray and a sphere in 3D space.

## Overview

This tool determines if and where a ray intersects with a sphere, providing intersection points, distances, and surface normals for ray tracing and collision detection applications.

## API Endpoint

```
POST /sphere-ray-intersection
```

## Input

```json
{
  "sphere": {
    "center": {"x": 0.0, "y": 0.0, "z": 0.0},
    "radius": 2.0
  },
  "ray": {
    "origin": {"x": -5.0, "y": 0.0, "z": 0.0},
    "direction": {"x": 1.0, "y": 0.0, "z": 0.0}
  }
}
```

## Output

```json
{
  "intersects": true,
  "intersection_points": [
    {
      "point": {"x": -2.0, "y": 0.0, "z": 0.0},
      "distance": 3.0,
      "normal": {"x": -1.0, "y": 0.0, "z": 0.0}
    },
    {
      "point": {"x": 2.0, "y": 0.0, "z": 0.0},
      "distance": 7.0,
      "normal": {"x": 1.0, "y": 0.0, "z": 0.0}
    }
  ],
  "closest_distance": 3.0
}
```

## Example Usage

```bash
# Test ray-sphere intersection
echo '{"sphere": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 2.0}, "ray": {"origin": {"x": -5.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}' | \
  ./curl.sh http://127.0.0.1:3000/sphere-ray-intersection
```

## Technical Details

- **Algorithm**: Geometric ray-sphere intersection using quadratic formula
- **Cases Handled**: No intersection, tangent, entry/exit points
- **Ray Direction**: Automatically normalized for consistent results
- **Surface Normals**: Computed for each intersection point
- **Distance**: Parametric distance along ray to intersection
- **Performance**: Sub-millisecond response time

## Use Cases

- **Ray Tracing**: Photorealistic rendering and light simulation
- **Game Development**: Collision detection and physics systems
- **CAD Systems**: Object selection and geometric analysis
- **Scientific Visualization**: Particle tracking and simulation