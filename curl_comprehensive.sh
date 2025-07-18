#!/bin/bash

# Comprehensive Testing and Validation Initiative - All 84 Tools
# Tests every HTTP endpoint defined in spin.toml with proper validation data

BASE_URL="http://127.0.0.1:3000"
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0

echo "=== Core Tools - Comprehensive HTTP Endpoint Testing ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo "Testing all 84 tools systematically..."
echo

# Function to test endpoint with proper HTTP code tracking
test_endpoint() {
    local endpoint="$1"
    local data="$2"
    local description="$3"
    
    TOTAL_TESTS=$((TOTAL_TESTS + 1))
    echo "--- Test $TOTAL_TESTS: $description ---"
    
    response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST "$BASE_URL/$endpoint" -H "Content-Type: application/json" -d "$data")
    http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
    response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
    
    echo "Endpoint: /$endpoint"
    echo "HTTP Code: $http_code"
    echo "Response: $response_body"
    
    if [ "$http_code" = "200" ]; then
        PASSED_TESTS=$((PASSED_TESTS + 1))
        echo "✅ PASS"
    else
        FAILED_TESTS=$((FAILED_TESTS + 1))
        echo "❌ FAIL"
    fi
    echo
}

echo "=== BASIC MATH TOOLS ==="
echo

# Basic Math Operations
test_endpoint "add" '{"a": 5, "b": 3}' "Addition - Basic Math"
test_endpoint "subtract" '{"a": 10, "b": 4}' "Subtraction - Basic Math"
test_endpoint "multiply" '{"a": 6, "b": 7}' "Multiplication - Basic Math"
test_endpoint "divide" '{"a": 15, "b": 3}' "Division - Basic Math"
test_endpoint "remainder" '{"a": 17, "b": 5}' "Remainder - Basic Math"
test_endpoint "modulus" '{"a": 17, "b": 5}' "Modulus - Basic Math"
test_endpoint "power" '{"base": 2, "exponent": 8}' "Power - Basic Math"
test_endpoint "square" '{"value": 9}' "Square - Basic Math"
test_endpoint "sqrt" '{"value": 64}' "Square Root - Basic Math"

# 2D Distance and Geometry
test_endpoint "pythagorean" '{"a": 3, "b": 4}' "Pythagorean Theorem"
test_endpoint "distance-two-d" '{"point1": {"x": 0, "y": 0}, "point2": {"x": 3, "y": 4}}' "2D Distance Calculation"

echo "=== GEOSPATIAL TOOLS ==="
echo

# Geospatial Operations
test_endpoint "distance" '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}' "Geographic Distance (NYC to LA)"
test_endpoint "bearing" '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}' "Geographic Bearing"
test_endpoint "coordinate-conversion" '{"latitude": 40.7128, "longitude": -74.0060, "from_format": "decimal", "to_format": "dms"}' "Coordinate Format Conversion"

# Polygon Operations  
test_endpoint "polygon-area" '{"points": [{"lat": 0, "lon": 0}, {"lat": 0, "lon": 1}, {"lat": 1, "lon": 1}, {"lat": 1, "lon": 0}]}' "Polygon Area Calculation"
test_endpoint "point-in-polygon" '{"point": {"lat": 0.5, "lon": 0.5}, "polygon": [{"lat": 0, "lon": 0}, {"lat": 0, "lon": 1}, {"lat": 1, "lon": 1}, {"lat": 1, "lon": 0}]}' "Point in Polygon Test"
test_endpoint "polygon-simplification" '{"points": [{"lat": 0, "lon": 0}, {"lat": 0.1, "lon": 0.1}, {"lat": 1, "lon": 1}], "tolerance": 0.05}' "Polygon Simplification"

# Buffer and Proximity
test_endpoint "buffer-polygon" '{"center": {"lat": 40.7128, "lon": -74.0060}, "radius_meters": 1000, "num_points": 16}' "Buffer Polygon Generation"
test_endpoint "proximity-search" '{"center": {"lat": 40.7128, "lon": -74.0060}, "radius_meters": 1000, "points": [{"lat": 40.713, "lon": -74.006}, {"lat": 40.720, "lon": -74.010}]}' "Proximity Search"
test_endpoint "proximity-zone" '{"center": {"lat": 40.7128, "lon": -74.0060}, "radius_meters": 1000}' "Proximity Zone Creation"

echo "=== 3D MATH TOOLS ==="
echo

