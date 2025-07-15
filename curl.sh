#!/bin/bash

# Core Tools MCP Server Testing Script
# Add/remove curl commands as needed for testing

BASE_URL="http://127.0.0.1:3000"

echo "Testing Core Tools MCP Server..."
echo "Base URL: $BASE_URL"
echo

# Test MCP tools list
echo "=== MCP Tools List ==="
curl -X POST $BASE_URL/mcp -H "Content-Type: application/json" -d '{"jsonrpc": "2.0", "method": "tools/list", "params": {}, "id": 1}' | jq '.result.tools[] | {name, title, description}' 2>/dev/null || echo "Failed to parse JSON"
echo

# Test basic math tools
echo "=== Basic Math Tools ==="
echo "Testing add tool:"
curl -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{"a": 5, "b": 3}'
echo

echo "Testing pythagorean tool (with service chaining):"
curl -X POST $BASE_URL/pythagorean -H "Content-Type: application/json" -d '{"a": 3, "b": 4}'
echo

# Test geospatial tools
echo "=== Geospatial Tools ==="
echo "Testing distance calculation (NYC to LA):"
curl -X POST $BASE_URL/distance -H "Content-Type: application/json" -d '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}'
echo

echo "Testing geofencing (point in polygon):"
curl -X POST $BASE_URL/point-in-polygon -H "Content-Type: application/json" -d '{"point": {"lat": 40.7128, "lon": -74.0060}, "polygon": [{"lat": 40.7, "lon": -74.0}, {"lat": 40.72, "lon": -74.0}, {"lat": 40.72, "lon": -74.01}, {"lat": 40.7, "lon": -74.01}]}'
echo

# Test 3D math tools
echo "=== 3D Mathematics Tools ==="
echo "Testing 3D dot product:"
curl -X POST $BASE_URL/dot-product -H "Content-Type: application/json" -d '{"vector1": {"x": 1.0, "y": 2.0, "z": 3.0}, "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}}'
echo

echo "Testing 3D line intersection:"
curl -X POST $BASE_URL/line-intersection -H "Content-Type: application/json" -d '{"line1": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}, "line2": {"point": {"x": 1.0, "y": 0.0, "z": 0.0}, "direction": {"x": 0.0, "y": 1.0, "z": 0.0}}}'
echo

# Test plane operations
echo "=== 3D Plane Operations ==="
echo "Testing line-plane intersection:"
curl -X POST $BASE_URL/line-plane-intersection -H "Content-Type: application/json" -d '{"line": {"point": {"x": 0.0, "y": 0.0, "z": 1.0}, "direction": {"x": 0.0, "y": 0.0, "z": -1.0}}, "plane": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "normal": {"x": 0.0, "y": 0.0, "z": 1.0}}}'
echo

echo "Testing point-plane distance:"
curl -X POST $BASE_URL/point-plane-distance -H "Content-Type: application/json" -d '{"point": {"x": 1.0, "y": 2.0, "z": 5.0}, "plane": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "normal": {"x": 0.0, "y": 0.0, "z": 1.0}}}'
echo

# Test transformation tools
echo "=== 3D Transformation Tools ==="
echo "Testing rotation matrix (90 degree Z rotation):"
curl -X POST $BASE_URL/rotation-matrix -H "Content-Type: application/json" -d '{"axis": "z", "angle": 1.5707963267948966}'
echo

echo "Testing rotation matrix (identity - 0 rotation):"
curl -X POST $BASE_URL/rotation-matrix -H "Content-Type: application/json" -d '{"axis": "x", "angle": 0}'
echo

echo "Testing rotation matrix error handling (invalid axis):"
curl -X POST $BASE_URL/rotation-matrix -H "Content-Type: application/json" -d '{"axis": "w", "angle": 1.0}'
echo

echo "Testing arbitrary rotation (45 degrees around [1,1,1] axis):"
curl -X POST $BASE_URL/arbitrary-rotation -H "Content-Type: application/json" -d '{"axis": {"x": 1.0, "y": 1.0, "z": 1.0}, "angle": 0.7853981633974483}'
echo

