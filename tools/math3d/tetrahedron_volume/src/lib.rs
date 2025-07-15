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
struct TetrahedronVolumeInput {
    point_a: Vector3D,
    point_b: Vector3D,
    point_c: Vector3D,
    point_d: Vector3D,
}

#[derive(Serialize)]
struct TetrahedronVolumeResponse {
    volume: f64,
    calculation_method: String,
    points: [Vector3D; 4],
}

fn calculate_tetrahedron_volume(input: &TetrahedronVolumeInput) -> TetrahedronVolumeResponse {
    let a = &input.point_a;
    let b = &input.point_b;
    let c = &input.point_c;
    let d = &input.point_d;
    
    // Calculate vectors from point A to the other three points
    let ab = Vector3D {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    
    let ac = Vector3D {
        x: c.x - a.x,
        y: c.y - a.y,
        z: c.z - a.z,
    };
    
    let ad = Vector3D {
        x: d.x - a.x,
        y: d.y - a.y,
        z: d.z - a.z,
    };
    
    // Calculate the scalar triple product: AB · (AC × AD)
    let cross_ac_ad = Vector3D {
        x: ac.y * ad.z - ac.z * ad.y,
        y: ac.z * ad.x - ac.x * ad.z,
        z: ac.x * ad.y - ac.y * ad.x,
    };
    
    let scalar_triple_product = ab.x * cross_ac_ad.x + ab.y * cross_ac_ad.y + ab.z * cross_ac_ad.z;
    
    // Volume = |scalar triple product| / 6
    let volume = scalar_triple_product.abs() / 6.0;
    
    TetrahedronVolumeResponse {
        volume,
        calculation_method: "Scalar triple product".to_string(),
        points: [input.point_a.clone(), input.point_b.clone(), input.point_c.clone(), input.point_d.clone()],
    }
}

#[tool]
fn tetrahedron_volume(input: TetrahedronVolumeInput) -> ToolResponse {
    let result = calculate_tetrahedron_volume(&input);
    
    match serde_json::to_string(&result) {
        Ok(json) => ToolResponse::text(json),
        Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
    }
}