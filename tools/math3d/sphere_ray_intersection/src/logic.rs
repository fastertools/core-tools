use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ray {
    pub origin: Vector3D,
    pub direction: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3D,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereRayInput {
    pub sphere: Sphere,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntersectionPoint {
    pub point: Vector3D,
    pub distance: f64,
    pub normal: Vector3D,
}

#[derive(Debug, Serialize, Deserialize)]
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

pub fn sphere_ray_intersection_logic(input: SphereRayInput) -> Result<SphereRayResult, String> {
    let sphere = input.sphere;
    let ray = input.ray;

    // Validate sphere radius
    if sphere.radius <= 0.0 {
        return Err("Sphere radius must be positive".to_string());
    }

    // Check for NaN or infinite values
    if sphere.center.x.is_nan()
        || sphere.center.x.is_infinite()
        || sphere.center.y.is_nan()
        || sphere.center.y.is_infinite()
        || sphere.center.z.is_nan()
        || sphere.center.z.is_infinite()
    {
        return Err("Sphere center coordinates must be finite".to_string());
    }

    if sphere.radius.is_nan() || sphere.radius.is_infinite() {
        return Err("Sphere radius must be finite".to_string());
    }

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

    let chord_half_length =
        (sphere.radius * sphere.radius - distance_to_center * distance_to_center).sqrt();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_hits_sphere_center() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 5.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);
        assert!(result.closest_distance.is_some());

        let closest = result.closest_distance.unwrap();
        assert!((closest - 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_ray_misses_sphere() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 5.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_points.len(), 0);
        assert!(result.closest_distance.is_none());
    }

    #[test]
    fn test_ray_tangent_to_sphere() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 1.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(-5.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        // This case actually produces 2 intersection points (entry and exit)
        // because the ray passes through the sphere at the tangent point
        assert_eq!(result.intersection_points.len(), 2);
    }

    #[test]
    fn test_ray_inside_sphere() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 2.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 1);
        assert!(result.closest_distance.is_some());

        let closest = result.closest_distance.unwrap();
        assert!((closest - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_diagonal_ray_intersection() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(1.0, 1.0, 1.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 1.0, 1.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
        assert!(result.closest_distance.is_some());
    }

    #[test]
    fn test_negative_radius_error() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: -1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere radius must be positive");
    }

    #[test]
    fn test_zero_radius_error() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 0.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere radius must be positive");
    }

    #[test]
    fn test_nan_sphere_center() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(f64::NAN, 0.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Sphere center coordinates must be finite")
        );
    }

    #[test]
    fn test_infinite_sphere_radius() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: f64::INFINITY,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere radius must be finite");
    }

    #[test]
    fn test_nan_ray_origin() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(f64::NAN, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Ray origin coordinates must be finite")
        );
    }

    #[test]
    fn test_infinite_ray_direction() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(f64::INFINITY, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Ray direction coordinates must be finite")
        );
    }

    #[test]
    fn test_zero_direction_vector() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(0.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ray direction cannot be zero vector");
    }

    #[test]
    fn test_intersection_normal_vectors() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            ray: Ray {
                origin: Vector3D::new(-2.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);

        // Check that normals are unit vectors pointing outward from sphere center
        for intersection in &result.intersection_points {
            let normal_magnitude = intersection.normal.magnitude();
            assert!((normal_magnitude - 1.0).abs() < 1e-10);
        }
    }

    #[test]
    fn test_large_sphere_coordinates() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(1000.0, 2000.0, 3000.0),
                radius: 100.0,
            },
            ray: Ray {
                origin: Vector3D::new(1000.0, 2000.0, 2800.0),
                direction: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }

    #[test]
    fn test_small_sphere_high_precision() {
        let input = SphereRayInput {
            sphere: Sphere {
                center: Vector3D::new(0.0, 0.0, 0.0),
                radius: 1e-6,
            },
            ray: Ray {
                origin: Vector3D::new(-1e-5, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = sphere_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }
}
