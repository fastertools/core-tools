use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
struct SphereVolumeInput {
    center: Vector3D,
    radius: f64,
}

#[derive(Serialize)]
struct SphereVolumeResponse {
    volume: f64,
    calculation_method: String,
    center: Vector3D,
    radius: f64,
}

fn calculate_sphere_volume(input: &SphereVolumeInput) -> Result<SphereVolumeResponse, String> {
    if input.radius < 0.0 {
        return Err("Radius cannot be negative".to_string());
    }
    
    // Volume = (4/3) * π * r³
    let volume = (4.0 / 3.0) * std::f64::consts::PI * input.radius.powi(3);
    
    Ok(SphereVolumeResponse {
        volume,
        calculation_method: "Sphere formula: (4/3)πr³".to_string(),
        center: input.center.clone(),
        radius: input.radius,
    })
}

#[tool]
fn sphere_volume(input: SphereVolumeInput) -> ToolResponse {
    match calculate_sphere_volume(&input) {
        Ok(result) => {
            match serde_json::to_string(&result) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}