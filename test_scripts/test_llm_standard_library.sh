#!/bin/bash

# Test script for LLM Standard Library tools
# Uses curl.sh for testing according to project rules

echo "Testing LLM Standard Library tools..."
echo

# Test UUID Generator
echo "=== UUID Generator Tests ==="
echo "Testing single UUID generation:"
./curl.sh POST http://localhost:3000/uuid-generator \
  -H "Content-Type: application/json" \
  -d '{}'
echo

echo "Testing multiple UUIDs (count: 3):"
./curl.sh POST http://localhost:3000/uuid-generator \
  -H "Content-Type: application/json" \
  -d '{"count": 3}'
echo

echo "Testing UUID with simple format:"
./curl.sh POST http://localhost:3000/uuid-generator \
  -H "Content-Type: application/json" \
  -d '{"count": 1, "format": "simple"}'
echo

echo "Testing UUID with URN format:"
./curl.sh POST http://localhost:3000/uuid-generator \
  -H "Content-Type: application/json" \
  -d '{"count": 1, "format": "urn"}'
echo

# Test Current DateTime
echo "=== Current DateTime Tests ==="
echo "Testing current datetime (UTC default):"
./curl.sh POST http://localhost:3000/current-datetime \
  -H "Content-Type: application/json" \
  -d '{}'
echo

echo "Testing with timezone offset (+05:30):"
./curl.sh POST http://localhost:3000/current-datetime \
  -H "Content-Type: application/json" \
  -d '{"timezone": "+05:30"}'
echo

echo "Testing with negative timezone offset (-08:00):"
./curl.sh POST http://localhost:3000/current-datetime \
  -H "Content-Type: application/json" \
  -d '{"timezone": "-08:00"}'
echo

# Test Base64 Encoder
echo "=== Base64 Encoder Tests ==="
echo "Testing basic encoding:"
./curl.sh POST http://localhost:3000/base64-encoder \
  -H "Content-Type: application/json" \
  -d '{"data": "Hello, World!"}'
echo

echo "Testing URL-safe encoding:"
./curl.sh POST http://localhost:3000/base64-encoder \
  -H "Content-Type: application/json" \
  -d '{"data": "Hello??>>", "variant": "url_safe"}'
echo

echo "Testing no-padding encoding:"
./curl.sh POST http://localhost:3000/base64-encoder \
  -H "Content-Type: application/json" \
  -d '{"data": "Test data", "variant": "standard_no_pad"}'
echo

# Test Base64 Decoder
echo "=== Base64 Decoder Tests ==="
echo "Testing basic decoding:"
./curl.sh POST http://localhost:3000/base64-decoder \
  -H "Content-Type: application/json" \
  -d '{"encoded": "SGVsbG8sIFdvcmxkIQ=="}'
echo

echo "Testing decoding with whitespace:"
./curl.sh POST http://localhost:3000/base64-decoder \
  -H "Content-Type: application/json" \
  -d '{"encoded": "SGVs bG8s\nIFdv cmxk IQ=="}'
echo

echo "Testing URL-safe decoding:"
./curl.sh POST http://localhost:3000/base64-decoder \
  -H "Content-Type: application/json" \
  -d '{"encoded": "Pz8-Pg", "variant": "url_safe_no_pad"}'
echo

# Test round-trip encoding/decoding
echo "=== Round-trip Test ==="
echo "Encoding 'The quick brown fox':"
ENCODED=$(./curl.sh POST http://localhost:3000/base64-encoder \
  -H "Content-Type: application/json" \
  -d '{"data": "The quick brown fox"}' 2>/dev/null | jq -r '.encoded')
echo "Encoded: $ENCODED"

echo "Decoding the result:"
./curl.sh POST http://localhost:3000/base64-decoder \
  -H "Content-Type: application/json" \
  -d "{\"encoded\": \"$ENCODED\"}"
echo

echo "All LLM Standard Library tool tests completed!"