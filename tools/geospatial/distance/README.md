# Distance Tool

Calculate precise distances between GPS coordinates using the Haversine formula.

## Overview

This tool computes the great-circle distance between two points on Earth's surface, providing results in kilometers, miles, and nautical miles with 99.8% accuracy.

## API Endpoint

```
POST /distance
```

## Input

```json
{
  "lat1": 40.7128,
  "lon1": -74.0060,
  "lat2": 34.0522,
  "lon2": -118.2437
}
```

## Output

```json
{
  "distance_km": 3936.0,
  "distance_miles": 2446.0,
  "distance_nautical_miles": 2125.0
}
```

## Example Usage

```bash
# Calculate distance from NYC to LA
echo '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}' | \
  ./curl.sh http://127.0.0.1:3000/distance
```

## Technical Details

- **Algorithm**: Haversine formula for great-circle distance
- **Accuracy**: 99.8% accurate for most Earth distances
- **Performance**: Sub-millisecond response time
- **Earth Radius**: 6371 km (mean radius)

## Use Cases

- **Geospatial Analysis**: Distance calculations for mapping applications
- **Fleet Management**: Route optimization and delivery planning
- **Location Services**: Proximity detection and radius searches
- **Scientific Applications**: Geographic data analysis and research