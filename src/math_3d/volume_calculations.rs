use serde::{Deserialize, Serialize};
use crate::math_3d::vector_ops::Vector3D;
use crate::common::ErrorResponse;

#[derive(Deserialize)]
pub struct TetrahedronInput {
    pub point_a: Vector3D,
    pub point_b: Vector3D,
    pub point_c: Vector3D,
    pub point_d: Vector3D,
}

#[derive(Deserialize)]
pub struct ConvexHullInput {
    pub points: Vec<Vector3D>,
}

#[derive(Deserialize)]
pub struct BoundingBoxInput {
    pub points: Vec<Vector3D>,
    pub box_type: String, // "aabb" for axis-aligned, "obb" for oriented
}

#[derive(Deserialize)]
pub struct SphereInput {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Deserialize)]
pub struct CylinderInput {
    pub base_center: Vector3D,
    pub axis: Vector3D,
    pub radius: f64,
    pub height: f64,
}

#[derive(Deserialize)]
pub struct PyramidInput {
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[derive(Serialize)]
pub struct VolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
}

#[derive(Serialize)]
pub struct TetrahedronResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub points: [Vector3D; 4],
}

#[derive(Serialize)]
pub struct BoundingBoxResponse {
    pub volume: f64,
    pub box_type: String,
    pub min_point: Vector3D,
    pub max_point: Vector3D,
    pub dimensions: Vector3D,
}

#[derive(Serialize)]
pub struct ConvexHullResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub hull_points: Vec<Vector3D>,
    pub num_triangles: usize,
}

pub fn calculate_tetrahedron_volume(input: TetrahedronInput) -> Result<TetrahedronResponse, ErrorResponse> {
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
    
    Ok(TetrahedronResponse {
        volume,
        calculation_method: "Scalar triple product".to_string(),
        points: [input.point_a, input.point_b, input.point_c, input.point_d],
    })
}

pub fn calculate_sphere_volume(input: SphereInput) -> Result<VolumeResponse, ErrorResponse> {
    if input.radius < 0.0 {
        return Err(ErrorResponse {
            error: "Radius cannot be negative".to_string(),
        });
    }
    
    // Volume = (4/3) * π * r³
    let volume = (4.0 / 3.0) * std::f64::consts::PI * input.radius.powi(3);
    
    Ok(VolumeResponse {
        volume,
        calculation_method: "Sphere formula: (4/3)πr³".to_string(),
    })
}

pub fn calculate_cylinder_volume(input: CylinderInput) -> Result<VolumeResponse, ErrorResponse> {
    if input.radius < 0.0 {
        return Err(ErrorResponse {
            error: "Radius cannot be negative".to_string(),
        });
    }
    
    if input.height < 0.0 {
        return Err(ErrorResponse {
            error: "Height cannot be negative".to_string(),
        });
    }
    
    // Volume = π * r² * h
    let volume = std::f64::consts::PI * input.radius.powi(2) * input.height;
    
    Ok(VolumeResponse {
        volume,
        calculation_method: "Cylinder formula: πr²h".to_string(),
    })
}

pub fn calculate_aabb_volume(input: BoundingBoxInput) -> Result<BoundingBoxResponse, ErrorResponse> {
    if input.points.is_empty() {
        return Err(ErrorResponse {
            error: "At least one point is required".to_string(),
        });
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

pub fn calculate_pyramid_volume(input: PyramidInput) -> Result<VolumeResponse, ErrorResponse> {
    if input.base_points.len() < 3 {
        return Err(ErrorResponse {
            error: "At least 3 points are required for the base".to_string(),
        });
    }
    
    // Calculate the area of the base polygon using the shoelace formula
    let base_area = calculate_polygon_area(&input.base_points)?;
    
    // Calculate the height by finding the distance from apex to the base plane
    let height = calculate_point_to_plane_distance(&input.apex, &input.base_points)?;
    
    // Volume = (1/3) * base_area * height
    let volume = (1.0 / 3.0) * base_area * height;
    
    Ok(VolumeResponse {
        volume,
        calculation_method: "Pyramid formula: (1/3) × base_area × height".to_string(),
    })
}

pub fn calculate_convex_hull_volume(input: ConvexHullInput) -> Result<ConvexHullResponse, ErrorResponse> {
    if input.points.len() < 4 {
        return Err(ErrorResponse {
            error: "At least 4 points are required to form a 3D convex hull".to_string(),
        });
    }
    
    // For simplicity, we'll use a basic triangulation approach
    // In a full implementation, you'd use algorithms like QuickHull or Gift Wrapping
    
    // Find the hull points (simplified - just use the original points for now)
    let hull_points = input.points.clone();
    
    // Calculate volume using triangulation from a reference point
    let reference_point = &hull_points[0];
    let mut total_volume = 0.0;
    let mut triangle_count = 0;
    
    // This is a simplified approach - triangulate from the reference point
    for i in 1..hull_points.len() - 1 {
        for j in i + 1..hull_points.len() {
            if j + 1 < hull_points.len() {
                // Form tetrahedron with reference point and three other points
                let tetrahedron_input = TetrahedronInput {
                    point_a: reference_point.clone(),
                    point_b: hull_points[i].clone(),
                    point_c: hull_points[j].clone(),
                    point_d: hull_points[j + 1].clone(),
                };
                
                if let Ok(tetrahedron_result) = calculate_tetrahedron_volume(tetrahedron_input) {
                    total_volume += tetrahedron_result.volume;
                    triangle_count += 1;
                }
            }
        }
    }
    
    Ok(ConvexHullResponse {
        volume: total_volume,
        calculation_method: "Simplified tetrahedron triangulation".to_string(),
        hull_points,
        num_triangles: triangle_count,
    })
}

fn calculate_polygon_area(points: &[Vector3D]) -> Result<f64, ErrorResponse> {
    if points.len() < 3 {
        return Err(ErrorResponse {
            error: "At least 3 points required for polygon area".to_string(),
        });
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
        return Err(ErrorResponse {
            error: "Points are collinear, cannot form a polygon".to_string(),
        });
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

fn calculate_point_to_plane_distance(point: &Vector3D, plane_points: &[Vector3D]) -> Result<f64, ErrorResponse> {
    if plane_points.len() < 3 {
        return Err(ErrorResponse {
            error: "At least 3 points required to define a plane".to_string(),
        });
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
        return Err(ErrorResponse {
            error: "Points are collinear, cannot define a plane".to_string(),
        });
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

pub fn handle_tetrahedron_volume(input: TetrahedronInput) -> Result<TetrahedronResponse, ErrorResponse> {
    calculate_tetrahedron_volume(input)
}

pub fn handle_sphere_volume(input: SphereInput) -> Result<VolumeResponse, ErrorResponse> {
    calculate_sphere_volume(input)
}

pub fn handle_cylinder_volume(input: CylinderInput) -> Result<VolumeResponse, ErrorResponse> {
    calculate_cylinder_volume(input)
}

pub fn handle_aabb_volume(input: BoundingBoxInput) -> Result<BoundingBoxResponse, ErrorResponse> {
    if input.box_type.to_lowercase() != "aabb" {
        return Err(ErrorResponse {
            error: "Only AABB (axis-aligned bounding box) is currently supported".to_string(),
        });
    }
    calculate_aabb_volume(input)
}

pub fn handle_pyramid_volume(input: PyramidInput) -> Result<VolumeResponse, ErrorResponse> {
    calculate_pyramid_volume(input)
}

pub fn handle_convex_hull_volume(input: ConvexHullInput) -> Result<ConvexHullResponse, ErrorResponse> {
    calculate_convex_hull_volume(input)
}