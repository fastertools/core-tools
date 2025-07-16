# Cartesian to Spherical Tool

Convert Cartesian coordinates (x, y, z) to spherical coordinates (radius, theta, phi).

## Overview

This tool converts 3D Cartesian coordinates to spherical coordinates using physics convention, providing radius, azimuthal angle (theta), and polar angle (phi).

## API Endpoint

```
POST /cartesian-to-spherical
```

## Input

```json
{
  "coordinates": {
    "x": 3.0,
    "y": 4.0,
    "z": 5.0
  }
}
```

## Output

```json
{
  "original_cartesian": {"x": 3.0, "y": 4.0, "z": 5.0},
  "spherical_coordinates": {
    "radius": 7.071,
    "theta": 0.927,
    "phi": 0.785
  },
  "conversion_notes": "Converted from Cartesian (3.000, 4.000, 5.000) to Spherical (r=7.071, θ=0.927 rad, φ=0.785 rad)"
}
```

## Example Usage

```bash
# Convert point (3,4,5) to spherical coordinates
echo '{"coordinates": {"x": 3.0, "y": 4.0, "z": 5.0}}' | \
  ./curl.sh http://127.0.0.1:3000/cartesian-to-spherical
```

## Technical Details

- **Radius**: √(x² + y² + z²) - distance from origin
- **Theta (θ)**: atan2(y, x) - azimuthal angle in XY plane
- **Phi (φ)**: acos(z/r) - polar angle from positive Z axis
- **Angle Ranges**: θ ∈ [-π, π], φ ∈ [0, π]
- **Convention**: Physics convention (ISO 80000-2)
- **Performance**: Sub-millisecond response time

## Use Cases

- **Physics**: Electromagnetic field calculations and quantum mechanics
- **Computer Graphics**: Spherical mapping and environment mapping
- **Astronomy**: Celestial coordinate systems and star positions
- **Navigation**: GPS and spherical Earth calculations