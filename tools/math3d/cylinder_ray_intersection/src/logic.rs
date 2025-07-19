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
pub struct Cylinder {
    pub center: Vector3,
    pub axis: Vector3,
    pub radius: f64,
    pub height: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderRayInput {
    pub cylinder: Cylinder,
    pub ray: Ray,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IntersectionPoint {
    pub point: Vector3,
    pub distance: f64,
    pub normal: Vector3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CylinderRayResult {
    pub intersects: bool,
    pub intersection_points: Vec<IntersectionPoint>,
    pub closest_distance: Option<f64>,
}

impl Vector3 {
    #[allow(dead_code)]
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
            Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }
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

impl Cylinder {
    #[allow(dead_code)]
    pub fn new(center: Vector3, axis: Vector3, radius: f64, height: f64) -> Self {
        Cylinder {
            center,
            axis,
            radius,
            height,
        }
    }
}

impl Ray {
    #[allow(dead_code)]
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray { origin, direction }
    }
}

pub fn cylinder_ray_intersection_logic(
    input: CylinderRayInput,
) -> Result<CylinderRayResult, String> {
    let cylinder = input.cylinder;
    let ray = input.ray;

    // Validate cylinder parameters
    if cylinder.radius <= 0.0 || cylinder.height <= 0.0 {
        return Err("Cylinder radius and height must be positive".to_string());
    }

    // Validate for NaN and infinite values
    if cylinder.center.x.is_nan()
        || cylinder.center.x.is_infinite()
        || cylinder.center.y.is_nan()
        || cylinder.center.y.is_infinite()
        || cylinder.center.z.is_nan()
        || cylinder.center.z.is_infinite()
    {
        return Err("Cylinder center coordinates must be finite".to_string());
    }

    if cylinder.axis.x.is_nan()
        || cylinder.axis.x.is_infinite()
        || cylinder.axis.y.is_nan()
        || cylinder.axis.y.is_infinite()
        || cylinder.axis.z.is_nan()
        || cylinder.axis.z.is_infinite()
    {
        return Err("Cylinder axis coordinates must be finite".to_string());
    }

    if cylinder.radius.is_nan() || cylinder.radius.is_infinite() {
        return Err("Cylinder radius must be finite".to_string());
    }

    if cylinder.height.is_nan() || cylinder.height.is_infinite() {
        return Err("Cylinder height must be finite".to_string());
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

    // Check for zero vectors
    if ray.direction.magnitude() == 0.0 {
        return Err("Ray direction cannot be zero vector".to_string());
    }

    if cylinder.axis.magnitude() == 0.0 {
        return Err("Cylinder axis cannot be zero vector".to_string());
    }

    let ray_dir = ray.direction.normalize();
    let cylinder_axis = cylinder.axis.normalize();

    let to_cylinder = ray.origin.subtract(&cylinder.center);
    let axis_dot_ray = cylinder_axis.dot(&ray_dir);
    let axis_dot_to_cylinder = cylinder_axis.dot(&to_cylinder);

    let a = ray_dir.dot(&ray_dir) - axis_dot_ray * axis_dot_ray;
    let b = 2.0 * (to_cylinder.dot(&ray_dir) - axis_dot_ray * axis_dot_to_cylinder);
    let c = to_cylinder.dot(&to_cylinder)
        - axis_dot_to_cylinder * axis_dot_to_cylinder
        - cylinder.radius * cylinder.radius;

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
            let point_on_axis = cylinder
                .center
                .add(&cylinder_axis.scale(cylinder_axis.dot(&point.subtract(&cylinder.center))));

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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_ray_hits_cylinder_center() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_points.len(), 2);
        assert!(result.closest_distance.is_some());

        let closest = result.closest_distance.unwrap();
        assert!((closest - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ray_misses_cylinder() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 3.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_points.len(), 0);
        assert!(result.closest_distance.is_none());
    }

    #[test]
    fn test_ray_through_cylinder_diagonal() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                4.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.5), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        // This ray passes through the cylinder at height 0.5 (within ±2 height bounds)
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }

    #[test]
    fn test_ray_outside_cylinder_height() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 3.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_points.len(), 0);
    }

    #[test]
    fn test_negative_radius_error() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                -1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cylinder radius and height must be positive"
        );
    }

    #[test]
    fn test_zero_height_error() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                0.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cylinder radius and height must be positive"
        );
    }

    #[test]
    fn test_nan_cylinder_center() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(f64::NAN, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cylinder center coordinates must be finite"
        );
    }

    #[test]
    fn test_infinite_cylinder_axis() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(f64::INFINITY, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cylinder axis coordinates must be finite"
        );
    }

    #[test]
    fn test_nan_ray_origin() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(
                Vector3::new(f64::NAN, 0.0, 0.0),
                Vector3::new(1.0, 0.0, 0.0),
            ),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ray origin coordinates must be finite");
    }

    #[test]
    fn test_infinite_ray_direction() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(
                Vector3::new(-2.0, 0.0, 0.0),
                Vector3::new(f64::INFINITY, 0.0, 0.0),
            ),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Ray direction coordinates must be finite"
        );
    }

    #[test]
    fn test_zero_ray_direction() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Ray direction cannot be zero vector");
    }

    #[test]
    fn test_zero_cylinder_axis() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 0.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cylinder axis cannot be zero vector");
    }

    #[test]
    fn test_diagonal_cylinder_intersection() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(1.0, 1.0, 0.0),
                1.0,
                4.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 1.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }

    #[test]
    fn test_intersection_normal_vectors() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1.0,
                2.0,
            ),
            ray: Ray::new(Vector3::new(-2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);

        // Check that normals are unit vectors
        for intersection in &result.intersection_points {
            let normal_magnitude = intersection.normal.magnitude();
            assert!((normal_magnitude - 1.0).abs() < EPSILON);
        }
    }

    #[test]
    fn test_large_cylinder_coordinates() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(1000.0, 2000.0, 3000.0),
                Vector3::new(0.0, 0.0, 1.0),
                100.0,
                200.0,
            ),
            ray: Ray::new(
                Vector3::new(800.0, 2000.0, 3000.0),
                Vector3::new(1.0, 0.0, 0.0),
            ),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }

    #[test]
    fn test_small_cylinder_high_precision() {
        let input = CylinderRayInput {
            cylinder: Cylinder::new(
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                1e-6,
                2e-6,
            ),
            ray: Ray::new(Vector3::new(-1e-5, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)),
        };

        let result = cylinder_ray_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert!(!result.intersection_points.is_empty());
    }
}
