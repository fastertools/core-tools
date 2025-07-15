use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct Cylinder {
    pub center: Vector3,
    pub axis: Vector3,
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CylinderRayInput {
    pub cylinder: Cylinder,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CylinderRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

impl Vector3 {
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
            Vector3 { x: 0.0, y: 0.0, z: 0.0 }
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
}

#[tool]
pub fn cylinder_ray_intersection(input: CylinderRayInput) -> Result<CylinderRayResult, String> {
    let cylinder = input.cylinder;
    let ray = input.ray;

    if cylinder.radius <= 0.0 || cylinder.height <= 0.0 {
        return Err("Cylinder radius and height must be positive".to_string());
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