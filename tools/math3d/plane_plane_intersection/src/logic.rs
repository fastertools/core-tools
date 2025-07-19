use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

const EPSILON: f64 = 1e-10;

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, PartialEq)]
pub struct Line3D {
    /// A point on the line
    pub point: Vector3D,
    /// Direction vector of the line
    pub direction: Vector3D,
}

#[derive(Deserialize, Serialize, JsonSchema, Clone, Debug, PartialEq)]
pub struct Plane3D {
    /// A point on the plane
    pub point: Vector3D,
    /// Normal vector to the plane
    pub normal: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanePlaneIntersectionInput {
    /// First plane
    pub plane1: Plane3D,
    /// Second plane
    pub plane2: Plane3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanePlaneIntersectionOutput {
    /// Type of intersection: "intersecting", "parallel", or "coincident"
    pub intersection_type: String,
    /// Whether the planes intersect
    pub intersects: bool,
    /// The intersection line if planes intersect
    pub intersection_line: Option<Line3D>,
    /// Whether the planes are parallel
    pub are_parallel: bool,
    /// Whether the planes are coincident (same plane)
    pub are_coincident: bool,
    /// Angle between planes in radians
    pub angle_radians: f64,
    /// Angle between planes in degrees
    pub angle_degrees: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag < EPSILON {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(self.scale(1.0 / mag))
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Line3D {
    pub fn new(point: Vector3D, direction: Vector3D) -> Result<Self, String> {
        if !point.is_valid() || !direction.is_valid() {
            return Err("Line3D contains invalid coordinates".to_string());
        }
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }

    pub fn is_valid(&self) -> bool {
        self.point.is_valid() && self.direction.is_valid() && !self.direction.is_zero()
    }
}

impl Plane3D {
    pub fn new(point: Vector3D, normal: Vector3D) -> Result<Self, String> {
        if !point.is_valid() || !normal.is_valid() {
            return Err("Plane3D contains invalid coordinates".to_string());
        }
        if normal.is_zero() {
            return Err("Normal vector cannot be zero".to_string());
        }
        Ok(Plane3D { point, normal })
    }

    pub fn is_valid(&self) -> bool {
        self.point.is_valid() && self.normal.is_valid() && !self.normal.is_zero()
    }

    pub fn is_parallel_to(&self, other: &Plane3D) -> bool {
        let cross = self.normal.cross(&other.normal);
        cross.is_zero()
    }

    pub fn angle_with(&self, other: &Plane3D) -> Result<f64, String> {
        let n1 = self.normal.normalize()?;
        let n2 = other.normal.normalize()?;
        let dot = n1.dot(&n2);
        let clamped = dot.clamp(-1.0, 1.0);
        Ok(clamped.acos())
    }

    pub fn distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };

        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit).abs()
    }
}

pub fn plane_plane_intersection_logic(
    input: PlanePlaneIntersectionInput,
) -> Result<PlanePlaneIntersectionOutput, String> {
    let plane1 = &input.plane1;
    let plane2 = &input.plane2;

    // Validate inputs
    if !plane1.is_valid() {
        return Err("Plane1 is invalid: contains NaN/infinite values or zero normal".to_string());
    }
    if !plane2.is_valid() {
        return Err("Plane2 is invalid: contains NaN/infinite values or zero normal".to_string());
    }

    let are_parallel = plane1.is_parallel_to(plane2);
    let angle_radians = plane1.angle_with(plane2).unwrap_or(0.0);
    let angle_degrees = angle_radians.to_degrees();

    if are_parallel {
        // Check if planes are coincident
        let distance = plane1.distance_to_point(&plane2.point);
        let are_coincident = distance < EPSILON;

        return Ok(PlanePlaneIntersectionOutput {
            intersection_type: if are_coincident {
                "coincident".to_string()
            } else {
                "parallel".to_string()
            },
            intersects: are_coincident,
            intersection_line: None,
            are_parallel: true,
            are_coincident,
            angle_radians,
            angle_degrees,
        });
    }

    // Planes intersect in a line
    let direction = plane1.normal.cross(&plane2.normal);

    // Find a point on the intersection line
    // We'll find the point closest to the origin that lies on both planes
    let n1 = &plane1.normal;
    let n2 = &plane2.normal;
    let d1 = n1.dot(&plane1.point);
    let d2 = n2.dot(&plane2.point);

    // Find the direction with the largest component to avoid division by small numbers
    let abs_dir = Vector3D::new(direction.x.abs(), direction.y.abs(), direction.z.abs());

    let intersection_point = if abs_dir.z >= abs_dir.x && abs_dir.z >= abs_dir.y {
        // Solve for x and y, set z = 0
        let det = n1.x * n2.y - n1.y * n2.x;
        if det.abs() < EPSILON {
            return Err("Cannot find intersection point: determinant too small".to_string());
        }
        let x = (d1 * n2.y - d2 * n1.y) / det;
        let y = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, y, 0.0)
    } else if abs_dir.y >= abs_dir.x {
        // Solve for x and z, set y = 0
        let det = n1.x * n2.z - n1.z * n2.x;
        if det.abs() < EPSILON {
            return Err("Cannot find intersection point: determinant too small".to_string());
        }
        let x = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, 0.0, z)
    } else {
        // Solve for y and z, set x = 0
        let det = n1.y * n2.z - n1.z * n2.y;
        if det.abs() < EPSILON {
            return Err("Cannot find intersection point: determinant too small".to_string());
        }
        let y = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.y - d1 * n2.y) / det;
        Vector3D::new(0.0, y, z)
    };

    let intersection_line = Line3D::new(intersection_point, direction)?;

    // Validate result
    if !intersection_line.is_valid() {
        return Err("Generated intersection line is invalid".to_string());
    }

    Ok(PlanePlaneIntersectionOutput {
        intersection_type: "intersecting".to_string(),
        intersects: true,
        intersection_line: Some(intersection_line),
        are_parallel: false,
        are_coincident: false,
        angle_radians,
        angle_degrees,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_planes_different() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 1.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        assert_eq!(result.intersection_type, "parallel");
        assert!(!result.intersects);
        assert!(result.are_parallel);
        assert!(!result.are_coincident);
        assert!(result.intersection_line.is_none());
        assert!(result.angle_radians.abs() < 1e-15);
    }

    #[test]
    fn test_coincident_planes() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(1.0, 1.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        assert_eq!(result.intersection_type, "coincident");
        assert!(result.intersects);
        assert!(result.are_parallel);
        assert!(result.are_coincident);
        assert!(result.intersection_line.is_none());
    }

    #[test]
    fn test_intersecting_planes() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 1.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersects);
        assert!(!result.are_parallel);
        assert!(!result.are_coincident);
        assert!(result.intersection_line.is_some());

        let line = result.intersection_line.unwrap();
        // Should be along z-axis
        assert!(line.direction.x.abs() < 1e-15);
        assert!(line.direction.y.abs() < 1e-15);
        assert!(line.direction.z.abs() > 1e-15);

        // Angle should be 90 degrees
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-14);
        assert!((result.angle_degrees - 90.0).abs() < 1e-12);
    }

    #[test]
    fn test_perpendicular_planes() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 1.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        // Should be perpendicular (90 degrees)
        assert!((result.angle_degrees - 90.0).abs() < 1e-12);
        assert!((result.angle_radians - std::f64::consts::PI / 2.0).abs() < 1e-14);
    }

    #[test]
    fn test_acute_angle_planes() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 1.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        // Should be 45 degrees
        assert!((result.angle_degrees - 45.0).abs() < 1e-12);
        assert!((result.angle_radians - std::f64::consts::PI / 4.0).abs() < 1e-14);
    }

    #[test]
    fn test_invalid_plane_zero_normal() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Plane1 is invalid: contains NaN/infinite values or zero normal"
        );
    }

    #[test]
    fn test_invalid_plane_nan_values() {
        let plane1 = Plane3D {
            point: Vector3D::new(f64::NAN, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 1.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Plane1 is invalid: contains NaN/infinite values or zero normal"
        );
    }

    #[test]
    fn test_vector_operations() {
        let v1 = Vector3D::new(1.0, 2.0, 3.0);
        let v2 = Vector3D::new(4.0, 5.0, 6.0);

        // Test dot product
        let dot = v1.dot(&v2);
        assert!((dot - 32.0).abs() < 1e-15); // 1*4 + 2*5 + 3*6 = 32

        // Test cross product
        let cross = v1.cross(&v2);
        assert!((cross.x - (-3.0)).abs() < 1e-15); // 2*6 - 3*5 = -3
        assert!((cross.y - 6.0).abs() < 1e-15); // 3*4 - 1*6 = 6
        assert!((cross.z - (-3.0)).abs() < 1e-15); // 1*5 - 2*4 = -3

        // Test magnitude
        let mag = v1.magnitude();
        assert!((mag - (14.0_f64).sqrt()).abs() < 1e-15);

        // Test normalization
        let normalized = v1.normalize().unwrap();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_vector_validation() {
        let valid_vector = Vector3D::new(1.0, 2.0, 3.0);
        assert!(valid_vector.is_valid());

        let invalid_vector = Vector3D::new(f64::NAN, 2.0, 3.0);
        assert!(!invalid_vector.is_valid());

        let infinite_vector = Vector3D::new(f64::INFINITY, 2.0, 3.0);
        assert!(!infinite_vector.is_valid());
    }

    #[test]
    fn test_line_validation() {
        let valid_line =
            Line3D::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(1.0, 0.0, 0.0)).unwrap();
        assert!(valid_line.is_valid());

        let zero_direction =
            Line3D::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0));
        assert!(zero_direction.is_err());
        assert_eq!(
            zero_direction.unwrap_err(),
            "Direction vector cannot be zero"
        );
    }

    #[test]
    fn test_plane_validation() {
        let valid_plane =
            Plane3D::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 1.0)).unwrap();
        assert!(valid_plane.is_valid());

        let zero_normal = Plane3D::new(Vector3D::new(0.0, 0.0, 0.0), Vector3D::new(0.0, 0.0, 0.0));
        assert!(zero_normal.is_err());
        assert_eq!(zero_normal.unwrap_err(), "Normal vector cannot be zero");
    }

    #[test]
    fn test_plane_distance_to_point() {
        let plane = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(0.0, 0.0, 1.0),
        };

        let point = Vector3D::new(1.0, 1.0, 5.0);
        let distance = plane.distance_to_point(&point);
        assert!((distance - 5.0).abs() < 1e-15);

        let point_on_plane = Vector3D::new(1.0, 1.0, 0.0);
        let distance = plane.distance_to_point(&point_on_plane);
        assert!(distance.abs() < 1e-15);
    }

    #[test]
    fn test_complex_intersection() {
        // Two planes intersecting at an arbitrary angle
        let plane1 = Plane3D {
            point: Vector3D::new(1.0, 2.0, 3.0),
            normal: Vector3D::new(1.0, 1.0, 0.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(2.0, 1.0, 3.0),
            normal: Vector3D::new(1.0, -1.0, 0.0),
        };

        let input = PlanePlaneIntersectionInput { plane1, plane2 };
        let result = plane_plane_intersection_logic(input).unwrap();

        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersects);
        assert!(!result.are_parallel);
        assert!(!result.are_coincident);
        assert!(result.intersection_line.is_some());

        let line = result.intersection_line.unwrap();
        assert!(line.is_valid());

        // The intersection line should be along z-axis (normal1 × normal2 = (0,0,-2))
        assert!(line.direction.x.abs() < 1e-15);
        assert!(line.direction.y.abs() < 1e-15);
        assert!(line.direction.z.abs() > 1e-15);
    }

    #[test]
    fn test_zero_vector_normalization() {
        let zero_vector = Vector3D::new(0.0, 0.0, 0.0);
        let result = zero_vector.normalize();

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Cannot normalize zero vector");
    }

    #[test]
    fn test_parallel_plane_detection() {
        let plane1 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 2.0, 3.0),
        };
        let plane2 = Plane3D {
            point: Vector3D::new(1.0, 1.0, 1.0),
            normal: Vector3D::new(2.0, 4.0, 6.0), // Parallel normal (2x)
        };

        assert!(plane1.is_parallel_to(&plane2));

        let plane3 = Plane3D {
            point: Vector3D::new(0.0, 0.0, 0.0),
            normal: Vector3D::new(1.0, 0.0, 0.0),
        };

        assert!(!plane1.is_parallel_to(&plane3));
    }
}