echo "Testing arbitrary rotation error handling (zero axis):"
curl -X POST $BASE_URL/arbitrary-rotation -H "Content-Type: application/json" -d '{"axis": {"x": 0.0, "y": 0.0, "z": 0.0}, "angle": 1.0}'
echo

# Test quaternion operations
echo "=== Quaternion Operations ==="
echo "Testing quaternion from axis-angle (90 degrees around Z axis):"
curl -X POST $BASE_URL/quaternion-from-axis-angle -H "Content-Type: application/json" -d '{"axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "angle": 1.5707963267948966}'
echo

echo "Testing quaternion from axis-angle error handling (zero axis):"
curl -X POST $BASE_URL/quaternion-from-axis-angle -H "Content-Type: application/json" -d '{"axis": {"x": 0.0, "y": 0.0, "z": 0.0}, "angle": 1.0}'
echo

echo "Testing quaternion multiplication (identity * rotation):"
curl -X POST $BASE_URL/quaternion-multiply -H "Content-Type: application/json" -d '{"q1": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "q2": {"x": 0.0, "y": 0.0, "z": 0.7071067811865475, "w": 0.7071067811865476}}'
echo

echo "Testing quaternion SLERP (50% interpolation between identity and Z-rotation):"
curl -X POST $BASE_URL/quaternion-slerp -H "Content-Type: application/json" -d '{"q1": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "q2": {"x": 0.0, "y": 0.0, "z": 0.7071067811865475, "w": 0.7071067811865476}, "t": 0.5}'
echo

echo "Testing quaternion SLERP error handling (t > 1.0):"
curl -X POST $BASE_URL/quaternion-slerp -H "Content-Type: application/json" -d '{"q1": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "q2": {"x": 0.0, "y": 0.0, "z": 0.7071067811865475, "w": 0.7071067811865476}, "t": 1.5}'
echo

# Test matrix operations
echo "=== Matrix Operations ===" 
echo "Testing matrix-vector multiplication (rotation example):"
curl -X POST $BASE_URL/matrix-vector-multiply -H "Content-Type: application/json" -d '{"matrix": {"m00": 0.0, "m01": -1.0, "m02": 0.0, "m10": 1.0, "m11": 0.0, "m12": 0.0, "m20": 0.0, "m21": 0.0, "m22": 1.0}, "vector": {"x": 1.0, "y": 0.0, "z": 0.0}}'
echo

