# Cylinder Volume Tool

Calculate the volume of a cylinder given its base center, axis, radius, and height.

## Overview

This tool computes the volume of a cylinder using the standard formula πr²h, with validation for negative dimensions and support for arbitrary orientation in 3D space.

## API Endpoint

```
POST /cylinder-volume
```

## Input

```json
{
  "base_center": {"x": 0.0, "y": 0.0, "z": 0.0},
  "axis": {"x": 0.0, "y": 0.0, "z": 1.0},
  "radius": 2.0,
  "height": 5.0
}
```

## Output

```json
{
  "volume": 62.8319,
  "calculation_method": "Cylinder formula: πr²h",
  "base_center": {"x": 0.0, "y": 0.0, "z": 0.0},
  "axis": {"x": 0.0, "y": 0.0, "z": 1.0},
  "radius": 2.0,
  "height": 5.0
}
```

## Example Usage

```bash
# Calculate volume of cylinder with radius 2 and height 5
echo '{"base_center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": 2.0, "height": 5.0}' | \
  ./curl.sh http://127.0.0.1:3000/cylinder-volume
```

## Technical Details

- **Algorithm**: Standard cylinder volume formula (πr²h)
- **Validation**: Prevents negative radius and height values
- **Orientation**: Supports arbitrary axis direction in 3D space
- **Precision**: Full double-precision floating-point accuracy
- **Units**: Volume in cubic units (same as radius²×height)
- **Performance**: Sub-millisecond response time

## Use Cases

- **Engineering**: Pipe and tank capacity calculations
- **Manufacturing**: Material volume estimation and cost analysis
- **3D Modeling**: Procedural geometry and object analysis
- **Physics Simulations**: Fluid dynamics and container modeling