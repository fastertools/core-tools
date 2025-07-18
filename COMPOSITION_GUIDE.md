# HTTP Tool Composition Guide

## Overview

This guide demonstrates the HTTP-based composition pattern used in the Core Tools project for building complex operations from atomic tools.

## Composition Pattern

### Atomic vs Composite Tools

**Atomic Tools**: Single-purpose tools that perform one specific calculation
- `vector_magnitude`: Calculates the magnitude of a vector
- `vector_angle`: Calculates the angle between two vectors
- `dot_product`: Computes the dot product of two vectors
- `cross_product`: Computes the cross product of two vectors

**Composite Tools**: Complex operations that combine multiple atomic tools via HTTP calls
- `vector_analysis`: Performs comprehensive vector analysis using multiple atomic tools

### Example: Vector Analysis Composite Tool

The `vector_analysis` tool demonstrates proper HTTP composition:

```rust
// HTTP composition pattern - async calls to atomic tools
let magnitude_a = call_vector_magnitude(&input.vector_a).await?;
let magnitude_b = call_vector_magnitude(&input.vector_b).await?;
let angle = call_vector_angle(&input.vector_a, &input.vector_b).await?;
let dot_product = call_dot_product(&input.vector_a, &input.vector_b).await?;
let cross_product = call_cross_product(&input.vector_a, &input.vector_b).await?;
```

## Benefits of Composition

1. **Single Responsibility**: Each tool has one clear purpose
2. **Modularity**: Tools can be used individually or in combination
3. **Reusability**: Atomic tools can be reused in multiple compositions
4. **Testability**: Each component can be tested independently
5. **Maintainability**: Changes to individual tools don't affect others

## Implementation Guidelines

### HTTP Error Handling

Always handle HTTP errors gracefully:

```rust
async fn call_vector_magnitude(vector: &[f64]) -> Result<f64, String> {
    let response = reqwest::Client::new()
        .post("http://localhost:8000/vector-magnitude")
        .json(&VectorMagnitudeInput { vector: vector.to_vec() })
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }
    
    let result: VectorMagnitudeOutput = response.json().await
        .map_err(|e| format!("JSON parsing failed: {}", e))?;
    
    Ok(result.magnitude)
}
```

### Error Aggregation

When multiple HTTP calls fail, aggregate errors meaningfully:

```rust
let mut errors = Vec::new();

match call_vector_magnitude(&input.vector_a).await {
    Ok(mag) => magnitude_a = mag,
    Err(e) => errors.push(format!("Vector A magnitude: {}", e)),
}

if !errors.is_empty() {
    return ToolResponse::text(format!("Errors: {}", errors.join(", ")));
}
```

### Performance Considerations

- Use `reqwest::Client` for HTTP calls
- Consider parallel execution where possible
- Handle timeouts appropriately
- Cache client instances when beneficial

## Best Practices

1. **Fail Early**: If a critical calculation fails, return immediately
2. **Clear Error Messages**: Provide specific error context
3. **Consistent APIs**: Use standard input/output patterns
4. **Resource Management**: Properly manage HTTP client resources
5. **Documentation**: Document composition chains clearly

## Testing Composite Tools

Test both individual components and the composition:

```bash
# Test atomic tools individually
curl -X POST http://localhost:8000/vector-magnitude \
  -H "Content-Type: application/json" \
  -d '{"vector": [3.0, 4.0, 5.0]}'

# Test composite tool
curl -X POST http://localhost:8000/vector-analysis \
  -H "Content-Type: application/json" \
  -d '{"vector_a": [1.0, 2.0, 3.0], "vector_b": [4.0, 5.0, 6.0]}'
```

## When to Use Composition

- Complex operations requiring multiple calculations
- When the combined result is more valuable than individual parts
- When you need to maintain atomic tool independence
- When building domain-specific higher-level APIs

## When NOT to Use Composition

- Simple operations that can be done in a single tool
- When HTTP overhead outweighs the benefits
- When tight coupling between operations is required
- When performance is critical and milliseconds matter

## Future Enhancements

Potential improvements to the composition pattern:
- **Parallel Execution**: Execute independent calculations concurrently
- **Caching**: Cache intermediate results for repeated operations
- **Circuit Breakers**: Add resilience patterns for HTTP calls
- **Batch Operations**: Group multiple calculations into single HTTP calls