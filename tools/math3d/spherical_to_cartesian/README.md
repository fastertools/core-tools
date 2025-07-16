# Spherical to Cartesian Tool

Convert spherical coordinates (radius, theta, phi) to Cartesian coordinates (x, y, z).

## Overview

This tool converts spherical coordinates to 3D Cartesian coordinates using physics convention, transforming radius, azimuthal angle, and polar angle to xyz coordinates.

## API Endpoint

```
POST /spherical-to-cartesian
```

## Input

```json
{
  "coordinates": {
    "radius": 7.071,
    "theta": 0.927,
    "phi": 0.785
  }
}
```

## Output

```json
{
  "original_spherical": {
    "radius": 7.071,
    "theta": 0.927,
    "phi": 0.785
  },
  "cartesian_coordinates": {"x": 3.0, "y": 4.0, "z": 5.0},
  "conversion_notes": "Converted from Spherical (r=7.071, θ=0.927 rad, φ=0.785 rad) to Cartesian (3.000, 4.000, 5.000)"
}
```

## Example Usage

```bash
# Convert spherical coordinates to Cartesian
echo '{"coordinates": {"radius": 7.071, "theta": 0.927, "phi": 0.785}}' | \
  ./curl.sh http://127.0.0.1:3000/spherical-to-cartesian
```

## Technical Details

- **X**: r × sin(φ) × cos(θ)
- **Y**: r × sin(φ) × sin(θ) 
- **Z**: r × cos(φ)
- **Validation**: Prevents negative radius values
- **Convention**: Physics convention (ISO 80000-2)
- **Performance**: Sub-millisecond response time

## Use Cases

- **Physics**: Field calculations and particle positioning
- **Computer Graphics**: Spherical coordinate-based modeling
- **Astronomy**: Converting celestial coordinates to 3D positions
- **Engineering**: Antenna design and spherical coordinate systems