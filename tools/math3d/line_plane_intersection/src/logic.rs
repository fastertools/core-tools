use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Line3D {
    pub point: Vector3D,
    pub direction: Vector3D,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Plane3D {
    pub point: Vector3D,
    pub normal: Vector3D,
}

#[derive(Deserialize, Serialize)]
pub struct LinePlaneInput {
    pub line: Line3D,
    pub plane: Plane3D,
}

#[derive(Serialize, Debug)]
pub struct LinePlaneIntersectionResult {
    pub intersection_type: String,
    pub intersects: bool,
    pub intersection_point: Option<Vector3D>,
    pub parameter: Option<f64>,
    pub line_is_parallel: bool,
    pub line_is_in_plane: bool,
    pub distance_to_plane: f64,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    #[allow(dead_code)]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

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

    #[allow(dead_code)]
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
}

pub fn line_plane_intersection_logic(
    input: LinePlaneInput,
) -> Result<LinePlaneIntersectionResult, String> {
    // Validate inputs for NaN and infinite values
    if input.line.point.x.is_nan()
        || input.line.point.x.is_infinite()
        || input.line.point.y.is_nan()
        || input.line.point.y.is_infinite()
        || input.line.point.z.is_nan()
        || input.line.point.z.is_infinite()
    {
        return Err("Line point coordinates must be finite".to_string());
    }

    if input.line.direction.x.is_nan()
        || input.line.direction.x.is_infinite()
        || input.line.direction.y.is_nan()
        || input.line.direction.y.is_infinite()
        || input.line.direction.z.is_nan()
        || input.line.direction.z.is_infinite()
    {
        return Err("Line direction coordinates must be finite".to_string());
    }

    if input.plane.point.x.is_nan()
        || input.plane.point.x.is_infinite()
        || input.plane.point.y.is_nan()
        || input.plane.point.y.is_infinite()
        || input.plane.point.z.is_nan()
        || input.plane.point.z.is_infinite()
    {
        return Err("Plane point coordinates must be finite".to_string());
    }

    if input.plane.normal.x.is_nan()
        || input.plane.normal.x.is_infinite()
        || input.plane.normal.y.is_nan()
        || input.plane.normal.y.is_infinite()
        || input.plane.normal.z.is_nan()
        || input.plane.normal.z.is_infinite()
    {
        return Err("Plane normal coordinates must be finite".to_string());
    }

    // Validate inputs
    if input.line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }

    if input.plane.normal.is_zero() {
        return Err("Plane normal vector cannot be zero".to_string());
    }

    // Calculate dot product of line direction and plane normal
    let dot_product = input.line.direction.dot(&input.plane.normal);

    // Check if line is parallel to plane (direction perpendicular to normal)
    let is_parallel = dot_product.abs() < EPSILON;

    if is_parallel {
        // Line is parallel to plane
        // Check if line is in the plane
        let point_to_plane = input.line.point.subtract(&input.plane.point);
        let distance = point_to_plane.dot(&input.plane.normal).abs();

        let normal_mag = input.plane.normal.magnitude();
        let normalized_distance = if normal_mag > EPSILON {
            distance / normal_mag
        } else {
            0.0
        };

        let is_in_plane = normalized_distance < EPSILON;

        Ok(LinePlaneIntersectionResult {
            intersection_type: if is_in_plane {
                "line_in_plane".to_string()
            } else {
                "no_intersection".to_string()
            },
            intersects: is_in_plane,
            intersection_point: None,
            parameter: None,
            line_is_parallel: true,
            line_is_in_plane: is_in_plane,
            distance_to_plane: if is_in_plane {
                0.0
            } else {
                normalized_distance
            },
        })
    } else {
        // Line is not parallel - calculate intersection point
        // Using parametric form: P = line.point + t * line.direction
        // Plane equation: (P - plane.point) · plane.normal = 0
        // Substituting: ((line.point + t * line.direction) - plane.point) · plane.normal = 0
        // Solving for t: t = (plane.point - line.point) · plane.normal / (line.direction · plane.normal)

        let point_diff = input.plane.point.subtract(&input.line.point);
        let t = point_diff.dot(&input.plane.normal) / dot_product;

        // Calculate intersection point
        let intersection_point = input.line.point.add(&input.line.direction.scale(t));

        Ok(LinePlaneIntersectionResult {
            intersection_type: "point".to_string(),
            intersects: true,
            intersection_point: Some(intersection_point),
            parameter: Some(t),
            line_is_parallel: false,
            line_is_in_plane: false,
            distance_to_plane: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_intersects_plane_at_point() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, -1.0),
                direction: Vector3D::new(0.0, 0.0, 1.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "point");
        assert!(!result.line_is_parallel);
        assert!(!result.line_is_in_plane);

        let intersection = result.intersection_point.unwrap();
        assert!((intersection.x - 0.0).abs() < EPSILON);
        assert!((intersection.y - 0.0).abs() < EPSILON);
        assert!((intersection.z - 0.0).abs() < EPSILON);

        let param = result.parameter.unwrap();
        assert!((param - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_line_parallel_to_plane_no_intersection() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 1.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "no_intersection");
        assert!(result.line_is_parallel);
        assert!(!result.line_is_in_plane);
        assert!(result.intersection_point.is_none());
        assert!(result.parameter.is_none());
        assert!(result.distance_to_plane > 0.0);
    }

    #[test]
    fn test_line_lies_in_plane() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(1.0, 2.0, 0.0),
                direction: Vector3D::new(1.0, 1.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "line_in_plane");
        assert!(result.line_is_parallel);
        assert!(result.line_is_in_plane);
        assert!(result.intersection_point.is_none());
        assert!(result.parameter.is_none());
        assert!((result.distance_to_plane - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_diagonal_line_intersection() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(1.0, 1.0, 1.0),
                direction: Vector3D::new(1.0, 1.0, 1.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "point");
        assert!(!result.line_is_parallel);
        assert!(!result.line_is_in_plane);

        let intersection = result.intersection_point.unwrap();
        assert!((intersection.z - 0.0).abs() < EPSILON);

        let param = result.parameter.unwrap();
        assert!((param - (-1.0)).abs() < EPSILON);
    }

    #[test]
    fn test_zero_line_direction_error() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(0.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line direction vector cannot be zero");
    }

    #[test]
    fn test_zero_plane_normal_error() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 0.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Plane normal vector cannot be zero");
    }

    #[test]
    fn test_nan_line_point() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(f64::NAN, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line point coordinates must be finite");
    }

    #[test]
    fn test_infinite_line_direction() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(f64::INFINITY, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Line direction coordinates must be finite"
        );
    }

    #[test]
    fn test_nan_plane_point() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(f64::NAN, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Plane point coordinates must be finite"
        );
    }

    #[test]
    fn test_infinite_plane_normal() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(f64::INFINITY, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Plane normal coordinates must be finite"
        );
    }

    #[test]
    fn test_line_intersects_tilted_plane() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1.0, 1.0, 1.0),
            },
            plane: Plane3D {
                point: Vector3D::new(1.0, 0.0, 0.0),
                normal: Vector3D::new(1.0, 0.0, 0.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "point");

        let intersection = result.intersection_point.unwrap();
        assert!((intersection.x - 1.0).abs() < EPSILON);
        assert!((intersection.y - 1.0).abs() < EPSILON);
        assert!((intersection.z - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_very_small_direction_vector() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                direction: Vector3D::new(1e-15, 0.0, 0.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Line direction vector cannot be zero");
    }

    #[test]
    fn test_nearly_parallel_line() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(0.0, 0.0, 1.0),
                direction: Vector3D::new(1.0, 0.0, 1e-15),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 0.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "no_intersection");
        assert!(result.line_is_parallel);
    }

    #[test]
    fn test_large_coordinates() {
        let input = LinePlaneInput {
            line: Line3D {
                point: Vector3D::new(1000.0, 2000.0, 3000.0),
                direction: Vector3D::new(0.0, 0.0, -1.0),
            },
            plane: Plane3D {
                point: Vector3D::new(0.0, 0.0, 2500.0),
                normal: Vector3D::new(0.0, 0.0, 1.0),
            },
        };

        let result = line_plane_intersection_logic(input).unwrap();
        assert!(result.intersects);
        assert_eq!(result.intersection_type, "point");

        let intersection = result.intersection_point.unwrap();
        assert!((intersection.x - 1000.0).abs() < EPSILON);
        assert!((intersection.y - 2000.0).abs() < EPSILON);
        assert!((intersection.z - 2500.0).abs() < EPSILON);
    }
}