# Vector Operations
test_endpoint "vector-magnitude" '{"vector": {"x": 3, "y": 4, "z": 5}}' "Vector Magnitude"
test_endpoint "vector-angle" '{"vector_a": {"x": 1, "y": 0, "z": 0}, "vector_b": {"x": 0, "y": 1, "z": 0}}' "Vector Angle"
test_endpoint "dot-product" '{"vector_a": {"x": 1, "y": 2, "z": 3}, "vector_b": {"x": 4, "y": 5, "z": 6}}' "Dot Product"
test_endpoint "cross-product" '{"vector_a": {"x": 1, "y": 0, "z": 0}, "vector_b": {"x": 0, "y": 1, "z": 0}}' "Cross Product"
test_endpoint "vector-analysis" '{"vector_a": [1, 0, 0], "vector_b": [0, 1, 0]}' "Vector Analysis (Composite Tool)"

# Line Operations
test_endpoint "line-intersection" '{"line1": {"point": {"x": 0, "y": 0, "z": 0}, "direction": {"x": 1, "y": 0, "z": 0}}, "line2": {"point": {"x": 0, "y": 1, "z": 0}, "direction": {"x": 0, "y": -1, "z": 0}}}' "Line Intersection"
test_endpoint "line-segment-intersection" '{"segment1": {"start": {"x": 0, "y": 0, "z": 0}, "end": {"x": 2, "y": 0, "z": 0}}, "segment2": {"start": {"x": 1, "y": -1, "z": 0}, "end": {"x": 1, "y": 1, "z": 0}}}' "Line Segment Intersection"
test_endpoint "multiple-line-intersection" '{"lines": [{"point": {"x": 0, "y": 0, "z": 0}, "direction": {"x": 1, "y": 0, "z": 0}}, {"point": {"x": 1, "y": 1, "z": 0}, "direction": {"x": 0, "y": -1, "z": 0}}]}' "Multiple Line Intersection"
test_endpoint "line-plane-intersection" '{"line": {"point": {"x": 0, "y": 0, "z": 0}, "direction": {"x": 0, "y": 0, "z": 1}}, "plane": {"point": {"x": 0, "y": 0, "z": 5}, "normal": {"x": 0, "y": 0, "z": 1}}}' "Line-Plane Intersection"
test_endpoint "plane-plane-intersection" '{"plane1": {"point": {"x": 0, "y": 0, "z": 0}, "normal": {"x": 1, "y": 0, "z": 0}}, "plane2": {"point": {"x": 0, "y": 0, "z": 0}, "normal": {"x": 0, "y": 1, "z": 0}}}' "Plane-Plane Intersection"

# Distance Operations
test_endpoint "point-line-distance" '{"point": {"x": 1, "y": 1, "z": 0}, "line": {"point": {"x": 0, "y": 0, "z": 0}, "direction": {"x": 1, "y": 0, "z": 0}}}' "Point to Line Distance"
test_endpoint "point-plane-distance" '{"point": {"x": 1, "y": 1, "z": 1}, "plane": {"point": {"x": 0, "y": 0, "z": 0}, "normal": {"x": 0, "y": 0, "z": 1}}}' "Point to Plane Distance"

# Matrix Operations
test_endpoint "matrix-vector-multiply" '{"matrix": [[1, 0, 0], [0, 1, 0], [0, 0, 1]], "vector": {"x": 1, "y": 2, "z": 3}}' "Matrix-Vector Multiplication"
test_endpoint "rotation-matrix" '{"axis": "z", "angle_degrees": 90}' "Rotation Matrix"
test_endpoint "arbitrary-rotation" '{"axis": {"x": 0, "y": 0, "z": 1}, "angle_radians": 1.5708, "point": {"x": 1, "y": 0, "z": 0}}' "Arbitrary Rotation"

# Quaternion Operations
test_endpoint "quaternion-from-axis-angle" '{"axis": {"x": 0, "y": 0, "z": 1}, "angle_radians": 1.5708}' "Quaternion from Axis-Angle"
test_endpoint "quaternion-multiply" '{"q1": {"w": 1, "x": 0, "y": 0, "z": 0}, "q2": {"w": 0.707, "x": 0, "y": 0, "z": 0.707}}' "Quaternion Multiplication"
test_endpoint "quaternion-slerp" '{"q1": {"w": 1, "x": 0, "y": 0, "z": 0}, "q2": {"w": 0.707, "x": 0, "y": 0, "z": 0.707}, "t": 0.5}' "Quaternion SLERP"

