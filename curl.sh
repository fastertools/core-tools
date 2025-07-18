#!/bin/bash

# Dual-Mode Tool Deployment System - Category Component Testing
# Testing only the basic-math-category component endpoint

BASE_URL="http://127.0.0.1:3000"

echo "=== Dual-Mode Tool Deployment System - Category Component Testing ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo

# === BASIC MATH CATEGORY COMPONENT ===
echo "=== BASIC MATH CATEGORY COMPONENT ==="
echo

# Test add operation
echo "--- Test: Add Operation (5 + 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/basic-math-category -H "Content-Type: application/json" -d '{
  "operation": "add",
  "operands": [5, 3]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test subtract operation
echo "--- Test: Subtract Operation (10 - 4) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/basic-math-category -H "Content-Type: application/json" -d '{
  "operation": "subtract",
  "operands": [10, 4]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test multiply operation
echo "--- Test: Multiply Operation (6 * 7) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/basic-math-category -H "Content-Type: application/json" -d '{
  "operation": "multiply",
  "operands": [6, 7]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test invalid operation
echo "--- Test: Invalid Operation (divide - not implemented) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/basic-math-category -H "Content-Type: application/json" -d '{
  "operation": "divide",
  "operands": [10, 2]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# Test error case (wrong number of operands)
echo "--- Test: Error Case (add with 1 operand instead of 2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/basic-math-category -H "Content-Type: application/json" -d '{
  "operation": "add",
  "operands": [5]
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "=== SUMMARY ==="
echo "This script tests the basic-math-category component demonstrating:"
echo "1. Zero HTTP overhead - direct function calls instead of HTTP requests"
echo "2. Unified API - single endpoint for multiple operations"
echo "3. Standardized types - consistent input/output formats"
echo "4. Error handling - proper validation and error responses"
echo