# Test coordinate conversions
echo "=== Coordinate Conversions ==="
echo "Testing cartesian to spherical (point (1,1,1)):"
curl -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{"from_type": "cartesian", "to_type": "spherical", "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}}'
echo

echo "Testing spherical to cartesian (radius=2, theta=π/4, phi=π/3):"
curl -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{"from_type": "spherical", "to_type": "cartesian", "coordinates": {"x": 2.0, "y": 0.7853981633974483, "z": 1.0471975511965976}}'
echo

echo "Testing cartesian to cylindrical (point (3,4,5)):"
curl -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{"from_type": "cartesian", "to_type": "cylindrical", "coordinates": {"x": 3.0, "y": 4.0, "z": 5.0}}'
echo

echo "Testing invalid conversion type:"
curl -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{"from_type": "invalid", "to_type": "spherical", "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}}'
echo

# Test volume calculations
echo "=== Volume Calculations ==="
echo "Testing tetrahedron volume (unit tetrahedron):"
curl -X POST $BASE_URL/tetrahedron-volume -H "Content-Type: application/json" -d '{"point_a": {"x": 0.0, "y": 0.0, "z": 0.0}, "point_b": {"x": 1.0, "y": 0.0, "z": 0.0}, "point_c": {"x": 0.0, "y": 1.0, "z": 0.0}, "point_d": {"x": 0.0, "y": 0.0, "z": 1.0}}'
echo

echo "Testing sphere volume (radius=2, center at origin):"
curl -X POST $BASE_URL/sphere-volume -H "Content-Type: application/json" -d '{"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 2.0}'
echo

echo "Testing sphere volume error handling (negative radius):"
curl -X POST $BASE_URL/sphere-volume -H "Content-Type: application/json" -d '{"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": -1.0}'
echo

echo "Testing cylinder volume (radius=3, height=5):"
curl -X POST $BASE_URL/cylinder-volume -H "Content-Type: application/json" -d '{"base_center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": 3.0, "height": 5.0}'
echo

echo "Testing cylinder volume error handling (negative radius):"
curl -X POST $BASE_URL/cylinder-volume -H "Content-Type: application/json" -d '{"base_center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": -2.0, "height": 5.0}'
echo

echo "Testing AABB volume (unit cube from origin to (1,1,1)):"
curl -X POST $BASE_URL/aabb-volume -H "Content-Type: application/json" -d '{"points": [{"x": 0.0, "y": 0.0, "z": 0.0}, {"x": 1.0, "y": 1.0, "z": 1.0}]}'
echo

echo "Testing AABB volume (complex point cloud):"
curl -X POST $BASE_URL/aabb-volume -H "Content-Type: application/json" -d '{"points": [{"x": -1.0, "y": -2.0, "z": -3.0}, {"x": 2.0, "y": 1.0, "z": 4.0}, {"x": 0.5, "y": -0.5, "z": 1.5}]}'
echo

echo "Testing AABB volume error handling (empty points):"
curl -X POST $BASE_URL/aabb-volume -H "Content-Type: application/json" -d '{"points": []}'
echo

echo "Testing pyramid volume (square base with apex above):"
curl -X POST $BASE_URL/pyramid-volume -H "Content-Type: application/json" -d '{"base_points": [{"x": 0.0, "y": 0.0, "z": 0.0}, {"x": 2.0, "y": 0.0, "z": 0.0}, {"x": 2.0, "y": 2.0, "z": 0.0}, {"x": 0.0, "y": 2.0, "z": 0.0}], "apex": {"x": 1.0, "y": 1.0, "z": 3.0}}'
echo

echo "Testing pyramid volume error handling (insufficient base points):"
curl -X POST $BASE_URL/pyramid-volume -H "Content-Type: application/json" -d '{"base_points": [{"x": 0.0, "y": 0.0, "z": 0.0}, {"x": 1.0, "y": 0.0, "z": 0.0}], "apex": {"x": 0.5, "y": 1.0, "z": 2.0}}'
echo

# Test 3D primitive operations
echo "=== 3D Primitive Operations ==="
echo "Testing sphere-ray intersection (ray hits sphere):"
curl -X POST $BASE_URL/sphere-ray-intersection -H "Content-Type: application/json" -d '{"sphere": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 1.0}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing sphere-ray intersection (ray misses sphere):"
curl -X POST $BASE_URL/sphere-ray-intersection -H "Content-Type: application/json" -d '{"sphere": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 1.0}, "ray": {"origin": {"x": -2.0, "y": 2.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing sphere-ray intersection error handling (negative radius):"
curl -X POST $BASE_URL/sphere-ray-intersection -H "Content-Type: application/json" -d '{"sphere": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": -1.0}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing sphere-sphere intersection (two separate spheres):"
curl -X POST $BASE_URL/sphere-sphere-intersection -H "Content-Type: application/json" -d '{"sphere1": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 1.0}, "sphere2": {"center": {"x": 3.0, "y": 0.0, "z": 0.0}, "radius": 1.0}}'
echo

echo "Testing sphere-sphere intersection (two intersecting spheres):"
curl -X POST $BASE_URL/sphere-sphere-intersection -H "Content-Type: application/json" -d '{"sphere1": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 1.0}, "sphere2": {"center": {"x": 1.5, "y": 0.0, "z": 0.0}, "radius": 1.0}}'
echo

echo "Testing sphere-sphere intersection (one sphere inside another):"
curl -X POST $BASE_URL/sphere-sphere-intersection -H "Content-Type: application/json" -d '{"sphere1": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 2.0}, "sphere2": {"center": {"x": 0.5, "y": 0.0, "z": 0.0}, "radius": 0.5}}'
echo

echo "Testing sphere-sphere intersection (two spheres just touching):"
curl -X POST $BASE_URL/sphere-sphere-intersection -H "Content-Type: application/json" -d '{"sphere1": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": 1.0}, "sphere2": {"center": {"x": 2.0, "y": 0.0, "z": 0.0}, "radius": 1.0}}'
echo

echo "Testing sphere-sphere intersection error handling (negative radius):"
curl -X POST $BASE_URL/sphere-sphere-intersection -H "Content-Type: application/json" -d '{"sphere1": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "radius": -1.0}, "sphere2": {"center": {"x": 2.0, "y": 0.0, "z": 0.0}, "radius": 1.0}}'
echo

echo "Testing cylinder-ray intersection (ray hits cylinder):"
curl -X POST $BASE_URL/cylinder-ray-intersection -H "Content-Type: application/json" -d '{"cylinder": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": 1.0, "height": 2.0}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing cylinder-ray intersection (ray misses cylinder):"
curl -X POST $BASE_URL/cylinder-ray-intersection -H "Content-Type: application/json" -d '{"cylinder": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": 1.0, "height": 2.0}, "ray": {"origin": {"x": -2.0, "y": 2.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing cylinder-ray intersection error handling (negative radius):"
curl -X POST $BASE_URL/cylinder-ray-intersection -H "Content-Type: application/json" -d '{"cylinder": {"center": {"x": 0.0, "y": 0.0, "z": 0.0}, "axis": {"x": 0.0, "y": 0.0, "z": 1.0}, "radius": -1.0, "height": 2.0}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing ray-AABB intersection (ray hits box):"
curl -X POST $BASE_URL/ray-aabb-intersection -H "Content-Type: application/json" -d '{"aabb": {"min": {"x": -1.0, "y": -1.0, "z": -1.0}, "max": {"x": 1.0, "y": 1.0, "z": 1.0}}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing ray-AABB intersection (ray misses box):"
curl -X POST $BASE_URL/ray-aabb-intersection -H "Content-Type: application/json" -d '{"aabb": {"min": {"x": -1.0, "y": -1.0, "z": -1.0}, "max": {"x": 1.0, "y": 1.0, "z": 1.0}}, "ray": {"origin": {"x": -2.0, "y": 2.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing ray-AABB intersection error handling (invalid box):"
curl -X POST $BASE_URL/ray-aabb-intersection -H "Content-Type: application/json" -d '{"aabb": {"min": {"x": 1.0, "y": 1.0, "z": 1.0}, "max": {"x": -1.0, "y": -1.0, "z": -1.0}}, "ray": {"origin": {"x": -2.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

# Test 3D distance operations
echo "=== 3D Distance Operations ==="
echo "Testing point-line distance (point not on line):"
curl -X POST $BASE_URL/point-line-distance -H "Content-Type: application/json" -d '{"point": {"x": 2.0, "y": 3.0, "z": 0.0}, "line": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing point-line distance (point on line):"
curl -X POST $BASE_URL/point-line-distance -H "Content-Type: application/json" -d '{"point": {"x": 5.0, "y": 0.0, "z": 0.0}, "line": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "direction": {"x": 1.0, "y": 0.0, "z": 0.0}}}'
echo

echo "Testing point-line distance error handling (zero direction):"
curl -X POST $BASE_URL/point-line-distance -H "Content-Type: application/json" -d '{"point": {"x": 2.0, "y": 3.0, "z": 0.0}, "line": {"point": {"x": 0.0, "y": 0.0, "z": 0.0}, "direction": {"x": 0.0, "y": 0.0, "z": 0.0}}}'
echo

echo "All tests completed!"