# Coordinate Conversions
test_endpoint "coordinate-conversion-three-d" '{"from_type": "cartesian", "to_type": "spherical", "coordinates": {"x": 1, "y": 1, "z": 1}}' "3D Coordinate Conversion"
test_endpoint "cartesian-to-spherical" '{"x": 1, "y": 1, "z": 1}' "Cartesian to Spherical"
test_endpoint "spherical-to-cartesian" '{"radius": 1.732, "theta": 0.785, "phi": 0.955}' "Spherical to Cartesian"
test_endpoint "cartesian-to-cylindrical" '{"x": 1, "y": 1, "z": 2}' "Cartesian to Cylindrical"
test_endpoint "cylindrical-to-cartesian" '{"radius": 1.414, "theta": 0.785, "z": 2}' "Cylindrical to Cartesian"

# Volume Calculations
test_endpoint "sphere-volume" '{"radius": 5}' "Sphere Volume"
test_endpoint "cylinder-volume" '{"radius": 3, "height": 10}' "Cylinder Volume"
test_endpoint "tetrahedron-volume" '{"vertices": [{"x": 0, "y": 0, "z": 0}, {"x": 1, "y": 0, "z": 0}, {"x": 0, "y": 1, "z": 0}, {"x": 0, "y": 0, "z": 1}]}' "Tetrahedron Volume"
test_endpoint "pyramid-volume" '{"base_area": 12, "height": 8}' "Pyramid Volume"
test_endpoint "aabb-volume" '{"min_point": {"x": 0, "y": 0, "z": 0}, "max_point": {"x": 2, "y": 3, "z": 4}}' "AABB Volume"

# Ray Intersections
test_endpoint "sphere-ray-intersection" '{"sphere": {"center": {"x": 0, "y": 0, "z": 0}, "radius": 1}, "ray": {"origin": {"x": -2, "y": 0, "z": 0}, "direction": {"x": 1, "y": 0, "z": 0}}}' "Sphere-Ray Intersection"
test_endpoint "sphere-sphere-intersection" '{"sphere1": {"center": {"x": 0, "y": 0, "z": 0}, "radius": 1}, "sphere2": {"center": {"x": 1.5, "y": 0, "z": 0}, "radius": 1}}' "Sphere-Sphere Intersection"
test_endpoint "cylinder-ray-intersection" '{"cylinder": {"center": {"x": 0, "y": 0, "z": 0}, "axis": {"x": 0, "y": 0, "z": 1}, "radius": 1, "height": 2}, "ray": {"origin": {"x": -2, "y": 0, "z": 1}, "direction": {"x": 1, "y": 0, "z": 0}}}' "Cylinder-Ray Intersection"
test_endpoint "ray-aabb-intersection" '{"aabb": {"min": {"x": 0, "y": 0, "z": 0}, "max": {"x": 1, "y": 1, "z": 1}}, "ray": {"origin": {"x": -1, "y": 0.5, "z": 0.5}, "direction": {"x": 1, "y": 0, "z": 0}}}' "Ray-AABB Intersection"

echo "=== STATISTICS TOOLS ==="
echo

# Statistical Analysis
test_endpoint "descriptive-statistics" '{"data": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]}' "Descriptive Statistics"
test_endpoint "summary-statistics" '{"data": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]}' "Summary Statistics"
test_endpoint "pearson-correlation" '{"x": [1, 2, 3, 4, 5], "y": [2, 4, 6, 8, 10]}' "Pearson Correlation"
test_endpoint "spearman-correlation" '{"x": [1, 2, 3, 4, 5], "y": [1, 4, 9, 16, 25]}' "Spearman Correlation"
test_endpoint "correlation-matrix" '{"variables": [{"name": "x", "values": [1, 2, 3]}, {"name": "y", "values": [2, 4, 6]}]}' "Correlation Matrix"

# Regression Analysis
test_endpoint "linear-regression" '{"x": [1, 2, 3, 4, 5], "y": [2, 4, 6, 8, 10]}' "Linear Regression"
test_endpoint "polynomial-regression" '{"x": [1, 2, 3, 4, 5], "y": [1, 4, 9, 16, 25], "degree": 2}' "Polynomial Regression"
test_endpoint "predict-values" '{"slope": 2, "intercept": 0, "x_values": [6, 7, 8]}' "Predict Values"

