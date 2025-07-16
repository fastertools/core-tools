use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixVectorInput {
    pub matrix: Matrix3x3,
    pub vector: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixVectorOutput {
    pub result: Vector3D,
}

impl Matrix3x3 {
    pub fn multiply_vector(&self, v: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.m00 * v.x + self.m01 * v.y + self.m02 * v.z,
            y: self.m10 * v.x + self.m11 * v.y + self.m12 * v.z,
            z: self.m20 * v.x + self.m21 * v.y + self.m22 * v.z,
        }
    }

    pub fn is_valid(&self) -> bool {
        let values = [
            self.m00, self.m01, self.m02,
            self.m10, self.m11, self.m12,
            self.m20, self.m21, self.m22,
        ];
        values.iter().all(|&val| val.is_finite())
    }
}

impl Vector3D {
    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

pub fn matrix_vector_multiply_logic(input: MatrixVectorInput) -> Result<MatrixVectorOutput, String> {
    // Input validation
    if !input.matrix.is_valid() {
        return Err("Invalid matrix: contains NaN or infinite values".to_string());
    }
    
    if !input.vector.is_valid() {
        return Err("Invalid vector: contains NaN or infinite values".to_string());
    }
    
    // Perform matrix-vector multiplication
    let result = input.matrix.multiply_vector(&input.vector);
    
    // Validate result
    if !result.is_valid() {
        return Err("Matrix-vector multiplication resulted in invalid values".to_string());
    }
    
    Ok(MatrixVectorOutput { result })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_matrix() {
        let identity = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        let vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        
        let input = MatrixVectorInput {
            matrix: identity,
            vector: vector.clone(),
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x - vector.x).abs() < 1e-15);
        assert!((result.result.y - vector.y).abs() < 1e-15);
        assert!((result.result.z - vector.z).abs() < 1e-15);
    }

    #[test]
    fn test_zero_matrix() {
        let zero = Matrix3x3 {
            m00: 0.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 0.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 0.0,
        };
        let vector = Vector3D { x: 5.0, y: 10.0, z: 15.0 };
        
        let input = MatrixVectorInput {
            matrix: zero,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x).abs() < 1e-15);
        assert!((result.result.y).abs() < 1e-15);
        assert!((result.result.z).abs() < 1e-15);
    }

    #[test]
    fn test_scaling_matrix() {
        let scaling = Matrix3x3 {
            m00: 2.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 3.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 4.0,
        };
        let vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        
        let input = MatrixVectorInput {
            matrix: scaling,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x - 2.0).abs() < 1e-15);
        assert!((result.result.y - 6.0).abs() < 1e-15);
        assert!((result.result.z - 12.0).abs() < 1e-15);
    }

    #[test]
    fn test_general_matrix() {
        let matrix = Matrix3x3 {
            m00: 1.0, m01: 2.0, m02: 3.0,
            m10: 4.0, m11: 5.0, m12: 6.0,
            m20: 7.0, m21: 8.0, m22: 9.0,
        };
        let vector = Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x - 6.0).abs() < 1e-15);  // 1*1 + 2*1 + 3*1 = 6
        assert!((result.result.y - 15.0).abs() < 1e-15); // 4*1 + 5*1 + 6*1 = 15
        assert!((result.result.z - 24.0).abs() < 1e-15); // 7*1 + 8*1 + 9*1 = 24
    }

    #[test]
    fn test_zero_vector() {
        let matrix = Matrix3x3 {
            m00: 1.0, m01: 2.0, m02: 3.0,
            m10: 4.0, m11: 5.0, m12: 6.0,
            m20: 7.0, m21: 8.0, m22: 9.0,
        };
        let vector = Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x).abs() < 1e-15);
        assert!((result.result.y).abs() < 1e-15);
        assert!((result.result.z).abs() < 1e-15);
    }

    #[test]
    fn test_negative_values() {
        let matrix = Matrix3x3 {
            m00: -1.0, m01: 2.0, m02: -3.0,
            m10: 4.0, m11: -5.0, m12: 6.0,
            m20: -7.0, m21: 8.0, m22: -9.0,
        };
        let vector = Vector3D { x: 1.0, y: -2.0, z: 3.0 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x - (-14.0)).abs() < 1e-15); // -1*1 + 2*(-2) + (-3)*3 = -14
        assert!((result.result.y - 32.0).abs() < 1e-15);    // 4*1 + (-5)*(-2) + 6*3 = 32
        assert!((result.result.z - (-50.0)).abs() < 1e-15); // -7*1 + 8*(-2) + (-9)*3 = -50
    }

    #[test]
    fn test_nan_matrix() {
        let matrix = Matrix3x3 {
            m00: f64::NAN, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        let vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid matrix: contains NaN or infinite values");
    }

    #[test]
    fn test_infinite_vector() {
        let matrix = Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        };
        let vector = Vector3D { x: f64::INFINITY, y: 2.0, z: 3.0 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid vector: contains NaN or infinite values");
    }

    #[test]
    fn test_large_values() {
        let matrix = Matrix3x3 {
            m00: 1e10, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1e10, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1e10,
        };
        let vector = Vector3D { x: 1e-10, y: 2e-10, z: 3e-10 };
        
        let input = MatrixVectorInput {
            matrix,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!((result.result.x - 1.0).abs() < 1e-15);
        assert!((result.result.y - 2.0).abs() < 1e-15);
        assert!((result.result.z - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_rotation_matrix_z_axis() {
        // 90-degree rotation around Z-axis
        let angle = std::f64::consts::PI / 2.0;
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        
        let rotation_z = Matrix3x3 {
            m00: cos_a, m01: -sin_a, m02: 0.0,
            m10: sin_a, m11: cos_a,  m12: 0.0,
            m20: 0.0,   m21: 0.0,    m22: 1.0,
        };
        let vector = Vector3D { x: 1.0, y: 0.0, z: 0.0 };
        
        let input = MatrixVectorInput {
            matrix: rotation_z,
            vector,
        };
        
        let result = matrix_vector_multiply_logic(input).unwrap();
        assert!(result.result.x.abs() < 1e-15); // Should be ~0
        assert!((result.result.y - 1.0).abs() < 1e-15); // Should be 1
        assert!(result.result.z.abs() < 1e-15); // Should be 0
    }

    #[test]
    fn test_matrix_validation() {
        let valid_matrix = Matrix3x3 {
            m00: 1.0, m01: 2.0, m02: 3.0,
            m10: 4.0, m11: 5.0, m12: 6.0,
            m20: 7.0, m21: 8.0, m22: 9.0,
        };
        assert!(valid_matrix.is_valid());
        
        let invalid_matrix = Matrix3x3 {
            m00: f64::NAN, m01: 2.0, m02: 3.0,
            m10: 4.0, m11: 5.0, m12: 6.0,
            m20: 7.0, m21: 8.0, m22: 9.0,
        };
        assert!(!invalid_matrix.is_valid());
    }

    #[test]
    fn test_vector_validation() {
        let valid_vector = Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        assert!(valid_vector.is_valid());
        
        let invalid_vector = Vector3D { x: f64::NAN, y: 2.0, z: 3.0 };
        assert!(!invalid_vector.is_valid());
    }
}