use ftl_sdk::ToolResponse;
#[cfg(not(test))]
use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;

// Re-export types from logic module
pub use logic::{VectorAnalysisInput as LogicInput, VectorAnalysisOutput as LogicOutput};

// Define wrapper types with JsonSchema for FTL-SDK
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VectorAnalysisInput {
    /// First 3D vector [x, y, z]
    pub vector_a: Vec<f64>,
    /// Second 3D vector [x, y, z]
    pub vector_b: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct VectorAnalysisOutput {
    /// Magnitude of vector A
    pub magnitude_a: f64,
    /// Magnitude of vector B  
    pub magnitude_b: f64,
    /// Angle between vectors in radians
    pub angle_between_radians: f64,
    /// Angle between vectors in degrees
    pub angle_between_degrees: f64,
    /// Dot product of the two vectors
    pub dot_product: f64,
    /// Cross product of the two vectors
    pub cross_product: Vec<f64>,
    /// Whether the vectors are orthogonal (perpendicular)
    pub is_orthogonal: bool,
    /// Whether the vectors are parallel
    pub is_parallel: bool,
    /// Vector similarity score (-1 to 1, cosine similarity)
    pub vector_similarity: f64,
}

/// Comprehensive vector analysis using composition of atomic math3d tools
///
/// This composite tool demonstrates the composition pattern by calling multiple
/// atomic tools (vector_magnitude, vector_angle, dot_product, cross_product) and
/// combining their results for comprehensive vector analysis.
#[cfg_attr(not(test), tool)]
pub async fn vector_analysis(input: VectorAnalysisInput) -> ToolResponse {
    // Convert to logic types
    let logic_input = LogicInput {
        vector_a: input.vector_a,
        vector_b: input.vector_b,
    };

    // Call async logic implementation
    match logic::analyze_vectors(logic_input).await {
        Ok(result) => {
            // Convert back to wrapper types
            let response = VectorAnalysisOutput {
                magnitude_a: result.magnitude_a,
                magnitude_b: result.magnitude_b,
                angle_between_radians: result.angle_between_radians,
                angle_between_degrees: result.angle_between_degrees,
                dot_product: result.dot_product,
                cross_product: result.cross_product,
                is_orthogonal: result.is_orthogonal,
                is_parallel: result.is_parallel,
                vector_similarity: result.vector_similarity,
            };
            ToolResponse::text(serde_json::to_string_pretty(&response).unwrap())
        }
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_analysis_input_validation() {
        let input = VectorAnalysisInput {
            vector_a: vec![1.0, 0.0, 0.0],
            vector_b: vec![0.0, 1.0, 0.0],
        };

        assert_eq!(input.vector_a.len(), 3);
        assert_eq!(input.vector_b.len(), 3);
    }

    #[test]
    fn test_vector_analysis_output_structure() {
        let output = VectorAnalysisOutput {
            magnitude_a: 1.0,
            magnitude_b: 1.0,
            angle_between_radians: std::f64::consts::PI / 2.0,
            angle_between_degrees: 90.0,
            dot_product: 0.0,
            cross_product: vec![0.0, 0.0, 1.0],
            is_orthogonal: true,
            is_parallel: false,
            vector_similarity: 0.0,
        };

        assert!(output.is_orthogonal);
        assert!(!output.is_parallel);
        assert_eq!(output.cross_product.len(), 3);
    }
}