# Distribution Analysis
test_endpoint "histogram" '{"data": [1, 2, 2, 3, 3, 3, 4, 4, 5], "bins": 5}' "Histogram"
test_endpoint "test-normality" '{"data": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]}' "Normality Test"
test_endpoint "analyze-distribution" '{"data": [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]}' "Distribution Analysis"

echo "=== IDENTIFIERS & DATETIME ==="
echo

# Identifier Generation
test_endpoint "uuid-generator" '{"count": 3, "format": "simple"}' "UUID Generation"
test_endpoint "random-integer" '{"min": 1, "max": 100, "count": 5}' "Random Integer Generation"
test_endpoint "random-string" '{"length": 10, "character_set": "alphanumeric", "count": 3}' "Random String Generation"

# DateTime Operations
test_endpoint "current-datetime" '{"format": "iso8601", "timezone": "UTC"}' "Current DateTime"

echo "=== ENCODING TOOLS ==="
echo

# Base64 Operations
test_endpoint "base64-encoder" '{"input": "Hello, World!", "variant": "standard"}' "Base64 Encoding"
test_endpoint "base64-decoder" '{"input": "SGVsbG8sIFdvcmxkIQ==", "variant": "standard"}' "Base64 Decoding"

# Hex Operations
test_endpoint "hex-encoder" '{"input": "Hello, World!", "case": "lowercase"}' "Hex Encoding"
test_endpoint "hex-decoder" '{"input": "48656c6c6f2c20576f726c6421", "ignore_whitespace": true}' "Hex Decoding"

# URL Operations
test_endpoint "url-encoder" '{"input": "Hello, World! @#$%", "encode_type": "component"}' "URL Encoding"
test_endpoint "url-decoder" '{"input": "Hello%2C%20World%21%20%40%23%24%25"}' "URL Decoding"

echo "=== STRING MANIPULATION ==="
echo

# String Operations
test_endpoint "string-case-converter" '{"input": "Hello, World!", "target_case": "uppercase"}' "String Case Conversion"
test_endpoint "string-trimmer" '{"input": "  Hello, World!  ", "trim_type": "both"}' "String Trimming"
test_endpoint "string-splitter" '{"input": "apple,banana,cherry", "delimiter": ",", "max_splits": 3}' "String Splitting"

echo "=== DATA FORMAT TOOLS ==="
echo

# JSON Operations
test_endpoint "json-formatter" '{"input": "{\\"name\\":\\"John\\",\\"age\\":30}", "indent": 2, "sort_keys": true}' "JSON Formatting"
test_endpoint "json-validator" '{"input": "{\\"name\\":\\"John\\",\\"age\\":30}"}' "JSON Validation"

# CSV Operations
test_endpoint "csv-parser" '{"content": "name,age\\nJohn,30\\nJane,25", "has_headers": true, "delimiter": ","}' "CSV Parsing"

# YAML Operations
test_endpoint "yaml-formatter" '{"input": "name: John\\nage: 30", "indent": 2, "sort_keys": true}' "YAML Formatting"

echo "=== VALIDATION TOOLS ==="
echo

# Validation Operations
test_endpoint "email-validator" '{"email": "user@example.com"}' "Email Validation"
test_endpoint "url-validator" '{"url": "https://www.example.com/path?query=value#fragment"}' "URL Validation"
test_endpoint "regex-matcher" '{"text": "The quick brown fox", "pattern": "brown|red", "find_all": true}' "Regex Matching"

echo "=== CRYPTOGRAPHIC TOOLS ==="
echo

# Hash Operations
test_endpoint "hash-generator" '{"input": "Hello, World!", "algorithm": "sha256", "format": "hex"}' "Hash Generation"

echo
echo "=== TEST SUMMARY ==="
echo "Total Tests: $TOTAL_TESTS"
echo "Passed: $PASSED_TESTS"
echo "Failed: $FAILED_TESTS"
echo "Success Rate: $(( PASSED_TESTS * 100 / TOTAL_TESTS ))%"
echo

if [ $FAILED_TESTS -eq 0 ]; then
    echo "🎉 ALL TESTS PASSED! Core Tools HTTP endpoints are working correctly."
    exit 0
else
    echo "⚠️  $FAILED_TESTS tests failed. Please check the endpoints and data above."
    exit 1
fi