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
struct CylinderVolumeInput {
    base_center: Vector3D,
    axis: Vector3D,
    radius: f64,
    height: f64,
}

#[derive(Serialize)]
struct CylinderVolumeResponse {
    volume: f64,
    calculation_method: String,
    base_center: Vector3D,
    axis: Vector3D,
    radius: f64,
    height: f64,
}

fn calculate_cylinder_volume(input: &CylinderVolumeInput) -> Result<CylinderVolumeResponse, String> {
    if input.radius < 0.0 {
        return Err("Radius cannot be negative".to_string());
    }
    
    if input.height < 0.0 {
        return Err("Height cannot be negative".to_string());
    }
    
    // Volume = π * r² * h
    let volume = std::f64::consts::PI * input.radius.powi(2) * input.height;
    
    Ok(CylinderVolumeResponse {
        volume,
        calculation_method: "Cylinder formula: πr²h".to_string(),
        base_center: input.base_center.clone(),
        axis: input.axis.clone(),
        radius: input.radius,
        height: input.height,
    })
}

#[tool]
fn cylinder_volume(input: CylinderVolumeInput) -> ToolResponse {
    match calculate_cylinder_volume(&input) {
        Ok(result) => {
            match serde_json::to_string(&result) {
                Ok(json) => ToolResponse::text(json),
                Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
            }
        }
        Err(e) => ToolResponse::error(&e),
    }
}