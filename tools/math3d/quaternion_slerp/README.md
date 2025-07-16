# Quaternion SLERP Tool

Perform spherical linear interpolation (SLERP) between two quaternions.

## Overview

This tool performs spherical linear interpolation between two quaternions, providing smooth rotation interpolation essential for animation and orientation blending.

## API Endpoint

```
POST /quaternion-slerp
```

## Input

```json
{
  "q1": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.0,
    "w": 1.0
  },
  "q2": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.7071,
    "w": 0.7071
  },
  "t": 0.5
}
```

## Output

```json
{
  "result": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.3827,
    "w": 0.9239
  }
}
```

## Example Usage

```bash
# Interpolate halfway between identity and 90-degree Z rotation
echo '{"q1": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "q2": {"x": 0.0, "y": 0.0, "z": 0.7071, "w": 0.7071}, "t": 0.5}' | \
  ./curl.sh http://127.0.0.1:3000/quaternion-slerp
```

## Technical Details

- **Algorithm**: Spherical linear interpolation with geodesic path
- **Parameter Range**: t ∈ [0,1] where 0=q1, 1=q2
- **Path**: Shortest arc on 4D unit sphere
- **Optimization**: Linear interpolation for nearly identical quaternions
- **Sign Handling**: Automatic quaternion sign selection for shortest path
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Animation**: Smooth character and object rotation
- **Game Development**: Camera movement and orientation blending
- **Robotics**: Smooth joint transitions and path planning
- **VR/AR Applications**: Head tracking and motion interpolation