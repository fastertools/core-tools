# Quaternion From Axis Angle Tool

Create a quaternion representing a rotation around an arbitrary axis by a specified angle.

## Overview

This tool converts axis-angle rotation representation to quaternion form, providing a numerically stable method for representing 3D rotations.

## API Endpoint

```
POST /quaternion-from-axis-angle
```

## Input

```json
{
  "axis": {"x": 0.0, "y": 0.0, "z": 1.0},
  "angle": 1.5708
}
```

## Output

```json
{
  "quaternion": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.7071,
    "w": 0.7071
  }
}
```

## Example Usage

```bash
# Create quaternion for 90-degree rotation around Z-axis
echo '{"axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "angle": 1.5708}' | \
  ./curl.sh http://127.0.0.1:3000/quaternion-from-axis-angle
```

## Technical Details

- **Algorithm**: Axis-angle to quaternion conversion formula
- **Formula**: q = [sin(θ/2) * axis_unit, cos(θ/2)]
- **Normalization**: Automatically normalizes the input axis vector
- **Range**: Angle in radians, any real value
- **Validation**: Prevents zero axis vectors
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Smooth rotation interpolation and animation
- **Game Development**: Character and camera rotation systems
- **Robotics**: Joint angle representation and kinematics
- **Physics Simulations**: Angular velocity and orientation tracking