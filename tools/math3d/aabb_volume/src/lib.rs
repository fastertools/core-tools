use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct BoundingBoxInput {
    pub points: Vec<Vector3D>,
}

#[derive(Serialize, JsonSchema)]
pub struct BoundingBoxResponse {
    pub volume: f64,
    pub box_type: String,
    pub min_point: Vector3D,
    pub max_point: Vector3D,
    pub dimensions: Vector3D,
}

#[tool]
pub fn aabb_volume(input: BoundingBoxInput) -> Result<BoundingBoxResponse, String> {
    if input.points.is_empty() {
        return Err("At least one point is required".to_string());
    }
    
    let first_point = &input.points[0];
    let mut min_x = first_point.x;
    let mut max_x = first_point.x;
    let mut min_y = first_point.y;
    let mut max_y = first_point.y;
    let mut min_z = first_point.z;
    let mut max_z = first_point.z;
    
    // Find the minimum and maximum coordinates
    for point in &input.points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
        min_z = min_z.min(point.z);
        max_z = max_z.max(point.z);
    }
    
    let dimensions = Vector3D {
        x: max_x - min_x,
        y: max_y - min_y,
        z: max_z - min_z,
    };
    
    let volume = dimensions.x * dimensions.y * dimensions.z;
    
    Ok(BoundingBoxResponse {
        volume,
        box_type: "AABB (Axis-Aligned Bounding Box)".to_string(),
        min_point: Vector3D { x: min_x, y: min_y, z: min_z },
        max_point: Vector3D { x: max_x, y: max_y, z: max_z },
        dimensions,
    })
}