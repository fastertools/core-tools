# Rotation Matrix Tool

Generate 3x3 rotation matrices for rotations around coordinate axes.

## Overview

This tool creates rotation matrices for rotations around the X, Y, or Z axes by a specified angle in radians, using standard 3D rotation matrix formulas.

## API Endpoint

```
POST /rotation-matrix
```

## Input

```json
{
  "axis": "z",
  "angle": 1.5708
}
```

## Output

```json
{
  "matrix": {
    "m00": 0.0, "m01": -1.0, "m02": 0.0,
    "m10": 1.0, "m11": 0.0, "m12": 0.0,
    "m20": 0.0, "m21": 0.0, "m22": 1.0
  }
}
```

## Example Usage

```bash
# Generate 90-degree rotation around Z-axis
echo '{"axis": "z", "angle": 1.5708}' | \
  ./curl.sh http://127.0.0.1:3000/rotation-matrix
```

## Technical Details

- **Algorithm**: Standard 3D rotation matrix formulas
- **X-Axis Rotation**: Rodrigues' rotation formula around X
- **Y-Axis Rotation**: Rodrigues' rotation formula around Y  
- **Z-Axis Rotation**: Rodrigues' rotation formula around Z
- **Angle Units**: Radians (use Math.PI/2 for 90 degrees)
- **Matrix Format**: Row-major 3x3 matrix representation
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: 3D object rotation and transformation
- **Game Development**: Character and camera movement systems
- **CAD Applications**: Part orientation and assembly modeling
- **Robotics**: Joint rotation and pose transformation