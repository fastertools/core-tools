use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionSlerpInput {
    pub q1: Quaternion,
    pub q2: Quaternion,
    pub t: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionSlerpOutput {
    pub result: Quaternion,
}

impl Quaternion {
    pub fn normalize(&self) -> Result<Self, String> {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if magnitude < 1e-10 {
            return Err("Quaternion cannot be zero".to_string());
        }

        Ok(Quaternion {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        })
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite() && self.w.is_finite()
    }

    pub fn slerp(&self, other: &Quaternion, t: f64) -> Result<Quaternion, String> {
        if t < 0.0 || t > 1.0 {
            return Err("Interpolation parameter t must be between 0 and 1".to_string());
        }

        let dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        
        let (q1, q2) = if dot < 0.0 {
            (self.clone(), Quaternion { x: -other.x, y: -other.y, z: -other.z, w: -other.w })
        } else {
            (self.clone(), other.clone())
        };

        let dot_abs = dot.abs();
        if dot_abs > 0.9995 {
            let result = Quaternion {
                x: q1.x + t * (q2.x - q1.x),
                y: q1.y + t * (q2.y - q1.y),
                z: q1.z + t * (q2.z - q1.z),
                w: q1.w + t * (q2.w - q1.w),
            };
            return result.normalize();
        }

        let theta_0 = dot_abs.acos();
        let sin_theta_0 = theta_0.sin();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let s0 = cos_theta - dot_abs * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Ok(Quaternion {
            x: s0 * q1.x + s1 * q2.x,
            y: s0 * q1.y + s1 * q2.y,
            z: s0 * q1.z + s1 * q2.z,
            w: s0 * q1.w + s1 * q2.w,
        })
    }
}

pub fn quaternion_slerp_logic(input: QuaternionSlerpInput) -> Result<QuaternionSlerpOutput, String> {
    // Input validation
    if !input.q1.is_valid() {
        return Err("Invalid quaternion q1: contains NaN or infinite values".to_string());
    }
    
    if !input.q2.is_valid() {
        return Err("Invalid quaternion q2: contains NaN or infinite values".to_string());
    }
    
    if !input.t.is_finite() {
        return Err("Invalid interpolation parameter t: must be finite".to_string());
    }
    
    // Perform SLERP
    let result = input.q1.slerp(&input.q2, input.t)?;
    
    // Validate result
    if !result.is_valid() {
        return Err("SLERP resulted in invalid quaternion".to_string());
    }
    
    Ok(QuaternionSlerpOutput { result })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_quaternion() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        assert!((result.result.x).abs() < 1e-15);
        assert!((result.result.y).abs() < 1e-15);
        assert!((result.result.z).abs() < 1e-15);
        assert!((result.result.w - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_slerp_t_zero() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1: q1.clone(), q2, t: 0.0 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        assert!((result.result.x - q1.x).abs() < 1e-15);
        assert!((result.result.y - q1.y).abs() < 1e-15);
        assert!((result.result.z - q1.z).abs() < 1e-15);
        assert!((result.result.w - q1.w).abs() < 1e-15);
    }

    #[test]
    fn test_slerp_t_one() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2: q2.clone(), t: 1.0 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        // Result should be q2 (or -q2, but normalized)
        let mag = result.result.magnitude();
        assert!((mag - 1.0).abs() < 1e-15, "Result should be normalized");
    }

    #[test]
    fn test_slerp_halfway() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 0.0, y: 0.0, z: 1.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        // At t=0.5, should be halfway between the quaternions
        let expected_w = (std::f64::consts::PI / 4.0).cos(); // cos(45°)
        let expected_z = (std::f64::consts::PI / 4.0).sin(); // sin(45°)
        
        assert!((result.result.x).abs() < 1e-15);
        assert!((result.result.y).abs() < 1e-15);
        assert!((result.result.z - expected_z).abs() < 1e-14);
        assert!((result.result.w - expected_w).abs() < 1e-14);
    }

    #[test]
    fn test_quaternion_normalization() {
        let q = Quaternion { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        let normalized = q.normalize().unwrap();
        
        let magnitude = normalized.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-15, "Normalized quaternion should have magnitude 1");
    }

    #[test]
    fn test_zero_quaternion_normalization() {
        let q = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };
        let result = q.normalize();
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Quaternion cannot be zero");
    }

    #[test]
    fn test_slerp_opposite_quaternions() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: -1.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        // SLERP should handle the sign flip and interpolate correctly
        let magnitude = result.result.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-15, "Result should be normalized");
    }

    #[test]
    fn test_very_close_quaternions() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 0.0001, y: 0.0, z: 0.0, w: 0.99999999 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        // For very close quaternions, linear interpolation is used
        let magnitude = result.result.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-14, "Result should be normalized");
    }

    #[test]
    fn test_invalid_t_parameter() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: -0.5 };
        let result = quaternion_slerp_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Interpolation parameter t must be between 0 and 1");
        
        let input = QuaternionSlerpInput { q1: Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }, q2: Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 }, t: 1.5 };
        let result = quaternion_slerp_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Interpolation parameter t must be between 0 and 1");
    }

    #[test]
    fn test_nan_quaternion() {
        let q1 = Quaternion { x: f64::NAN, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid quaternion q1: contains NaN or infinite values");
    }

    #[test]
    fn test_infinite_quaternion() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: f64::INFINITY, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: 0.5 };
        let result = quaternion_slerp_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid quaternion q2: contains NaN or infinite values");
    }

    #[test]
    fn test_nan_t_parameter() {
        let q1 = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let q2 = Quaternion { x: 1.0, y: 0.0, z: 0.0, w: 0.0 };
        
        let input = QuaternionSlerpInput { q1, q2, t: f64::NAN };
        let result = quaternion_slerp_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid interpolation parameter t: must be finite");
    }

    #[test]
    fn test_quaternion_validation() {
        let valid_q = Quaternion { x: 1.0, y: 2.0, z: 3.0, w: 4.0 };
        assert!(valid_q.is_valid());
        
        let invalid_q = Quaternion { x: f64::NAN, y: 2.0, z: 3.0, w: 4.0 };
        assert!(!invalid_q.is_valid());
        
        let infinite_q = Quaternion { x: 1.0, y: f64::INFINITY, z: 3.0, w: 4.0 };
        assert!(!infinite_q.is_valid());
    }

    #[test]
    fn test_quaternion_magnitude() {
        let q = Quaternion { x: 3.0, y: 4.0, z: 0.0, w: 0.0 };
        let magnitude = q.magnitude();
        assert!((magnitude - 5.0).abs() < 1e-15); // 3-4-5 triangle
        
        let unit_q = Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
        let magnitude = unit_q.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_complex_slerp_scenario() {
        // Test SLERP with arbitrary quaternions representing rotations
        let q1 = Quaternion { x: 0.5, y: 0.5, z: 0.5, w: 0.5 };
        let q2 = Quaternion { x: -0.5, y: 0.5, z: -0.5, w: 0.5 };
        
        // Normalize first
        let q1_norm = q1.normalize().unwrap();
        let q2_norm = q2.normalize().unwrap();
        
        let input = QuaternionSlerpInput { q1: q1_norm, q2: q2_norm, t: 0.3 };
        let result = quaternion_slerp_logic(input).unwrap();
        
        // Result should be normalized
        let magnitude = result.result.magnitude();
        assert!((magnitude - 1.0).abs() < 1e-14, "Result should be normalized");
        
        // Result should be valid
        assert!(result.result.is_valid());
    }
}