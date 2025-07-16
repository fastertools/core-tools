use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sphere {
    pub center: Vector3,
    pub radius: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SphereSphereInput {
    pub sphere1: Sphere,
    pub sphere2: Sphere,
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

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
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

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

pub fn sphere_sphere_intersection_logic(input: SphereSphereInput) -> Result<SphereSphereResult, String> {
    let sphere1 = input.sphere1;
    let sphere2 = input.sphere2;

    // Validate sphere radii
    if sphere1.radius <= 0.0 || sphere2.radius <= 0.0 {
        return Err("Sphere radii must be positive".to_string());
    }

    // Check for NaN or infinite values in sphere1
    if sphere1.center.x.is_nan() || sphere1.center.x.is_infinite() ||
       sphere1.center.y.is_nan() || sphere1.center.y.is_infinite() ||
       sphere1.center.z.is_nan() || sphere1.center.z.is_infinite() {
        return Err("Sphere1 center coordinates must be finite".to_string());
    }

    if sphere1.radius.is_nan() || sphere1.radius.is_infinite() {
        return Err("Sphere1 radius must be finite".to_string());
    }

    // Check for NaN or infinite values in sphere2
    if sphere2.center.x.is_nan() || sphere2.center.x.is_infinite() ||
       sphere2.center.y.is_nan() || sphere2.center.y.is_infinite() ||
       sphere2.center.z.is_nan() || sphere2.center.z.is_infinite() {
        return Err("Sphere2 center coordinates must be finite".to_string());
    }

    if sphere2.radius.is_nan() || sphere2.radius.is_infinite() {
        return Err("Sphere2 radius must be finite".to_string());
    }

    let center_distance = sphere1.center.subtract(&sphere2.center).magnitude();
    let sum_radii = sphere1.radius + sphere2.radius;
    let diff_radii = (sphere1.radius - sphere2.radius).abs();

    // Use epsilon for floating point comparisons
    const EPSILON: f64 = 1e-10;

    let (intersects, intersection_type) = if center_distance > sum_radii + EPSILON {
        (false, "separate".to_string())
    } else if center_distance < diff_radii - EPSILON {
        (false, "one_inside_other".to_string())
    } else if (center_distance - sum_radii).abs() < EPSILON {
        (true, "external_tangent".to_string())
    } else if (center_distance - diff_radii).abs() < EPSILON {
        (true, "internal_tangent".to_string())
    } else {
        (true, "intersecting".to_string())
    };

    let mut intersection_circle = None;
    
    if intersects && intersection_type == "intersecting" {
        // Calculate intersection circle using analytical geometry
        let a = (sphere1.radius * sphere1.radius - sphere2.radius * sphere2.radius + center_distance * center_distance) / (2.0 * center_distance);
        let h_squared = sphere1.radius * sphere1.radius - a * a;
        
        // Ensure h_squared is non-negative to avoid NaN
        if h_squared >= 0.0 {
            let h = h_squared.sqrt();
            
            let direction = sphere2.center.subtract(&sphere1.center).normalize();
            let circle_center = sphere1.center.add(&direction.scale(a));
            
            intersection_circle = Some(IntersectionCircle {
                center: circle_center,
                radius: h,
                normal: direction,
            });
        }
    }

    Ok(SphereSphereResult {
        intersects,
        intersection_type,
        distance_between_centers: center_distance,
        intersection_circle,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_spheres_separate() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(5.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "separate");
        assert!((result.distance_between_centers - 5.0).abs() < EPSILON);
        assert!(result.intersection_circle.is_none());
    }

    #[test]
    fn test_spheres_intersecting() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0),
            sphere2: Sphere::new(Vector3::new(2.0, 0.0, 0.0), 2.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!((result.distance_between_centers - 2.0).abs() < EPSILON);
        assert!(result.intersection_circle.is_some());
        
        let circle = result.intersection_circle.unwrap();
        assert!((circle.center.x - 1.0).abs() < EPSILON);
        assert!((circle.center.y - 0.0).abs() < EPSILON);
        assert!((circle.center.z - 0.0).abs() < EPSILON);
        
        let expected_radius = (3.0_f64).sqrt(); // sqrt(4 - 1) = sqrt(3)
        assert!((circle.radius - expected_radius).abs() < EPSILON);
    }

    #[test]
    fn test_spheres_external_tangent() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(3.0, 0.0, 0.0), 2.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "external_tangent");
        assert!((result.distance_between_centers - 3.0).abs() < EPSILON);
        assert!(result.intersection_circle.is_none());
    }

    #[test]
    fn test_spheres_internal_tangent() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 3.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 2.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "internal_tangent");
        assert!((result.distance_between_centers - 1.0).abs() < EPSILON);
        assert!(result.intersection_circle.is_none());
    }

    #[test]
    fn test_spheres_one_inside_other() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 5.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 2.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "one_inside_other");
        assert!((result.distance_between_centers - 1.0).abs() < EPSILON);
        assert!(result.intersection_circle.is_none());
    }

    #[test]
    fn test_identical_spheres() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        // Identical spheres with same center and radius should be "internal_tangent"
        // because distance (0) equals the difference in radii (0)
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "internal_tangent");
        assert!((result.distance_between_centers - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_diagonal_intersection() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 2.0),
            sphere2: Sphere::new(Vector3::new(1.0, 1.0, 1.0), 2.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersection_circle.is_some());
        
        let expected_distance = (3.0_f64).sqrt(); // sqrt(1^2 + 1^2 + 1^2)
        assert!((result.distance_between_centers - expected_distance).abs() < EPSILON);
    }

    #[test]
    fn test_negative_radius_error() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), -1.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere radii must be positive");
    }

    #[test]
    fn test_zero_radius_error() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 0.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere radii must be positive");
    }

    #[test]
    fn test_nan_sphere1_center() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(f64::NAN, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere1 center coordinates must be finite");
    }

    #[test]
    fn test_infinite_sphere1_radius() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), f64::INFINITY),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere1 radius must be finite");
    }

    #[test]
    fn test_nan_sphere2_center() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(f64::NAN, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere2 center coordinates must be finite");
    }

    #[test]
    fn test_infinite_sphere2_radius() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(1.0, 0.0, 0.0), f64::INFINITY),
        };

        let result = sphere_sphere_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Sphere2 radius must be finite");
    }

    #[test]
    fn test_intersection_circle_properties() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(1.5, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersection_circle.is_some());
        
        let circle = result.intersection_circle.unwrap();
        // Check that normal vector is unit length
        let normal_magnitude = circle.normal.magnitude();
        assert!((normal_magnitude - 1.0).abs() < EPSILON);
        
        // Check intersection circle radius is positive
        assert!(circle.radius > 0.0);
    }

    #[test]
    fn test_large_spheres() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(1000.0, 2000.0, 3000.0), 100.0),
            sphere2: Sphere::new(Vector3::new(1150.0, 2000.0, 3000.0), 100.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!((result.distance_between_centers - 150.0).abs() < EPSILON);
    }

    #[test]
    fn test_small_spheres_high_precision() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1e-6),
            sphere2: Sphere::new(Vector3::new(1e-6, 0.0, 0.0), 1e-6),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersection_circle.is_some());
    }

    #[test]
    fn test_nearly_tangent_spheres() {
        let input = SphereSphereInput {
            sphere1: Sphere::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            sphere2: Sphere::new(Vector3::new(2.0 + 1e-15, 0.0, 0.0), 1.0),
        };

        let result = sphere_sphere_intersection_logic(input).unwrap();
        // With epsilon tolerance, this should be considered external tangent
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "external_tangent");
    }
}