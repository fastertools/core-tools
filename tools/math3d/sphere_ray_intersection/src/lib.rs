use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Sphere {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereRayInput {
    pub sphere: Sphere,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3D,
    pub distance: f64,
    pub normal: Vector3D,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vector3D {
        let mag = self.magnitude();
        if mag > 0.0 {
            Vector3D {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        } else {
            Vector3D::new(0.0, 0.0, 0.0)
        }
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

#[tool]
pub fn sphere_ray_intersection(input: SphereRayInput) -> Result<SphereRayResult, String> {
    let sphere = input.sphere;
    let ray = input.ray;

    if sphere.radius <= 0.0 {
        return Err("Sphere radius must be positive".to_string());
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