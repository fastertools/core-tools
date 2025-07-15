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
pub struct PyramidInput {
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[derive(Serialize, JsonSchema)]
pub struct PyramidResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub base_area: f64,
    pub height: f64,
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[tool]
pub fn pyramid_volume(input: PyramidInput) -> Result<PyramidResponse, String> {
    if input.base_points.len() < 3 {
        return Err("At least 3 points are required for the base".to_string());
    }
    
    // Calculate the area of the base polygon using the shoelace formula
    let base_area = calculate_polygon_area(&input.base_points)?;
    
    // Calculate the height by finding the distance from apex to the base plane
    let height = calculate_point_to_plane_distance(&input.apex, &input.base_points)?;
    
    // Volume = (1/3) * base_area * height
    let volume = (1.0 / 3.0) * base_area * height;
    
    Ok(PyramidResponse {
        volume,
        calculation_method: "Pyramid formula: (1/3) × base_area × height".to_string(),
        base_area,
        height,
        base_points: input.base_points,
        apex: input.apex,
    })
}

fn calculate_polygon_area(points: &[Vector3D]) -> Result<f64, String> {
    if points.len() < 3 {
        return Err("At least 3 points required for polygon area".to_string());
    }
    
    // Calculate the normal vector of the plane containing the polygon
    let v1 = Vector3D {
        x: points[1].x - points[0].x,
        y: points[1].y - points[0].y,
        z: points[1].z - points[0].z,
    };
    
    let v2 = Vector3D {
        x: points[2].x - points[0].x,
        y: points[2].y - points[0].y,
        z: points[2].z - points[0].z,
    };
    
    let normal = Vector3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };
    
    let normal_magnitude = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    
    if normal_magnitude < 1e-10 {
        return Err("Points are collinear, cannot form a polygon".to_string());
    }
    
    // Project the polygon onto the plane with the largest normal component
    let abs_nx = normal.x.abs();
    let abs_ny = normal.y.abs();
    let abs_nz = normal.z.abs();
    
    let mut area = 0.0;
    
    if abs_nz >= abs_nx && abs_nz >= abs_ny {
        // Project onto XY plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].y - points[j].x * points[i].y;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_nz);
    } else if abs_ny >= abs_nx {
        // Project onto XZ plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].z - points[j].x * points[i].z;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_ny);
    } else {
        // Project onto YZ plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].y * points[j].z - points[j].y * points[i].z;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_nx);
    }
    
    Ok(area)
}

fn calculate_point_to_plane_distance(point: &Vector3D, plane_points: &[Vector3D]) -> Result<f64, String> {
    if plane_points.len() < 3 {
        return Err("At least 3 points required to define a plane".to_string());
    }
    
    // Calculate plane normal
    let v1 = Vector3D {
        x: plane_points[1].x - plane_points[0].x,
        y: plane_points[1].y - plane_points[0].y,
        z: plane_points[1].z - plane_points[0].z,
    };
    
    let v2 = Vector3D {
        x: plane_points[2].x - plane_points[0].x,
        y: plane_points[2].y - plane_points[0].y,
        z: plane_points[2].z - plane_points[0].z,
    };
    
    let normal = Vector3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };
    
    let normal_magnitude = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();
    
    if normal_magnitude < 1e-10 {
        return Err("Points are collinear, cannot define a plane".to_string());
    }
    
    // Normalize the normal vector
    let unit_normal = Vector3D {
        x: normal.x / normal_magnitude,
        y: normal.y / normal_magnitude,
        z: normal.z / normal_magnitude,
    };
    
    // Vector from plane point to the test point
    let plane_to_point = Vector3D {
        x: point.x - plane_points[0].x,
        y: point.y - plane_points[0].y,
        z: point.z - plane_points[0].z,
    };
    
    // Distance is the dot product with the unit normal
    let distance = (plane_to_point.x * unit_normal.x + 
                   plane_to_point.y * unit_normal.y + 
                   plane_to_point.z * unit_normal.z).abs();
    
    Ok(distance)
}