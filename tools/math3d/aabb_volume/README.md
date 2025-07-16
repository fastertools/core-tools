# AABB Volume Tool

Calculate the volume of an Axis-Aligned Bounding Box (AABB) from a set of points.

## Overview

This tool computes the volume of the smallest axis-aligned bounding box that contains all input points, providing bounding box dimensions and corner coordinates.

## API Endpoint

```
POST /aabb-volume
```

## Input

```json
{
  "points": [
    {"x": 1.0, "y": 2.0, "z": 3.0},
    {"x": 4.0, "y": 5.0, "z": 6.0},
    {"x": 0.0, "y": 1.0, "z": 2.0}
  ]
}
```

## Output

```json
{
  "volume": 60.0,
  "box_type": "AABB (Axis-Aligned Bounding Box)",
  "min_point": {"x": 0.0, "y": 1.0, "z": 2.0},
  "max_point": {"x": 4.0, "y": 5.0, "z": 6.0},
  "dimensions": {"x": 4.0, "y": 4.0, "z": 4.0}
}
```

## Example Usage

```bash
# Calculate AABB volume for set of points
echo '{"points": [{"x": 1.0, "y": 2.0, "z": 3.0}, {"x": 4.0, "y": 5.0, "z": 6.0}, {"x": 0.0, "y": 1.0, "z": 2.0}]}' | \
  ./curl.sh http://127.0.0.1:3000/aabb-volume
```

## Technical Details

- **Algorithm**: Min/max coordinate finding across all dimensions
- **Volume Formula**: (max_x - min_x) × (max_y - min_y) × (max_z - min_z)
- **Axis Alignment**: Bounding box aligned with coordinate axes
- **Minimal Enclosure**: Smallest possible axis-aligned box containing all points
- **Performance**: Linear time complexity O(n) for n points

## Use Cases

- **3D Graphics**: Collision detection and spatial partitioning
- **Game Development**: Object bounding volume calculations
- **CAD Systems**: Part envelope and clearance analysis
- **Scientific Computing**: Data range analysis and visualization bounds