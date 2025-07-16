# Quaternion Multiply Tool

Multiply two quaternions to compose rotations in 3D space.

## Overview

This tool performs quaternion multiplication, which is essential for composing multiple rotations. The result represents the combined effect of applying both rotations sequentially.

## API Endpoint

```
POST /quaternion-multiply
```

## Input

```json
{
  "q1": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.7071,
    "w": 0.7071
  },
  "q2": {
    "x": 0.7071,
    "y": 0.0,
    "z": 0.0,
    "w": 0.7071
  }
}
```

## Output

```json
{
  "result": {
    "x": 0.5,
    "y": 0.5,
    "z": 0.5,
    "w": 0.5
  }
}
```

## Example Usage

```bash
# Multiply two quaternions representing 90-degree rotations
echo '{"q1": {"x": 0.0, "y": 0.0, "z": 0.7071, "w": 0.7071}, "q2": {"x": 0.7071, "y": 0.0, "z": 0.0, "w": 0.7071}}' | \
  ./curl.sh http://127.0.0.1:3000/quaternion-multiply
```

## Technical Details

- **Algorithm**: Standard quaternion multiplication (Hamilton product)
- **Formula**: q1 * q2 = [w1w2 - v1·v2, w1v2 + w2v1 + v1×v2]
- **Order**: Non-commutative (q1*q2 ≠ q2*q1)
- **Composition**: Result = apply q1 then q2 rotation
- **Numerical Stability**: Maintains quaternion properties
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Animation and rotation composition
- **Game Development**: Character orientation and camera movement
- **Robotics**: Multi-axis rotation calculations and kinematics
- **Physics Simulations**: Angular motion and orientation tracking