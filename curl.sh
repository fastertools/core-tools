#!/bin/bash

# Core Tools MCP Server Testing Script - FINAL MIGRATION TOOL TEST
# Testing polygon-simplification - the final tool to complete 100% migration

BASE_URL="http://127.0.0.1:3000"

echo "=== FINAL MIGRATION: Testing Polygon Simplification Tool ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo "Testing: polygon-simplification (final tool for 100% completion)"
echo

# Add detailed logging
echo "=== Server Status Check ==="
echo "Port 3000 usage:"
lsof -i :3000 | head -5
echo

echo "WASM file verification:"
echo "Polygon simplification: $(ls -la tools/geospatial/polygon_simplification/target/wasm32-wasip1/release/polygon_simplification_tool.wasm 2>/dev/null || echo 'NOT FOUND')"
echo

echo "=== Component Log Check ==="
echo "Checking for component logs in .spin/logs/:"
ls -la .spin/logs/ | grep -E "polygon.*simplification" || echo "No logs found for polygon simplification tool"
echo

# Test 1: Polygon Simplification - Douglas-Peucker Algorithm
echo "=== Test 1: Polygon Simplification - Douglas-Peucker ==="
echo "Testing polygon simplification with Douglas-Peucker algorithm:"
echo "Request: POST $BASE_URL/polygon-simplification"
echo "Data: Complex polygon with 8 points (NYC area outline) with 100m tolerance"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/polygon-simplification -H "Content-Type: application/json" -d '{"polygon": [{"lat": 40.7128, "lon": -74.0060}, {"lat": 40.7589, "lon": -73.9851}, {"lat": 40.7831, "lon": -73.9712}, {"lat": 40.7505, "lon": -73.9934}, {"lat": 40.7282, "lon": -73.7949}, {"lat": 40.6892, "lon": -73.9442}, {"lat": 40.6782, "lon": -74.0442}, {"lat": 40.7128, "lon": -74.0060}], "tolerance_meters": 100.0, "algorithm": "douglas_peucker"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with simplified polygon (fewer vertices than original 8)"
echo


# Test 2: Polygon Simplification - Visvalingam Algorithm
echo "=== Test 2: Polygon Simplification - Visvalingam ==="
echo "Testing polygon simplification with Visvalingam algorithm:"
echo "Request: POST $BASE_URL/polygon-simplification"
echo "Data: Same polygon with 200m tolerance using Visvalingam"
echo
echo "Response:"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/polygon-simplification -H "Content-Type: application/json" -d '{"polygon": [{"lat": 40.7128, "lon": -74.0060}, {"lat": 40.7589, "lon": -73.9851}, {"lat": 40.7831, "lon": -73.9712}, {"lat": 40.7505, "lon": -73.9934}, {"lat": 40.7282, "lon": -73.7949}, {"lat": 40.6892, "lon": -73.9442}, {"lat": 40.6782, "lon": -74.0442}, {"lat": 40.7128, "lon": -74.0060}], "tolerance_meters": 200.0, "algorithm": "visvalingam"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')

echo "HTTP Status: $http_code"
echo "Response Body: $response_body"
echo "Expected: HTTP 200 with simplified polygon using different algorithm"
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

echo "=== FINAL MIGRATION TEST Complete ==="
echo "This tool completes 100% migration of all core tools!"