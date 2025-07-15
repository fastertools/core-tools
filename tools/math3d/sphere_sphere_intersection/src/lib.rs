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
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereSphereInput {
    pub sphere1: Sphere,
    pub sphere2: Sphere,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SphereSphereResult {
    pub intersects: bool,
    pub intersection_type: String,
    pub distance_between_centers: f64,
    pub intersection_circle: Option<IntersectionCircle>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionCircle {
    pub center: Vector3,
    pub radius: f64,
    pub normal: Vector3,
}

impl Vector3 {
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
}

#[tool]
pub fn sphere_sphere_intersection(input: SphereSphereInput) -> Result<SphereSphereResult, String> {
    let sphere1 = input.sphere1;
    let sphere2 = input.sphere2;

    if sphere1.radius <= 0.0 || sphere2.radius <= 0.0 {
        return Err("Sphere radii must be positive".to_string());
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