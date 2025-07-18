#!/bin/bash

# Architecture Improvements Initiative - Focused Testing
# Testing only tools being worked on in current initiative

BASE_URL="http://127.0.0.1:3000"

echo "=== Architecture Improvements Initiative - Focused Testing ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo

# === LINE INTERSECTION TOOLS ===
echo "=== LINE INTERSECTION TOOLS ==="
echo

# Test Single Line Intersection (recently fixed from ToolResponse to Result pattern)
echo "--- Test: Line Intersection (intersecting lines) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/line-intersection -H "Content-Type: application/json" -d '{
  "line1": {
    "point": {"x": 0, "y": 0, "z": 0}, 
    "direction": {"x": 1, "y": 0, "z": 0}
  }, 
  "line2": {
    "point": {"x": 0, "y": 1, "z": 0}, 
    "direction": {"x": 0, "y": -1, "z": 0}
  }
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test Multiple Line Intersection (already extracted tool)
echo "--- Test: Multiple Line Intersection (3 lines) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiple-line-intersection -H "Content-Type: application/json" -d '{
  "lines": [
    {
      "point": {"x": 0, "y": 0, "z": 0}, 
      "direction": {"x": 1, "y": 0, "z": 0}
    },
    {
      "point": {"x": 1, "y": 1, "z": 0}, 
      "direction": {"x": 0, "y": -1, "z": 0}
    },
    {
      "point": {"x": 0, "y": 0, "z": 1}, 
      "direction": {"x": 0, "y": 0, "z": -1}
    }
  ]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === COORDINATE CONVERSION TOOLS ===
echo "=== COORDINATE CONVERSION TOOLS ==="
echo

# Test bundled coordinate conversion (to be extracted)
echo "--- Test: Coordinate Conversion (bundled tool) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{
  "from_type": "cartesian",
  "to_type": "spherical",
  "coordinates": {"x": 1, "y": 1, "z": 1}
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test already extracted tools
# Test cylindrical conversions to identify what needs extraction
echo "--- Test: Cartesian to Cylindrical (bundled - needs extraction) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{
  "from_type": "cartesian",
  "to_type": "cylindrical",
  "coordinates": {"x": 1, "y": 1, "z": 2}
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "--- Test: Cylindrical to Cartesian (bundled - needs extraction) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/coordinate-conversion-three-d -H "Content-Type: application/json" -d '{
  "from_type": "cylindrical",
  "to_type": "cartesian",
  "coordinates": {"x": 1.414, "y": 0.785, "z": 2}
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "--- Test: Cartesian to Spherical (extracted tool) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/cartesian-to-spherical -H "Content-Type: application/json" -d '{
  "x": 1,
  "y": 1, 
  "z": 1
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test newly extracted cylindrical conversion tools
echo "--- Test: Cartesian to Cylindrical (newly extracted tool) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/cartesian-to-cylindrical -H "Content-Type: application/json" -d '{
  "x": 1,
  "y": 1,
  "z": 2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "--- Test: Cylindrical to Cartesian (newly extracted tool) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/cylindrical-to-cartesian -H "Content-Type: application/json" -d '{
  "radius": 1.414,
  "theta": 0.785,
  "z": 2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === VECTOR ANALYSIS COMPOSITE TOOL ===
echo "=== VECTOR ANALYSIS COMPOSITE TOOL ==="
echo

# Test Vector Analysis (composite tool demonstrating HTTP composition pattern)
echo "--- Test: Vector Analysis (composite tool - calls vector_magnitude, vector_angle, dot_product, cross_product) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/vector-analysis -H "Content-Type: application/json" -d '{
  "vector_a": [1, 0, 0],
  "vector_b": [0, 1, 0]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "=== SUMMARY ==="
echo "This script tests tools in the Architecture Improvements Initiative:"
echo "1. line-intersection (pattern fixed)"
echo "2. multiple-line-intersection (already extracted)"
echo "3. coordinate conversion tools (coordinate-conversion-three-d fixed)"
echo "4. cartesian-to-cylindrical (newly extracted)"
echo "5. cylindrical-to-cartesian (newly extracted)"
echo "6. vector-analysis (composite tool demonstrating HTTP composition pattern)"
echo