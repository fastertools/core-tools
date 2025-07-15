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
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AABBRayInput {
    pub aabb: AABB,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AABBIntersectionResult {
    pub intersects: bool,
    pub closest_distance: Option<f64>,
    pub intersection_points: Vec<IntersectionPoint>,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
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

#[tool]
pub fn ray_aabb_intersection(input: AABBRayInput) -> Result<AABBIntersectionResult, String> {
    let aabb = input.aabb;
    let ray = input.ray;

    if aabb.min.x >= aabb.max.x || aabb.min.y >= aabb.max.y || aabb.min.z >= aabb.max.z {
        return Err("AABB min coordinates must be less than max coordinates".to_string());
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