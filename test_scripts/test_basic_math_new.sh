#!/bin/bash

# Test script for new basic math operations
# Uses curl.sh for testing according to project rules

echo "Testing new basic math operations..."
echo

# Test subtract
echo "Testing subtract (10 - 3):"
./curl.sh POST http://localhost:3000/subtract \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 3}'
echo

echo "Testing subtract with negative result (3 - 5):"
./curl.sh POST http://localhost:3000/subtract \
  -H "Content-Type: application/json" \
  -d '{"a": 3, "b": 5}'
echo

# Test divide
echo "Testing divide (10 / 2):"
./curl.sh POST http://localhost:3000/divide \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 2}'
echo

echo "Testing divide with fraction result (7 / 2):"
./curl.sh POST http://localhost:3000/divide \
  -H "Content-Type: application/json" \
  -d '{"a": 7, "b": 2}'
echo

echo "Testing divide by zero (should error):"
./curl.sh POST http://localhost:3000/divide \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 0}'
echo

# Test modulo
echo "Testing modulo (10 % 3):"
./curl.sh POST http://localhost:3000/modulo \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 3}'
echo

echo "Testing modulo with exact division (12 % 4):"
./curl.sh POST http://localhost:3000/modulo \
  -H "Content-Type: application/json" \
  -d '{"a": 12, "b": 4}'
echo

echo "Testing modulo with negative dividend (-10 % 3):"
./curl.sh POST http://localhost:3000/modulo \
  -H "Content-Type: application/json" \
  -d '{"a": -10, "b": 3}'
echo

# Test power
echo "Testing power (2^3):"
./curl.sh POST http://localhost:3000/power \
  -H "Content-Type: application/json" \
  -d '{"a": 2, "b": 3}'
echo

echo "Testing square root via power (4^0.5):"
./curl.sh POST http://localhost:3000/power \
  -H "Content-Type: application/json" \
  -d '{"a": 4, "b": 0.5}'
echo

echo "Testing negative exponent (2^-3):"
./curl.sh POST http://localhost:3000/power \
  -H "Content-Type: application/json" \
  -d '{"a": 2, "b": -3}'
echo

echo "Testing 0^0 (should error):"
./curl.sh POST http://localhost:3000/power \
  -H "Content-Type: application/json" \
  -d '{"a": 0, "b": 0}'
echo

echo "All tests completed!"