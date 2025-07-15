use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct TwoVectorInput {
    pub vector1: Vector3D,
    pub vector2: Vector3D,
}

#[derive(Serialize, JsonSchema)]
pub struct VectorAngleResult {
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub cos_angle: f64,
    pub vector1_magnitude: f64,
    pub vector2_magnitude: f64,
    pub is_perpendicular: bool,
    pub is_parallel: bool,
}

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn angle_with(&self, other: &Vector3D) -> Result<f64, String> {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();
        
        if mag1 == 0.0 || mag2 == 0.0 {
            return Err("Cannot compute angle with zero vector".to_string());
        }

        let cos_angle = self.dot(other) / (mag1 * mag2);
        
        // Clamp to [-1, 1] to handle numerical precision issues
        let cos_angle = cos_angle.max(-1.0).min(1.0);
        
        Ok(cos_angle.acos())
    }
}

#[tool]
pub fn vector_angle(input: TwoVectorInput) -> Result<VectorAngleResult, String> {
    let v1 = &input.vector1;
    let v2 = &input.vector2;
    
    if v1.is_zero() || v2.is_zero() {
        return Err("Cannot compute angle with zero vector".to_string());
    }
    
    let mag1 = v1.magnitude();
    let mag2 = v2.magnitude();
    let angle_radians = v1.angle_with(v2)?;
    let angle_degrees = angle_radians.to_degrees();
    let cos_angle = v1.dot(v2) / (mag1 * mag2);
    
    // Check for special relationships
    const EPSILON: f64 = 1e-10;
    let is_perpendicular = (angle_radians - std::f64::consts::PI / 2.0).abs() < EPSILON;
    let is_parallel = angle_radians < EPSILON || (angle_radians - std::f64::consts::PI).abs() < EPSILON;
    
    Ok(VectorAngleResult {
        angle_radians,
        angle_degrees,
        cos_angle,
        vector1_magnitude: mag1,
        vector2_magnitude: mag2,
        is_perpendicular,
        is_parallel,
    })
}