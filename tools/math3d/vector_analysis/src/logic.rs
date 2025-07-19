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

// Helper structs for calling other tools
#[derive(Serialize, Deserialize)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize)]
struct VectorInput {
    vector: Vector3D,
}

#[derive(Serialize)]
struct TwoVectorInput {
    vector1: Vector3D,
    vector2: Vector3D,
}

#[derive(Deserialize)]
struct MagnitudeResult {
    magnitude: f64,
    unit_vector: Vector3D,
    is_zero_vector: bool,
}

#[derive(Deserialize)]
struct AngleResult {
    angle_radians: f64,
    angle_degrees: f64,
    cos_angle: f64,
    vector1_magnitude: f64,
    vector2_magnitude: f64,
    is_perpendicular: bool,
    is_parallel: bool,
}

#[derive(Deserialize)]
struct DotProductResult {
    dot_product: f64,
    angle_radians: f64,
    angle_degrees: f64,
    are_perpendicular: bool,
    are_parallel: bool,
}

#[derive(Deserialize)]
struct CrossProductResult {
    cross_product: CrossProductVector,
    magnitude: f64,
    area_parallelogram: f64,
    are_parallel: bool,
}

#[derive(Deserialize)]
struct CrossProductVector {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Deserialize)]
struct ToolResponseWrapper<T> {
    content: Vec<ContentItem<T>>,
}

#[derive(Deserialize)]
struct ContentItem<T> {
    #[serde(rename = "type")]
    item_type: String,
    text: String,
    #[serde(skip)]
    _phantom: std::marker::PhantomData<T>,
}

pub async fn analyze_vectors(input: VectorAnalysisInput) -> Result<VectorAnalysisOutput, String> {
    // Validate input vectors
    if input.vector_a.len() != 3 || input.vector_b.len() != 3 {
        return Err("Both vectors must be 3-dimensional".to_string());
    }

    // Call atomic tools via Spin HTTP
    let magnitude_a = call_vector_magnitude(&input.vector_a).await?;
    let magnitude_b = call_vector_magnitude(&input.vector_b).await?;
    let angle_result = call_vector_angle(&input.vector_a, &input.vector_b).await?;
    let dot_product = call_dot_product(&input.vector_a, &input.vector_b).await?;
    let cross_product = call_cross_product(&input.vector_a, &input.vector_b).await?;

    // Calculate derived properties
    let is_orthogonal = dot_product.abs() < 1e-10;
    let is_parallel = cross_product.iter().all(|&x| x.abs() < 1e-10);
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
    use spin_sdk::http::{Method, Request};

    if vector.len() != 3 {
        return Err("Vector must be 3-dimensional".to_string());
    }

    let input = VectorInput {
        vector: Vector3D {
            x: vector[0],
            y: vector[1],
            z: vector[2],
        },
    };
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize vector input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://vector-magnitude.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Failed to call vector_magnitude: {e:?}"))?;

    let body_bytes = response.into_body();
    let body = String::from_utf8(body_bytes)
        .map_err(|e| format!("Failed to parse response body: {e}"))?;

    // Parse direct ToolResponse format like pythagorean does
    let wrapper: ToolResponseWrapper<String> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse response wrapper: {e}"))?;

    let result_text = &wrapper.content[0].text;
    let result: MagnitudeResult = serde_json::from_str(result_text)
        .map_err(|e| format!("Failed to parse magnitude result: {e}"))?;

    Ok(result.magnitude)
}

async fn call_vector_angle(vector_a: &[f64], vector_b: &[f64]) -> Result<f64, String> {
    use spin_sdk::http::{Method, Request};

    if vector_a.len() != 3 || vector_b.len() != 3 {
        return Err("Vectors must be 3-dimensional".to_string());
    }

    let input = TwoVectorInput {
        vector1: Vector3D {
            x: vector_a[0],
            y: vector_a[1],
            z: vector_a[2],
        },
        vector2: Vector3D {
            x: vector_b[0],
            y: vector_b[1],
            z: vector_b[2],
        },
    };
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize vector angle input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://vector-angle.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Failed to call vector_angle: {e:?}"))?;

    let body_bytes = response.into_body();
    let body = String::from_utf8(body_bytes)
        .map_err(|e| format!("Failed to parse response body: {e}"))?;

    let wrapper: ToolResponseWrapper<String> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse response wrapper: {e}"))?;

    let result_text = &wrapper.content[0].text;
    let result: AngleResult = serde_json::from_str(result_text).map_err(|e| {
        format!(
            "Failed to parse angle result: {}. Response body: {}",
            e, body
        )
    })?;

    Ok(result.angle_radians)
}

async fn call_dot_product(vector_a: &[f64], vector_b: &[f64]) -> Result<f64, String> {
    use spin_sdk::http::{Method, Request};

    let input = TwoVectorInput {
        vector1: Vector3D {
            x: vector_a[0],
            y: vector_a[1],
            z: vector_a[2],
        },
        vector2: Vector3D {
            x: vector_b[0],
            y: vector_b[1],
            z: vector_b[2],
        },
    };
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize dot product input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://dot-product.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Failed to call dot_product: {e:?}"))?;

    let body_bytes = response.into_body();
    let body = String::from_utf8(body_bytes)
        .map_err(|e| format!("Failed to parse response body: {e}"))?;

    let wrapper: ToolResponseWrapper<String> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse response wrapper: {e}"))?;

    let result_text = &wrapper.content[0].text;
    let result: DotProductResult = serde_json::from_str(result_text)
        .map_err(|e| format!("Failed to parse dot product result: {e}"))?;

    Ok(result.dot_product)
}

async fn call_cross_product(vector_a: &[f64], vector_b: &[f64]) -> Result<Vec<f64>, String> {
    use spin_sdk::http::{Method, Request};

    let input = TwoVectorInput {
        vector1: Vector3D {
            x: vector_a[0],
            y: vector_a[1],
            z: vector_a[2],
        },
        vector2: Vector3D {
            x: vector_b[0],
            y: vector_b[1],
            z: vector_b[2],
        },
    };
    let request_body = serde_json::to_string(&input)
        .map_err(|e| format!("Failed to serialize cross product input: {e}"))?;

    let request = Request::builder()
        .method(Method::Post)
        .uri("http://cross-product.spin.internal")
        .header("Content-Type", "application/json")
        .body(request_body.into_bytes())
        .build();

    let response: spin_sdk::http::Response = spin_sdk::http::send(request)
        .await
        .map_err(|e| format!("Failed to call cross_product: {e:?}"))?;

    let body_bytes = response.into_body();
    let body = String::from_utf8(body_bytes)
        .map_err(|e| format!("Failed to parse response body: {e}"))?;

    let wrapper: ToolResponseWrapper<String> = serde_json::from_str(&body)
        .map_err(|e| format!("Failed to parse response wrapper: {e}"))?;

    let result_text = &wrapper.content[0].text;
    let result: CrossProductResult = serde_json::from_str(result_text)
        .map_err(|e| format!("Failed to parse cross product result: {e}"))?;

    Ok(vec![
        result.cross_product.x,
        result.cross_product.y,
        result.cross_product.z,
    ])
}
