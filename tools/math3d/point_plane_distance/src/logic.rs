use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Plane3D {
    /// A point on the plane
    pub point: Vector3D,
    /// Normal vector to the plane
    pub normal: Vector3D,
}

#[derive(Deserialize, Clone, Debug)]
pub struct PointPlaneInput {
    /// The point to measure distance from
    pub point: Vector3D,
    /// The plane to measure distance to
    pub plane: Plane3D,
}

#[derive(Serialize, Clone, Debug)]
pub struct PointPlaneResult {
    /// Absolute distance from point to plane
    pub distance: f64,
    /// Signed distance (positive if point is on the side of normal, negative otherwise)
    pub signed_distance: f64,
    /// Closest point on the plane to the given point
    pub closest_point_on_plane: Vector3D,
    /// Whether the point lies exactly on the plane
    pub is_on_plane: bool,
    /// Which side of the plane the point is on
    pub side_of_plane: String,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
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

    pub fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Plane3D {
    pub fn distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };
        
        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit).abs()
    }

    pub fn signed_distance_to_point(&self, point: &Vector3D) -> f64 {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return 0.0,
        };
        
        let to_point = point.subtract(&self.point);
        to_point.dot(&normal_unit)
    }

    pub fn project_point(&self, point: &Vector3D) -> Vector3D {
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return point.clone(),
        };
        
        let signed_dist = self.signed_distance_to_point(point);
        point.subtract(&normal_unit.scale(signed_dist))
    }

    pub fn is_valid(&self) -> bool {
        self.point.is_valid() && self.normal.is_valid() && !self.normal.is_zero()
    }
}

pub fn point_plane_distance_logic(input: PointPlaneInput) -> Result<PointPlaneResult, String> {
    // Input validation
    if !input.point.is_valid() {
        return Err("Invalid point coordinates: must be finite numbers".to_string());
    }

    if !input.plane.is_valid() {
        return Err("Invalid plane: normal vector cannot be zero and coordinates must be finite".to_string());
    }

    let distance = input.plane.distance_to_point(&input.point);
    let signed_distance = input.plane.signed_distance_to_point(&input.point);
    let closest_point_on_plane = input.plane.project_point(&input.point);
    let is_on_plane = distance < EPSILON;
    
    let side_of_plane = if is_on_plane {
        "on_plane".to_string()
    } else if signed_distance > 0.0 {
        "positive".to_string()
    } else {
        "negative".to_string()
    };

    Ok(PointPlaneResult {
        distance,
        signed_distance,
        closest_point_on_plane,
        is_on_plane,
        side_of_plane,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_vector(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    fn create_test_plane(point: Vector3D, normal: Vector3D) -> Plane3D {
        Plane3D { point, normal }
    }

    #[test]
    fn test_point_on_plane() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(1.0, 1.0, 0.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!(result.distance < 1e-10);
        assert!(result.is_on_plane);
        assert_eq!(result.side_of_plane, "on_plane");
    }

    #[test]
    fn test_point_above_plane() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(0.0, 0.0, 5.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!((result.distance - 5.0).abs() < 1e-10);
        assert_eq!(result.signed_distance, 5.0);
        assert!(!result.is_on_plane);
        assert_eq!(result.side_of_plane, "positive");
    }

    #[test]
    fn test_point_below_plane() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(0.0, 0.0, -3.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!((result.distance - 3.0).abs() < 1e-10);
        assert_eq!(result.signed_distance, -3.0);
        assert!(!result.is_on_plane);
        assert_eq!(result.side_of_plane, "negative");
    }

    #[test]
    fn test_oblique_plane() {
        // Plane through origin with normal (1,1,1)
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(1.0, 1.0, 1.0),
        );
        let point = create_test_vector(1.0, 1.0, 1.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        let expected_distance = 3.0_f64.sqrt(); // ||(1,1,1)|| = sqrt(3)
        assert!((result.distance - expected_distance).abs() < 1e-10);
        assert!(result.signed_distance > 0.0);
        assert_eq!(result.side_of_plane, "positive");
    }

    #[test]
    fn test_closest_point_projection() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(2.0, 3.0, 5.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        let closest = &result.closest_point_on_plane;
        assert!((closest.x - 2.0).abs() < 1e-10);
        assert!((closest.y - 3.0).abs() < 1e-10);
        assert!(closest.z.abs() < 1e-10);
    }

    #[test]
    fn test_zero_normal_vector() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 0.0), // Zero normal
        );
        let point = create_test_vector(1.0, 1.0, 1.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("normal vector cannot be zero"));
    }

    #[test]
    fn test_infinite_coordinates() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(f64::INFINITY, 1.0, 1.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_nan_coordinates() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(f64::NAN, 1.0, 1.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("must be finite numbers"));
    }

    #[test]
    fn test_plane_with_offset() {
        // Plane at z=2 with normal pointing up
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 2.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(0.0, 0.0, 5.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!((result.distance - 3.0).abs() < 1e-10);
        assert_eq!(result.signed_distance, 3.0);
    }

    #[test]
    fn test_very_small_distance() {
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 1.0),
        );
        let point = create_test_vector(0.0, 0.0, 1e-12);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!(result.is_on_plane); // Should be considered on plane due to EPSILON
        assert_eq!(result.side_of_plane, "on_plane");
    }

    #[test]
    fn test_plane_with_non_unit_normal() {
        // Normal vector that's not unit length
        let plane = create_test_plane(
            create_test_vector(0.0, 0.0, 0.0),
            create_test_vector(0.0, 0.0, 2.0), // Length 2
        );
        let point = create_test_vector(0.0, 0.0, 1.0);
        let input = PointPlaneInput { point, plane };

        let result = point_plane_distance_logic(input).unwrap();
        assert!((result.distance - 1.0).abs() < 1e-10); // Should still be 1.0 after normalization
    }

    #[test]
    fn test_vector_operations() {
        let v1 = create_test_vector(1.0, 2.0, 3.0);
        let v2 = create_test_vector(4.0, 5.0, 6.0);

        // Test dot product
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 32

        // Test magnitude
        let mag = v1.magnitude();
        assert!((mag - (14.0_f64.sqrt())).abs() < 1e-10);

        // Test normalization
        let normalized = v1.normalize().unwrap();
        assert!((normalized.magnitude() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_validation() {
        let valid_vector = create_test_vector(1.0, 2.0, 3.0);
        assert!(valid_vector.is_valid());

        let invalid_vector = create_test_vector(f64::NAN, 2.0, 3.0);
        assert!(!invalid_vector.is_valid());

        let zero_vector = create_test_vector(0.0, 0.0, 0.0);
        assert!(zero_vector.is_zero());
    }
}