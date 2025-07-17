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

# Test Add
echo "--- Test: Add (10 + 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/add -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Subtract
echo "--- Test: Subtract (10 - 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/subtract -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Multiply
echo "--- Test: Multiply (10 * 3) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/multiply -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
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

# Test Remainder (was Modulo)
echo "--- Test: Remainder (10 % 3) - Rust % operator ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/remainder -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Modulus (mathematical modulus)
echo "--- Test: Modulus (10 mod 3) - Mathematical modulus ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/modulus -H "Content-Type: application/json" -d '{"a": 10, "b": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test negative numbers to show difference
echo "--- Test: Remainder vs Modulus (-21, 4) ---"
echo "Remainder (should be -1):"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/remainder -H "Content-Type: application/json" -d '{"a": -21, "b": 4}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo "Modulus (should be 3):"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/modulus -H "Content-Type: application/json" -d '{"a": -21, "b": 4}')
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

# Test Square Root
echo "--- Test: Square Root (sqrt(16)) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/sqrt -H "Content-Type: application/json" -d '{"value": 16}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Square
echo "--- Test: Square (4^2) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/square -H "Content-Type: application/json" -d '{"value": 4}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Pythagorean
echo "--- Test: Pythagorean (3, 4) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/pythagorean -H "Content-Type: application/json" -d '{"a": 3, "b": 4}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test Distance 2D
echo "--- Test: Distance 2D (0,0) to (3,4) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/distance-2d -H "Content-Type: application/json" -d '{"x1": 0, "y1": 0, "x2": 3, "y2": 4}')
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

# === URL ENCODING ===
echo "=== URL ENCODING ==="
echo

# Test URL encoder
echo "--- Test: URL Encode 'hello world' (component mode) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-encoder -H "Content-Type: application/json" -d '{"data": "hello world"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: URL Encode with special chars (query mode) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-encoder -H "Content-Type: application/json" -d '{"data": "name=John Doe&age=30", "mode": "query"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test URL decoder
echo "--- Test: URL Decode 'hello%20world' ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-decoder -H "Content-Type: application/json" -d '{"encoded": "hello%20world"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: URL Decode with plus signs ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-decoder -H "Content-Type: application/json" -d '{"encoded": "hello+world", "decode_plus": true}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === HEX ENCODING ===
echo "=== HEX ENCODING ==="
echo

# Test hex encoder
echo "--- Test: Hex Encode 'Hello' ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hex-encoder -H "Content-Type: application/json" -d '{"data": "Hello"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: Hex Encode uppercase ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hex-encoder -H "Content-Type: application/json" -d '{"data": "Test", "case": "uppercase"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test hex decoder
echo "--- Test: Hex Decode '48656c6c6f' ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hex-decoder -H "Content-Type: application/json" -d '{"encoded": "48656c6c6f"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === STRING OPERATIONS ===
echo "=== STRING OPERATIONS ==="
echo

# Test string case converter
echo "--- Test: Convert to snake_case ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-case-converter -H "Content-Type: application/json" -d '{"text": "HelloWorldFromRust", "target_case": "snake_case"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "--- Test: Convert to camelCase ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-case-converter -H "Content-Type: application/json" -d '{"text": "hello_world_from_rust", "target_case": "camelCase"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === STRING TRIMMER ===
echo "=== STRING TRIMMER ==="
echo

# Test basic trim
echo "--- Test: Basic trim whitespace ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-trimmer -H "Content-Type: application/json" -d '{"text": "  hello world  "}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test trim character
echo "--- Test: Trim specific character ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-trimmer -H "Content-Type: application/json" -d '{"text": "---hello---", "operation": "trim_char", "char_to_trim": "-"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test padding
echo "--- Test: Pad right with asterisks ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-trimmer -H "Content-Type: application/json" -d '{"text": "hello", "operation": "pad_right", "pad_length": 10, "pad_char": "*"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === STRING SPLITTER ===
echo "=== STRING SPLITTER ==="
echo

# Test basic string split
echo "--- Test: Split comma-separated values ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-splitter -H "Content-Type: application/json" -d '{"text": "apple,banana,cherry", "delimiter": ","}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test whitespace split
echo "--- Test: Split by whitespace ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-splitter -H "Content-Type: application/json" -d '{"text": "hello   world  from    rust", "split_type": "whitespace"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test regex split
echo "--- Test: Split by regex (digits) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-splitter -H "Content-Type: application/json" -d '{"text": "one1two2three3four", "delimiter": "\\d+", "split_type": "regex"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test split with limit
echo "--- Test: Split with limit ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/string-splitter -H "Content-Type: application/json" -d '{"text": "a-b-c-d-e", "delimiter": "-", "limit": 3}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === JSON FORMATTER ===
echo "=== JSON FORMATTER ==="
echo

