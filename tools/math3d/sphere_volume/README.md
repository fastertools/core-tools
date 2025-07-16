# Sphere Volume Tool

Calculate the volume of a sphere given its center point and radius.

## Overview

This tool computes the volume of a sphere using the standard formula (4/3)πr³, with validation for negative radius values.

## API Endpoint

```
POST /sphere-volume
```

## Input

```json
{
  "center": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.0
  },
  "radius": 5.0
}
```

## Output

```json
{
  "volume": 523.5988,
  "calculation_method": "Sphere formula: (4/3)πr³",
  "center": {
    "x": 0.0,
    "y": 0.0,
    "z": 0.0
  },
  "radius": 5.0
}
```

## Example Usage

```bash
# Calculate volume of sphere with radius 5
echo '{"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 5.0}' | \
  ./curl.sh http://127.0.0.1:3000/sphere-volume
```

## Technical Details

- **Algorithm**: Standard sphere volume formula (4/3)πr³
- **Validation**: Prevents negative radius values
- **Precision**: Full double-precision floating-point accuracy
- **Units**: Volume in cubic units (same as radius cubed)
- **Performance**: Sub-millisecond response time

## Use Cases

- **Engineering**: Fluid capacity and material volume calculations
- **3D Modeling**: Object volume analysis and optimization
- **Physics Simulations**: Mass and density calculations
- **Manufacturing**: Material estimation and cost analysis