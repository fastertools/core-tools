#!/bin/bash

# Core Tools MCP Server Testing Script - SEGMENT 11 ARCHITECTURE REFINEMENT
# Testing new vector_angle tool - missing fundamental 3D math functionality

BASE_URL="http://127.0.0.1:3000"

echo "=== SEGMENT 11: Testing New Vector Angle Tool ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo "Testing: vector-angle (missing fundamental 3D math functionality)"
echo

# Add detailed logging
echo "=== Server Status Check ==="
echo "Port 3000 usage:"
lsof -i :3000 | head -5
echo

echo "WASM file verification:"
echo "Vector angle: $(ls -la tools/math3d/vector_angle/target/wasm32-wasip1/release/vector_angle_tool.wasm 2>/dev/null || echo 'NOT FOUND')"
echo

echo "=== Component Log Check ==="
echo "Checking for component logs in .spin/logs/:"
ls -la .spin/logs/ | grep -E "vector.*angle" || echo "No logs found for vector angle tool"
echo

# Test 1: Vector Angle - Perpendicular Vectors
echo "=== Test 1: Vector Angle - Perpendicular Vectors ==="
echo "Testing vector angle calculation (X and Y unit vectors - should be 90°):"
echo "Request: POST $BASE_URL/vector-angle"
echo "Data: {\"vector1\": {\"x\": 1.0, \"y\": 0.0, \"z\": 0.0}, \"vector2\": {\"x\": 0.0, \"y\": 1.0, \"z\": 0.0}}"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/vector-angle -H "Content-Type: application/json" -d '{"vector1": {"x": 1.0, "y": 0.0, "z": 0.0}, "vector2": {"x": 0.0, "y": 1.0, "z": 0.0}}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with angle_degrees=90.0, is_perpendicular=true, is_parallel=false"
echo


# Test 2: Vector Angle - Parallel Vectors
echo "=== Test 2: Vector Angle - Parallel Vectors ==="
echo "Testing vector angle calculation (same direction vectors - should be 0°):"
echo "Request: POST $BASE_URL/vector-angle"
echo "Data: {\"vector1\": {\"x\": 1.0, \"y\": 2.0, \"z\": 3.0}, \"vector2\": {\"x\": 2.0, \"y\": 4.0, \"z\": 6.0}}"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/vector-angle -H "Content-Type: application/json" -d '{"vector1": {"x": 1.0, "y": 2.0, "z": 3.0}, "vector2": {"x": 2.0, "y": 4.0, "z": 6.0}}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with angle_degrees=0.0, is_perpendicular=false, is_parallel=true"
echo

echo "=== Test Summary ==="
echo "All tests completed. Check HTTP status codes:"
echo "- 200 = Success"
echo "- 404 = Tool not found/registered"
echo "- 500 = Internal server error"
echo

echo "=== Debug Information ==="
echo "Recent server logs:"
echo "--- spin_stderr.log (last 10 lines) ---"
tail -10 spin_stderr.log
echo
echo "--- spin_stdout.log (last 5 lines) ---"
tail -5 spin_stdout.log
echo

# Test 3: Line Segment Intersection - Intersecting Segments
echo "=== Test 3: Line Segment Intersection - Intersecting Segments ==="
echo "Testing line segment intersection (crossing segments in 3D):"
echo "Request: POST $BASE_URL/line-segment-intersection"
echo "Data: {\"segment1_start\": {\"x\": 0.0, \"y\": 0.0, \"z\": 0.0}, \"segment1_end\": {\"x\": 2.0, \"y\": 2.0, \"z\": 0.0}, \"segment2_start\": {\"x\": 0.0, \"y\": 2.0, \"z\": 0.0}, \"segment2_end\": {\"x\": 2.0, \"y\": 0.0, \"z\": 0.0}}"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/line-segment-intersection -H "Content-Type: application/json" -d '{"segment1_start": {"x": 0.0, "y": 0.0, "z": 0.0}, "segment1_end": {"x": 2.0, "y": 2.0, "z": 0.0}, "segment2_start": {"x": 0.0, "y": 2.0, "z": 0.0}, "segment2_end": {"x": 2.0, "y": 0.0, "z": 0.0}}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with intersects=true, intersection_point near (1,1,0)"
echo

# Test 4: Line Segment Intersection - Non-intersecting Segments  
echo "=== Test 4: Line Segment Intersection - Non-intersecting Segments ==="
echo "Testing line segment intersection (parallel non-intersecting segments):"
echo "Request: POST $BASE_URL/line-segment-intersection"
echo "Data: {\"segment1_start\": {\"x\": 0.0, \"y\": 0.0, \"z\": 0.0}, \"segment1_end\": {\"x\": 1.0, \"y\": 0.0, \"z\": 0.0}, \"segment2_start\": {\"x\": 0.0, \"y\": 1.0, \"z\": 0.0}, \"segment2_end\": {\"x\": 1.0, \"y\": 1.0, \"z\": 0.0}}"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/line-segment-intersection -H "Content-Type: application/json" -d '{"segment1_start": {"x": 0.0, "y": 0.0, "z": 0.0}, "segment1_end": {"x": 1.0, "y": 0.0, "z": 0.0}, "segment2_start": {"x": 0.0, "y": 1.0, "z": 0.0}, "segment2_end": {"x": 1.0, "y": 1.0, "z": 0.0}}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with intersects=false, minimum_distance=1.0"
echo

# Test 5: Multiple Line Intersection - Three Lines Meeting at Point
echo "=== Test 5: Multiple Line Intersection - Three Lines Meeting at Point ==="
echo "Testing multiple line intersection (3 lines converging at origin):"
echo "Request: POST $BASE_URL/multiple-line-intersection"
echo "Data: {\"lines\": [{\"point\": {\"x\": 1.0, \"y\": 0.0, \"z\": 0.0}, \"direction\": {\"x\": -1.0, \"y\": 0.0, \"z\": 0.0}}, {\"point\": {\"x\": 0.0, \"y\": 1.0, \"z\": 0.0}, \"direction\": {\"x\": 0.0, \"y\": -1.0, \"z\": 0.0}}, {\"point\": {\"x\": 0.0, \"y\": 0.0, \"z\": 1.0}, \"direction\": {\"x\": 0.0, \"y\": 0.0, \"z\": -1.0}}]}"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiple-line-intersection -H "Content-Type: application/json" -d '{"lines": [{"point": {"x": 1.0, "y": 0.0, "z": 0.0}, "direction": {"x": -1.0, "y": 0.0, "z": 0.0}}, {"point": {"x": 0.0, "y": 1.0, "z": 0.0}, "direction": {"x": 0.0, "y": -1.0, "z": 0.0}}, {"point": {"x": 0.0, "y": 0.0, "z": 1.0}, "direction": {"x": 0.0, "y": 0.0, "z": -1.0}}]}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with best_intersection_point near (0,0,0), lines_processed=3"
echo

echo "=== SEGMENT 11 ARCHITECTURE REFINEMENT Test Complete ==="
echo "Vector angle and line segment intersection tools validate 3D mathematical composition principles!"