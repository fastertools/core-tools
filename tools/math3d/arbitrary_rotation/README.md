# Arbitrary Rotation Tool

Generate a 3x3 rotation matrix for rotation around an arbitrary axis.

## Overview

This tool creates rotation matrices for rotations around any arbitrary axis using Rodrigues' rotation formula, providing more flexibility than coordinate axis rotations.

## API Endpoint

```
POST /arbitrary-rotation
```

## Input

```json
{
  "axis": {"x": 1.0, "y": 1.0, "z": 0.0},
  "angle": 0.7854
}
```

## Output

```json
{
  "matrix": {
    "m00": 0.8536, "m01": -0.1464, "m02": 0.5000,
    "m10": 0.1464, "m11": 0.8536, "m12": 0.5000,
    "m20": -0.5000, "m21": -0.5000, "m22": 0.7071
  }
}
```

## Example Usage

```bash
# Create 45-degree rotation around axis (1,1,0)
echo '{"axis": {"x": 1.0, "y": 1.0, "z": 0.0}, "angle": 0.7854}' | \
  ./curl.sh http://127.0.0.1:3000/arbitrary-rotation
```

## Technical Details

- **Algorithm**: Rodrigues' rotation formula
- **Formula**: R = I + sin(θ)[k]× + (1-cos(θ))[k]×²
- **Normalization**: Automatically normalizes the input axis vector
- **Angle Units**: Radians (use Math.PI/4 for 45 degrees)
- **Matrix Format**: Row-major 3x3 matrix representation
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Complex 3D object rotations and animations
- **Game Development**: Character movement and camera rotation systems
- **Robotics**: Joint rotation and robotic arm positioning
- **CAD Applications**: Part orientation and assembly positioning