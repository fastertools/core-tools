# Matrix Vector Multiply Tool

Multiply a 3x3 matrix by a 3D vector for linear transformations.

## Overview

This tool performs matrix-vector multiplication, essential for applying linear transformations such as rotations, scaling, and shearing to 3D vectors.

## API Endpoint

```
POST /matrix-vector-multiply
```

## Input

```json
{
  "matrix": {
    "m00": 1.0, "m01": 0.0, "m02": 0.0,
    "m10": 0.0, "m11": 0.0, "m12": -1.0,
    "m20": 0.0, "m21": 1.0, "m22": 0.0
  },
  "vector": {
    "x": 1.0,
    "y": 2.0,
    "z": 3.0
  }
}
```

## Output

```json
{
  "result": {
    "x": 1.0,
    "y": -3.0,
    "z": 2.0
  }
}
```

## Example Usage

```bash
# Apply rotation matrix to vector
echo '{"matrix": {"m00": 1.0, "m01": 0.0, "m02": 0.0, "m10": 0.0, "m11": 0.0, "m12": -1.0, "m20": 0.0, "m21": 1.0, "m22": 0.0}, "vector": {"x": 1.0, "y": 2.0, "z": 3.0}}' | \
  ./curl.sh http://127.0.0.1:3000/matrix-vector-multiply
```

## Technical Details

- **Algorithm**: Standard matrix-vector multiplication
- **Formula**: result[i] = Σ(matrix[i][j] * vector[j])
- **Matrix Format**: Row-major 3x3 matrix representation
- **Linear Transformation**: Applies matrix transformation to vector
- **Applications**: Rotation, scaling, shearing, projection
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Vertex transformation in 3D rendering
- **Game Development**: Object positioning and orientation
- **CAD Systems**: Geometric transformation and modeling
- **Physics Simulations**: Force and motion vector calculations