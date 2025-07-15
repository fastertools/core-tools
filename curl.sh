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

echo "All tests completed!"