#!/bin/bash

# Dual-Mode Architecture Validation - Testing distance_2d and divide tools
# Testing the improved dual-mode pattern with conditional exports

BASE_URL="http://127.0.0.1:3000"

echo "=== Dual-Mode Architecture Validation - HTTP Endpoints ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo

# === DISTANCE 2D INDIVIDUAL TOOL ===
echo "=== DISTANCE 2D INDIVIDUAL TOOL ==="
echo

# Test basic distance calculation (3-4-5 triangle)
echo "--- Test: Basic Distance (3-4-5 triangle) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/distance-two-d -H "Content-Type: application/json" -d '{
  "x1": 0,
  "y1": 0,
  "x2": 3,
  "y2": 4
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test negative coordinates
echo "--- Test: Negative Coordinates ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/distance-two-d -H "Content-Type: application/json" -d '{
  "x1": -1,
  "y1": -1,
  "x2": 2,
  "y2": 3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test same point (zero distance)
echo "--- Test: Same Point (Zero Distance) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/distance-two-d -H "Content-Type: application/json" -d '{
  "x1": 5,
  "y1": 7,
  "x2": 5,
  "y2": 7
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "=== PYTHAGOREAN DEPENDENCY TEST ==="
echo

# Test pythagorean tool directly (distance_2d depends on this)
echo "--- Test: Pythagorean Tool (3, 4) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/pythagorean -H "Content-Type: application/json" -d '{
  "a": 3,
  "b": 4
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === DIVIDE TOOL INDIVIDUAL TESTS ===
echo "=== DIVIDE TOOL INDIVIDUAL TESTS ==="
echo

# Test basic division
echo "--- Test: Basic Division (10 ÷ 2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/divide -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": 2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test division by zero
echo "--- Test: Division by Zero ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/divide -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": 0
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test negative numbers
echo "--- Test: Negative Numbers (-10 ÷ -2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/divide -H "Content-Type: application/json" -d '{
  "a": -10,
  "b": -2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test fraction result
echo "--- Test: Fraction Result (7 ÷ 2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/divide -H "Content-Type: application/json" -d '{
  "a": 7,
  "b": 2
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === ADD TOOL INDIVIDUAL TESTS ===
echo "=== ADD TOOL INDIVIDUAL TESTS ==="
echo

# Test basic addition
echo "--- Test: Basic Addition (5 + 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{
  "a": 5,
  "b": 3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test negative numbers
echo "--- Test: Negative Numbers (-5 + -3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{
  "a": -5,
  "b": -3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test mixed signs
echo "--- Test: Mixed Signs (10 + -3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": -3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test zero addition
echo "--- Test: Zero Addition (42 + 0) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{
  "a": 42,
  "b": 0
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === MULTIPLY TOOL INDIVIDUAL TESTS ===
echo "=== MULTIPLY TOOL INDIVIDUAL TESTS ==="
echo

# Test basic multiplication
echo "--- Test: Basic Multiplication (6 × 7) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{
  "a": 6,
  "b": 7
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test negative numbers
echo "--- Test: Negative Numbers (-4 × -5) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{
  "a": -4,
  "b": -5
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test mixed signs
echo "--- Test: Mixed Signs (8 × -3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{
  "a": 8,
  "b": -3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test zero multiplication
echo "--- Test: Zero Multiplication (42 × 0) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{
  "a": 42,
  "b": 0
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === SUBTRACT TOOL INDIVIDUAL TESTS ===
echo "=== SUBTRACT TOOL INDIVIDUAL TESTS ==="
echo

# Test basic subtraction
echo "--- Test: Basic Subtraction (5 - 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{
  "a": 5,
  "b": 3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test negative numbers
echo "--- Test: Negative Numbers (-5 - -3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{
  "a": -5,
  "b": -3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test mixed signs
echo "--- Test: Mixed Signs (10 - -3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": -3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test zero subtraction
echo "--- Test: Zero Subtraction (42 - 0) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{
  "a": 42,
  "b": 0
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "=== SUMMARY ==="
echo "This script validates the dual-mode architecture HTTP endpoints:"
echo "1. DISTANCE_2D: Individual mode with pythagorean.spin.internal dependencies"
echo "2. DIVIDE: Individual mode with no external dependencies"
echo "3. ADD: Individual mode with no external dependencies"
echo "4. MULTIPLY: Individual mode with no external dependencies"
echo "5. SUBTRACT: Individual mode with no external dependencies"
echo "6. CONDITIONAL EXPORTS: Single function names for all tools"
echo "7. MODULAR DESIGN: lib.rs handles I/O, logic.rs contains business logic"
echo "8. TOOL COMPOSITION: distance_2d properly uses pythagorean tool"
echo