# Test basic formatting
echo "--- Test: Pretty format JSON ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/json-formatter -H "Content-Type: application/json" -d '{"json_string": "{\"name\":\"John\",\"age\":30,\"city\":\"New York\"}"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test compact formatting
echo "--- Test: Compact format JSON ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/json-formatter -H "Content-Type: application/json" -d '{"json_string": "{\n  \"name\": \"John\",\n  \"age\": 30\n}", "indent": 0}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test invalid JSON
echo "--- Test: Invalid JSON ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/json-formatter -H "Content-Type: application/json" -d '{"json_string": "{\"name\": \"John\", \"age\": }"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === JSON VALIDATOR ===
echo "=== JSON VALIDATOR ==="
echo

# Test valid JSON
echo "--- Test: Valid JSON ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/json-validator -H "Content-Type: application/json" -d '{"json_string": "{\"name\":\"John\",\"age\":30}"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test invalid JSON
echo "--- Test: Invalid JSON ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/json-validator -H "Content-Type: application/json" -d '{"json_string": "{\"name\": \"John\", \"age\": }"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === EMAIL VALIDATOR ===
echo "=== EMAIL VALIDATOR ==="
echo

# Test valid email
echo "--- Test: Valid email ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/email-validator -H "Content-Type: application/json" -d '{"email": "test@example.com"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test invalid email (no @)
echo "--- Test: Invalid email (no @) ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/email-validator -H "Content-Type: application/json" -d '{"email": "testexample.com"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test email with subdomain
echo "--- Test: Email with subdomain ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/email-validator -H "Content-Type: application/json" -d '{"email": "user@mail.example.com"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === HASH GENERATOR ===
echo "=== HASH GENERATOR ==="
echo

# Test SHA256 hex
echo "--- Test: SHA256 hex ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hash-generator -H "Content-Type: application/json" -d '{"text": "hello world", "algorithm": "sha256"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test MD5 base64
echo "--- Test: MD5 base64 ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hash-generator -H "Content-Type: application/json" -d '{"text": "hello world", "algorithm": "md5", "format": "base64"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test invalid algorithm
echo "--- Test: Invalid algorithm ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/hash-generator -H "Content-Type: application/json" -d '{"text": "test", "algorithm": "sha1"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === URL VALIDATOR TOOL ===
echo "=== URL VALIDATOR TOOL ==="
echo

# Test valid URL
echo "--- Test: Valid URL ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-validator -H "Content-Type: application/json" -d '{"url": "https://example.com/path?query=value"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test invalid URL
echo "--- Test: Invalid URL ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/url-validator -H "Content-Type: application/json" -d '{"url": "not a url"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === REGEX MATCHER TOOL ===
echo "=== REGEX MATCHER TOOL ==="
echo

# Test regex match
echo "--- Test: Regex match ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/regex-matcher -H "Content-Type: application/json" -d '{"text": "The quick brown fox", "pattern": "\\\\b\\\\w{5}\\\\b", "find_all": true}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test no match
echo "--- Test: No match ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/regex-matcher -H "Content-Type: application/json" -d '{"text": "hello world", "pattern": "xyz"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === CSV PARSER TOOL ===
echo "=== CSV PARSER TOOL ==="
echo

# Test CSV parsing
echo "--- Test: CSV parsing ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/csv-parser -H "Content-Type: application/json" -d '{"content": "Name,Age,City\\nJohn,30,New York\\nJane,25,Boston", "has_headers": true}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test CSV with custom delimiter
echo "--- Test: CSV custom delimiter ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/csv-parser -H "Content-Type: application/json" -d '{"content": "Name|Age|City\\nJohn|30|New York", "has_headers": true, "delimiter": "|"}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# === YAML FORMATTER TOOL ===
echo "=== YAML FORMATTER TOOL ==="
echo

# Test YAML formatting
echo "--- Test: YAML formatting ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/yaml-formatter -H "Content-Type: application/json" -d '{"content": "name: John\\nage: 30\\ncity: New York", "sort_keys": true}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

# Test YAML validation
echo "--- Test: YAML validation ---"
response=$(curl -s -w "HTTP_CODE:%{http_code}" -X POST $BASE_URL/yaml-formatter -H "Content-Type: application/json" -d '{"content": "invalid: yaml: content:", "validate_only": true}')
http_code=$(echo "$response" | grep -o "HTTP_CODE:[0-9]*" | cut -d: -f2)
response_body=$(echo "$response" | sed 's/HTTP_CODE:[0-9]*$//')
echo "Response: $response_body"
echo

echo "=== Test Suite Complete ==="