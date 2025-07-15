use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, JsonSchema)]
struct QuaternionSlerpInput {
    q1: Quaternion,
    q2: Quaternion,
    t: f64,
}

#[derive(Serialize)]
struct QuaternionSlerpResponse {
    result: Quaternion,
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

#[tool]
fn quaternion_slerp(input: QuaternionSlerpInput) -> ToolResponse {
    match input.q1.slerp(&input.q2, input.t) {
        Ok(result) => {
            let response = QuaternionSlerpResponse { result };
            match serde_json::to_string(&response) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}
