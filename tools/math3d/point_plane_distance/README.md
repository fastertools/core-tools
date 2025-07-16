# Point Plane Distance Tool

Calculate the distance from a point to a plane in 3D space with comprehensive geometric analysis.

## Overview

This tool computes both signed and unsigned distances from a point to a plane, determines which side of the plane the point is on, and finds the closest point on the plane.

## API Endpoint

```
POST /point-plane-distance
```

## Input

```json
{
  "point": {"x": 2.0, "y": 3.0, "z": 4.0},
  "plane": {
    "point": {"x": 0.0, "y": 0.0, "z": 0.0},
    "normal": {"x": 0.0, "y": 0.0, "z": 1.0}
  }
}
```

## Output

```json
{
  "distance": 4.0,
  "signed_distance": 4.0,
  "closest_point_on_plane": {"x": 2.0, "y": 3.0, "z": 0.0},
  "is_on_plane": false,
  "side_of_plane": "positive"
}
```

## Example Usage

```bash
# Calculate distance from point to XY plane
echo '{"point": {"x": 2.0, "y": 3.0, "z": 4.0}, "plane": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "normal": {"x": 0.0, "y": 0.0, "z": 1.0}}}' | \
  ./curl.sh http://127.0.0.1:3000/point-plane-distance
```

## Technical Details

- **Algorithm**: Point-to-plane projection using normal vector
- **Signed Distance**: Positive/negative based on normal direction
- **Tolerance**: 1e-10 for on-plane detection
- **Normal Validation**: Prevents zero normal vectors
- **Closest Point**: Orthogonal projection onto the plane
- **Performance**: Sub-millisecond response time

## Use Cases

- **CAD Systems**: Feature-to-surface distance measurements
- **Computer Graphics**: Clipping plane calculations and depth testing
- **Physics Simulations**: Collision detection and constraint solving
- **Engineering**: Tolerance analysis and quality control