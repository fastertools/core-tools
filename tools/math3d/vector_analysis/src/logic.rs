use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct VectorAnalysisInput {
    pub vector_a: Vec<f64>,
    pub vector_b: Vec<f64>,
}

#[derive(Debug, Serialize)]
pub struct VectorAnalysisOutput {
    pub magnitude_a: f64,
    pub magnitude_b: f64,
    pub angle_between_radians: f64,
    pub angle_between_degrees: f64,
    pub dot_product: f64,
    pub cross_product: Vec<f64>,
    pub is_orthogonal: bool,
    pub is_parallel: bool,
    pub vector_similarity: f64,
}

pub async fn analyze_vectors(input: VectorAnalysisInput) -> Result<VectorAnalysisOutput, String> {
    // Validate input vectors
    if input.vector_a.len() != 3 || input.vector_b.len() != 3 {
        return Err("Both vectors must be 3-dimensional".to_string());
    }

    // Call atomic tools via HTTP
    let magnitude_a = call_vector_magnitude(&input.vector_a).await?;
    let magnitude_b = call_vector_magnitude(&input.vector_b).await?;
    let angle_result = call_vector_angle(&input.vector_a, &input.vector_b).await?;
    let dot_product = call_dot_product(&input.vector_a, &input.vector_b).await?;
    let cross_product = call_cross_product(&input.vector_a, &input.vector_b).await?;

    // Calculate derived properties
    let is_orthogonal = (dot_product.abs() < 1e-10);
    let is_parallel = (cross_product.iter().all(|&x| x.abs() < 1e-10));
    let vector_similarity = if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    };

    Ok(VectorAnalysisOutput {
        magnitude_a,
        magnitude_b,
        angle_between_radians: angle_result,
        angle_between_degrees: angle_result * 180.0 / std::f64::consts::PI,
        dot_product,
        cross_product,
        is_orthogonal,
        is_parallel,
        vector_similarity,
    })
}

async fn call_vector_magnitude(vector: &[f64]) -> Result<f64, String> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "vector": vector
    });

    let response = client
        .post("http://spin.internal/vector-magnitude")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call vector_magnitude: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("vector_magnitude returned error: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse vector_magnitude response: {}", e))?;

    result["magnitude"]
        .as_f64()
        .ok_or_else(|| "Invalid magnitude response".to_string())
}

async fn call_vector_angle(vector_a: &[f64], vector_b: &[f64]) -> Result<f64, String> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "vector_a": vector_a,
        "vector_b": vector_b
    });

    let response = client
        .post("http://spin.internal/vector-angle")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call vector_angle: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("vector_angle returned error: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse vector_angle response: {}", e))?;

    result["angle_radians"]
        .as_f64()
        .ok_or_else(|| "Invalid angle response".to_string())
}

async fn call_dot_product(vector_a: &[f64], vector_b: &[f64]) -> Result<f64, String> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "vector_a": vector_a,
        "vector_b": vector_b
    });

    let response = client
        .post("http://spin.internal/dot-product")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call dot_product: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("dot_product returned error: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse dot_product response: {}", e))?;

    result["dot_product"]
        .as_f64()
        .ok_or_else(|| "Invalid dot_product response".to_string())
}

async fn call_cross_product(vector_a: &[f64], vector_b: &[f64]) -> Result<Vec<f64>, String> {
    let client = reqwest::Client::new();
    let payload = serde_json::json!({
        "vector_a": vector_a,
        "vector_b": vector_b
    });

    let response = client
        .post("http://spin.internal/cross-product")
        .json(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to call cross_product: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("cross_product returned error: {}", response.status()));
    }

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse cross_product response: {}", e))?;

    result["cross_product"]
        .as_array()
        .and_then(|arr| {
            arr.iter()
                .map(|v| v.as_f64())
                .collect::<Option<Vec<f64>>>()
        })
        .ok_or_else(|| "Invalid cross_product response".to_string())
}