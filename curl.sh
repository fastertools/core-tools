#!/bin/bash

# Core Tools Testing Script - LLM Standard Library Tools
# Testing all new tools: basic math, uuid, datetime, base64

BASE_URL="http://127.0.0.1:3000"

echo "=== Core Tools LLM Standard Library Test Suite ==="
echo "Base URL: $BASE_URL"
echo "Date: $(date)"
echo

# === BASIC MATH TOOLS ===
echo "=== BASIC MATH TOOLS ==="
echo

# Test Subtract
echo "--- Test: Subtract (10 - 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Divide  
echo "--- Test: Divide (10 / 2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/divide -H "Content-Type: application/json" -d '{"a": 10, "b": 2}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Modulo
echo "--- Test: Modulo (10 % 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/modulo -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Power
echo "--- Test: Power (2^3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/power -H "Content-Type: application/json" -d '{"a": 2, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === UUID GENERATOR ===
echo "=== UUID GENERATOR ==="
echo

# Test single UUID
echo "--- Test: Generate Single UUID ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/uuid-generator -H "Content-Type: application/json" -d '{}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test multiple UUIDs
echo "--- Test: Generate 3 UUIDs ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/uuid-generator -H "Content-Type: application/json" -d '{"count": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === CURRENT DATETIME ===
echo "=== CURRENT DATETIME ==="
echo

# Test UTC datetime
echo "--- Test: Current DateTime (UTC) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/current-datetime -H "Content-Type: application/json" -d '{}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test with timezone offset
echo "--- Test: Current DateTime (+05:30) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/current-datetime -H "Content-Type: application/json" -d '{"timezone": "+05:30"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === BASE64 ENCODING ===
echo "=== BASE64 ENCODING ==="
echo

# Test encode
echo "--- Test: Base64 Encode 'Hello, World!' ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/base64-encoder -H "Content-Type: application/json" -d '{"data": "Hello, World!"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test decode
echo "--- Test: Base64 Decode 'SGVsbG8sIFdvcmxkIQ==' ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/base64-decoder -H "Content-Type: application/json" -d '{"encoded": "SGVsbG8sIFdvcmxkIQ=="}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === RANDOM GENERATORS ===
echo "=== RANDOM GENERATORS ==="
echo

# Test random integer
echo "--- Test: Random Integer (default range 0-100) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/random-integer -H "Content-Type: application/json" -d '{}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: Random Integer (range 10-20, count 5) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/random-integer -H "Content-Type: application/json" -d '{"min": 10, "max": 20, "count": 5}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test random string
echo "--- Test: Random String (default alphanumeric, 16 chars) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/random-string -H "Content-Type: application/json" -d '{}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: Random String (hex charset, 32 chars, count 2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/random-string -H "Content-Type: application/json" -d '{"length": 32, "charset": "hex", "count": 2}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "=== Test Suite Complete ==="