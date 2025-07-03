use serde::{Deserialize, Serialize};
use crate::common::ErrorResponse;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cylinder {
    pub center: Vector3,
    pub axis: Vector3,
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereRayInput {
    pub sphere: Sphere,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereSphereInput {
    pub sphere1: Sphere,
    pub sphere2: Sphere,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderRayInput {
    pub cylinder: Cylinder,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AABBRayInput {
    pub aabb: AABB,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AABBAABBInput {
    pub aabb1: AABB,
    pub aabb2: AABB,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereSphereResult {
    pub intersects: bool,
    pub intersection_type: String,
    pub distance_between_centers: f64,
    pub intersection_circle: Option<IntersectionCircle>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntersectionCircle {
    pub center: Vector3,
    pub radius: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AABBIntersectionResult {
    pub intersects: bool,
    pub closest_distance: Option<f64>,
    pub intersection_points: Vec<IntersectionPoint>,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    pub fn dot(&self, other: &Vector3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3 {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vector3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn subtract(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3 {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn cross(&self, other: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

pub fn handle_sphere_ray_intersection(input: SphereRayInput) -> Result<SphereRayResult, ErrorResponse> {
    let sphere = input.sphere;
    let ray = input.ray;

    if sphere.radius <= 0.0 {
        return Err(ErrorResponse {
            error: "Sphere radius must be positive".to_string(),
        });
    }

    let ray_dir = ray.direction.normalize();
    let to_sphere = sphere.center.subtract(&ray.origin);
    
    let proj_length = to_sphere.dot(&ray_dir);
    let closest_point = ray.origin.add(&ray_dir.scale(proj_length));
    let distance_to_center = sphere.center.subtract(&closest_point).magnitude();
    
    if distance_to_center > sphere.radius {
        return Ok(SphereRayResult {
            intersects: false,
            intersection_points: vec![],
            closest_distance: None,
        });
    }

    let chord_half_length = (sphere.radius * sphere.radius - distance_to_center * distance_to_center).sqrt();
    
    let mut intersection_points = vec![];
    let mut closest_distance = None;

    if proj_length >= chord_half_length {
        let t1 = proj_length - chord_half_length;
        let t2 = proj_length + chord_half_length;
        
        let point1 = ray.origin.add(&ray_dir.scale(t1));
        let point2 = ray.origin.add(&ray_dir.scale(t2));
        
        let normal1 = point1.subtract(&sphere.center).normalize();
        let normal2 = point2.subtract(&sphere.center).normalize();
        
        intersection_points.push(IntersectionPoint {
            point: point1,
            distance: t1,
            normal: normal1,
        });
        
        intersection_points.push(IntersectionPoint {
            point: point2,
            distance: t2,
            normal: normal2,
        });
        
        closest_distance = Some(t1.min(t2));
    } else if proj_length >= -chord_half_length {
        let t2 = proj_length + chord_half_length;
        let point2 = ray.origin.add(&ray_dir.scale(t2));
        let normal2 = point2.subtract(&sphere.center).normalize();
        
        intersection_points.push(IntersectionPoint {
            point: point2,
            distance: t2,
            normal: normal2,
        });
        
        closest_distance = Some(t2);
    }

    Ok(SphereRayResult {
        intersects: !intersection_points.is_empty(),
        intersection_points,
        closest_distance,
    })
}

pub fn handle_sphere_sphere_intersection(input: SphereSphereInput) -> Result<SphereSphereResult, ErrorResponse> {
    let sphere1 = input.sphere1;
    let sphere2 = input.sphere2;

    if sphere1.radius <= 0.0 || sphere2.radius <= 0.0 {
        return Err(ErrorResponse {
            error: "Sphere radii must be positive".to_string(),
        });
    }

    let center_distance = sphere1.center.subtract(&sphere2.center).magnitude();
    let sum_radii = sphere1.radius + sphere2.radius;
    let diff_radii = (sphere1.radius - sphere2.radius).abs();

    let (intersects, intersection_type) = if center_distance > sum_radii {
        (false, "separate".to_string())
    } else if center_distance < diff_radii {
        (false, "one_inside_other".to_string())
    } else if center_distance == sum_radii {
        (true, "external_tangent".to_string())
    } else if center_distance == diff_radii {
        (true, "internal_tangent".to_string())
    } else {
        (true, "intersecting".to_string())
    };

    let mut intersection_circle = None;
    
    if intersects && intersection_type == "intersecting" {
        let a = (sphere1.radius * sphere1.radius - sphere2.radius * sphere2.radius + center_distance * center_distance) / (2.0 * center_distance);
        let h = (sphere1.radius * sphere1.radius - a * a).sqrt();
        
        let direction = sphere2.center.subtract(&sphere1.center).normalize();
        let circle_center = sphere1.center.add(&direction.scale(a));
        
        intersection_circle = Some(IntersectionCircle {
            center: circle_center,
            radius: h,
            normal: direction,
        });
    }

    Ok(SphereSphereResult {
        intersects,
        intersection_type,
        distance_between_centers: center_distance,
        intersection_circle,
    })
}

pub fn handle_cylinder_ray_intersection(input: CylinderRayInput) -> Result<CylinderRayResult, ErrorResponse> {
    let cylinder = input.cylinder;
    let ray = input.ray;

    if cylinder.radius <= 0.0 || cylinder.height <= 0.0 {
        return Err(ErrorResponse {
            error: "Cylinder radius and height must be positive".to_string(),
        });
    }

    let ray_dir = ray.direction.normalize();
    let cylinder_axis = cylinder.axis.normalize();
    
    let to_cylinder = ray.origin.subtract(&cylinder.center);
    let axis_dot_ray = cylinder_axis.dot(&ray_dir);
    let axis_dot_to_cylinder = cylinder_axis.dot(&to_cylinder);
    
    let a = ray_dir.dot(&ray_dir) - axis_dot_ray * axis_dot_ray;
    let b = 2.0 * (to_cylinder.dot(&ray_dir) - axis_dot_ray * axis_dot_to_cylinder);
    let c = to_cylinder.dot(&to_cylinder) - axis_dot_to_cylinder * axis_dot_to_cylinder - cylinder.radius * cylinder.radius;
    
    let discriminant = b * b - 4.0 * a * c;
    
    if discriminant < 0.0 {
        return Ok(CylinderRayResult {
            intersects: false,
            intersection_points: vec![],
            closest_distance: None,
        });
    }
    
    let mut intersection_points = vec![];
    let mut closest_distance = None;
    
    let sqrt_discriminant = discriminant.sqrt();
    let t1 = (-b - sqrt_discriminant) / (2.0 * a);
    let t2 = (-b + sqrt_discriminant) / (2.0 * a);
    
    for t in [t1, t2] {
        if t > 0.0 {
            let point = ray.origin.add(&ray_dir.scale(t));
            let point_on_axis = cylinder.center.add(&cylinder_axis.scale(
                cylinder_axis.dot(&point.subtract(&cylinder.center))
            ));
            
            let axis_distance = point_on_axis.subtract(&cylinder.center).magnitude();
            
            if axis_distance <= cylinder.height / 2.0 {
                let normal = point.subtract(&point_on_axis).normalize();
                
                intersection_points.push(IntersectionPoint {
                    point,
                    distance: t,
                    normal,
                });
                
                if closest_distance.is_none() || t < closest_distance.unwrap() {
                    closest_distance = Some(t);
                }
            }
        }
    }
    
    Ok(CylinderRayResult {
        intersects: !intersection_points.is_empty(),
        intersection_points,
        closest_distance,
    })
}

pub fn handle_aabb_ray_intersection(input: AABBRayInput) -> Result<AABBIntersectionResult, ErrorResponse> {
    let aabb = input.aabb;
    let ray = input.ray;

    if aabb.min.x >= aabb.max.x || aabb.min.y >= aabb.max.y || aabb.min.z >= aabb.max.z {
        return Err(ErrorResponse {
            error: "AABB min coordinates must be less than max coordinates".to_string(),
        });
    }

    let ray_dir = ray.direction.normalize();
    
    let inv_dir = Vector3::new(
        if ray_dir.x.abs() < 1e-10 { 1e10 } else { 1.0 / ray_dir.x },
        if ray_dir.y.abs() < 1e-10 { 1e10 } else { 1.0 / ray_dir.y },
        if ray_dir.z.abs() < 1e-10 { 1e10 } else { 1.0 / ray_dir.z },
    );

    let t1 = (aabb.min.x - ray.origin.x) * inv_dir.x;
    let t2 = (aabb.max.x - ray.origin.x) * inv_dir.x;
    let t3 = (aabb.min.y - ray.origin.y) * inv_dir.y;
    let t4 = (aabb.max.y - ray.origin.y) * inv_dir.y;
    let t5 = (aabb.min.z - ray.origin.z) * inv_dir.z;
    let t6 = (aabb.max.z - ray.origin.z) * inv_dir.z;

    let tmin = t1.min(t2).max(t3.min(t4)).max(t5.min(t6));
    let tmax = t1.max(t2).min(t3.max(t4)).min(t5.max(t6));

    if tmax < 0.0 || tmin > tmax {
        return Ok(AABBIntersectionResult {
            intersects: false,
            closest_distance: None,
            intersection_points: vec![],
        });
    }

    let mut intersection_points = vec![];
    let mut closest_distance = None;

    if tmin > 0.0 {
        let point = ray.origin.add(&ray_dir.scale(tmin));
        let normal = calculate_aabb_normal(&aabb, &point);
        
        intersection_points.push(IntersectionPoint {
            point,
            distance: tmin,
            normal,
        });
        
        closest_distance = Some(tmin);
    }

    if tmax > 0.0 && tmax != tmin {
        let point = ray.origin.add(&ray_dir.scale(tmax));
        let normal = calculate_aabb_normal(&aabb, &point);
        
        intersection_points.push(IntersectionPoint {
            point,
            distance: tmax,
            normal,
        });
        
        if closest_distance.is_none() || tmax < closest_distance.unwrap() {
            closest_distance = Some(tmax);
        }
    }

    Ok(AABBIntersectionResult {
        intersects: !intersection_points.is_empty(),
        closest_distance,
        intersection_points,
    })
}

fn calculate_aabb_normal(aabb: &AABB, point: &Vector3) -> Vector3 {
    let epsilon = 1e-6;
    
    if (point.x - aabb.min.x).abs() < epsilon {
        Vector3::new(-1.0, 0.0, 0.0)
    } else if (point.x - aabb.max.x).abs() < epsilon {
        Vector3::new(1.0, 0.0, 0.0)
    } else if (point.y - aabb.min.y).abs() < epsilon {
        Vector3::new(0.0, -1.0, 0.0)
    } else if (point.y - aabb.max.y).abs() < epsilon {
        Vector3::new(0.0, 1.0, 0.0)
    } else if (point.z - aabb.min.z).abs() < epsilon {
        Vector3::new(0.0, 0.0, -1.0)
    } else if (point.z - aabb.max.z).abs() < epsilon {
        Vector3::new(0.0, 0.0, 1.0)
    } else {
        Vector3::new(0.0, 0.0, 0.0)
    }
}

pub fn handle_aabb_aabb_intersection(input: AABBAABBInput) -> Result<AABBIntersectionResult, ErrorResponse> {
    let aabb1 = input.aabb1;
    let aabb2 = input.aabb2;

    if aabb1.min.x >= aabb1.max.x || aabb1.min.y >= aabb1.max.y || aabb1.min.z >= aabb1.max.z {
        return Err(ErrorResponse {
            error: "AABB1 min coordinates must be less than max coordinates".to_string(),
        });
    }

    if aabb2.min.x >= aabb2.max.x || aabb2.min.y >= aabb2.max.y || aabb2.min.z >= aabb2.max.z {
        return Err(ErrorResponse {
            error: "AABB2 min coordinates must be less than max coordinates".to_string(),
        });
    }

    let intersects = aabb1.min.x <= aabb2.max.x && aabb1.max.x >= aabb2.min.x &&
                     aabb1.min.y <= aabb2.max.y && aabb1.max.y >= aabb2.min.y &&
                     aabb1.min.z <= aabb2.max.z && aabb1.max.z >= aabb2.min.z;

    let mut intersection_points = vec![];
    let mut closest_distance = None;

    if intersects {
        let overlap_min = Vector3::new(
            aabb1.min.x.max(aabb2.min.x),
            aabb1.min.y.max(aabb2.min.y),
            aabb1.min.z.max(aabb2.min.z),
        );
        
        let overlap_max = Vector3::new(
            aabb1.max.x.min(aabb2.max.x),
            aabb1.max.y.min(aabb2.max.y),
            aabb1.max.z.min(aabb2.max.z),
        );
        
        let center = Vector3::new(
            (overlap_min.x + overlap_max.x) / 2.0,
            (overlap_min.y + overlap_max.y) / 2.0,
            (overlap_min.z + overlap_max.z) / 2.0,
        );
        
        intersection_points.push(IntersectionPoint {
            point: center,
            distance: 0.0,
            normal: Vector3::new(0.0, 0.0, 1.0),
        });
        
        closest_distance = Some(0.0);
    }

    Ok(AABBIntersectionResult {
        intersects,
        closest_distance,
        intersection_points,
    })
}