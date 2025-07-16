# Vector Magnitude Tool

Calculate the magnitude and unit vector of a 3D vector.

## Overview

This tool computes the magnitude (length) of a 3D vector and returns the corresponding unit vector, with special handling for zero vectors.

## API Endpoint

```
POST /vector-magnitude
```

## Input

```json
{
  "vector": {
    "x": 3.0,
    "y": 4.0,
    "z": 0.0
  }
}
```

## Output

```json
{
  "magnitude": 5.0,
  "unit_vector": {
    "x": 0.6,
    "y": 0.8,
    "z": 0.0
  },
  "is_zero_vector": false
}
```

## Example Usage

```bash
# Calculate magnitude of a 3-4-0 vector
echo '{"vector": {"x": 3.0, "y": 4.0, "z": 0.0}}' | \
  ./curl.sh http://127.0.0.1:3000/vector-magnitude
```

## Technical Details

- **Algorithm**: Euclidean norm (√(x² + y² + z²))
- **Unit Vector**: Normalized vector with magnitude 1
- **Zero Vector Detection**: Magnitude < 1e-10 tolerance
- **Error Handling**: Graceful handling of zero vector normalization
- **Performance**: Sub-millisecond response time

## Use Cases

- **Computer Graphics**: Vector normalization for lighting calculations
- **Physics Simulations**: Velocity and force vector analysis
- **3D Modeling**: Direction vector calculations and scaling
- **Game Development**: Movement and collision detection systems