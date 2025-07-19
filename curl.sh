#!/bin/bash

# Code Quality and Architecture Cleanup Initiative - Critical Tools Testing
# Testing the 5 critical tools that were fixed for anti-patterns

BASE_URL="http://127.0.0.1:3000"

echo "=== Code Quality Cleanup Initiative - Critical Tools Testing ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo

# === DISTANCE_2D TOOL ===
echo "=== DISTANCE_2D TOOL (Fixed: removed unused functions, now uses logic.rs) ==="
echo

echo "--- Test: Distance 2D (Pythagorean distance calculation) ---"
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

# === PYTHAGOREAN TOOL ===
echo "=== PYTHAGOREAN TOOL (Fixed: removed HTTP composition, eliminated unused function) ==="
echo

echo "--- Test: Pythagorean (Calculate hypotenuse from two legs) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/pythagorean -H "Content-Type: application/json" -d '{
  "a": 3,
  "b": 4
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === ADD TOOL ===
echo "=== ADD TOOL (Fixed: removed WASM dependencies, now uses logic.rs) ==="
echo

echo "--- Test: Add (Simple addition) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{
  "a": 7,
  "b": 8
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === MULTIPLY TOOL ===
echo "=== MULTIPLY TOOL (Fixed: removed unused functions, now uses logic.rs) ==="
echo

echo "--- Test: Multiply (Simple multiplication) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{
  "a": 6,
  "b": 7
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

# === SUBTRACT TOOL ===
echo "=== SUBTRACT TOOL (Fixed: removed unused functions, now uses logic.rs) ==="
echo

echo "--- Test: Subtract (Simple subtraction) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{
  "a": 10,
  "b": 3
}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "HTTP Code: $http_code"
echo "Response: $response_body"
echo

echo "=== SUMMARY ==="
echo "This script tests the 5 critical tools fixed in Code Quality Cleanup Initiative:"
echo "1. distance-two-d (removed dead files, unused functions, now uses logic.rs)"
echo "2. pythagorean (removed HTTP composition, eliminated unused function)"
echo "3. add (removed WASM dependencies, now properly uses logic.rs)" 
echo "4. multiply (removed unused functions, now properly uses logic.rs)"
echo "5. subtract (removed unused functions, now properly uses logic.rs)"
echo "All tools should return HTTP 200 and valid JSON responses."
echo