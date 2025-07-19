use serde::{Deserialize, Serialize};

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
#[allow(clippy::upper_case_acronyms)]
pub struct AABB {
    pub min: Vector3,
    pub max: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AABBRayInput {
    pub aabb: AABB,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
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
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
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

pub fn ray_aabb_intersection_logic(input: AABBRayInput) -> Result<AABBIntersectionResult, String> {
    let aabb = input.aabb;
    let ray = input.ray;

    // Validate AABB coordinates
    if aabb.min.x >= aabb.max.x || aabb.min.y >= aabb.max.y || aabb.min.z >= aabb.max.z {
        return Err("AABB min coordinates must be less than max coordinates".to_string());
    }

    // Check for NaN or infinite values in AABB
    if aabb.min.x.is_nan()
        || aabb.min.x.is_infinite()
        || aabb.min.y.is_nan()
        || aabb.min.y.is_infinite()
        || aabb.min.z.is_nan()
        || aabb.min.z.is_infinite()
    {
        return Err("AABB min coordinates must be finite".to_string());
    }

    if aabb.max.x.is_nan()
        || aabb.max.x.is_infinite()
        || aabb.max.y.is_nan()
        || aabb.max.y.is_infinite()
        || aabb.max.z.is_nan()
        || aabb.max.z.is_infinite()
    {
        return Err("AABB max coordinates must be finite".to_string());
    }

    // Check for NaN or infinite values in ray
    if ray.origin.x.is_nan()
        || ray.origin.x.is_infinite()
        || ray.origin.y.is_nan()
        || ray.origin.y.is_infinite()
        || ray.origin.z.is_nan()
        || ray.origin.z.is_infinite()
    {
        return Err("Ray origin coordinates must be finite".to_string());
    }

    if ray.direction.x.is_nan()
        || ray.direction.x.is_infinite()
        || ray.direction.y.is_nan()
        || ray.direction.y.is_infinite()
        || ray.direction.z.is_nan()
        || ray.direction.z.is_infinite()
    {
        return Err("Ray direction coordinates must be finite".to_string());
    }

    // Check for zero direction vector
    if ray.direction.magnitude() == 0.0 {
        return Err("Ray direction cannot be zero vector".to_string());
    }

    let ray_dir = ray.direction.normalize();

    let inv_dir = Vector3::new(
        if ray_dir.x.abs() < 1e-10 {
            1e10
        } else {
            1.0 / ray_dir.x
        },
        if ray_dir.y.abs() < 1e-10 {
            1e10
        } else {
            1.0 / ray_dir.y
        },
        if ray_dir.z.abs() < 1e-10 {
            1e10
        } else {
            1.0 / ray_dir.z
        },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_hits_aabb_front_face() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(-1.0, -1.0, -1.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, -5.0),
                direction: Vector3::new(0.0, 0.0, 1.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);
        assert!(result.closest_distance.is_some());

        let closest = result.closest_distance.unwrap();
        assert!((closest - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_ray_misses_aabb() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(2.0, 2.0, 2.0),
                max: Vector3::new(3.0, 3.0, 3.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_points.len(), 0);
        assert!(result.closest_distance.is_none());
    }

    #[test]
    fn test_ray_inside_aabb() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(-2.0, -2.0, -2.0),
                max: Vector3::new(2.0, 2.0, 2.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 1);
        assert!(result.closest_distance.is_some());

        let closest = result.closest_distance.unwrap();
        assert!((closest - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_diagonal_ray_intersection() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(2.0, 2.0, 2.0),
            },
            ray: Ray {
                origin: Vector3::new(-1.0, -1.0, -1.0),
                direction: Vector3::new(1.0, 1.0, 1.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
        assert!(result.closest_distance.is_some());
    }

    #[test]
    fn test_invalid_aabb_error() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(1.0, 1.0, 1.0),
                max: Vector3::new(0.0, 0.0, 0.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "AABB min coordinates must be less than max coordinates"
        );
    }

    #[test]
    fn test_nan_aabb_min() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(f64::NAN, 0.0, 0.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "AABB min coordinates must be finite");
    }

    #[test]
    fn test_infinite_aabb_max() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(f64::INFINITY, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "AABB max coordinates must be finite");
    }

    #[test]
    fn test_nan_ray_origin() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(f64::NAN, 0.0, 0.0),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ray origin coordinates must be finite");
    }

    #[test]
    fn test_infinite_ray_direction() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(f64::INFINITY, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Ray direction coordinates must be finite"
        );
    }

    #[test]
    fn test_zero_direction_vector() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, 0.0),
                direction: Vector3::new(0.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ray direction cannot be zero vector");
    }

    #[test]
    fn test_ray_parallel_to_face() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(-1.0, 0.5, 0.5),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);
    }

    #[test]
    fn test_aabb_normal_calculation() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(-1.0, -1.0, -1.0),
                max: Vector3::new(1.0, 1.0, 1.0),
            },
            ray: Ray {
                origin: Vector3::new(0.0, 0.0, -5.0),
                direction: Vector3::new(0.0, 0.0, 1.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);

        // Check that normals are unit vectors
        for intersection in &result.intersection_points {
            let normal_magnitude = intersection.normal.magnitude();
            assert!((normal_magnitude - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_large_aabb_coordinates() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(1000.0, 2000.0, 3000.0),
                max: Vector3::new(1100.0, 2100.0, 3100.0),
            },
            ray: Ray {
                origin: Vector3::new(1050.0, 2050.0, 2900.0),
                direction: Vector3::new(0.0, 0.0, 1.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }

    #[test]
    fn test_small_aabb_high_precision() {
        let input = AABBRayInput {
            aabb: AABB {
                min: Vector3::new(0.0, 0.0, 0.0),
                max: Vector3::new(1e-6, 1e-6, 1e-6),
            },
            ray: Ray {
                origin: Vector3::new(-1e-5, 5e-7, 5e-7),
                direction: Vector3::new(1.0, 0.0, 0.0),
            },
        };

        let result = ray_aabb_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }
}
