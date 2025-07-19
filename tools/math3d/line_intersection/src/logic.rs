use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Vector3D {
    /// X component of the vector
    pub x: f64,
    /// Y component of the vector
    pub y: f64,
    /// Z component of the vector
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Line3D {
    /// A point on the line
    pub point: Vector3D,
    /// Direction vector of the line
    pub direction: Vector3D,
}

#[derive(Deserialize)]
pub struct LineIntersectionInput {
    /// First 3D line
    pub line1: Line3D,
    /// Second 3D line
    pub line2: Line3D,
}

#[derive(Serialize, Debug)]
pub struct LineIntersectionResult {
    pub intersection_type: String,
    pub intersects: bool,
    pub intersection_point: Option<Vector3D>,
    pub closest_point_line1: Vector3D,
    pub closest_point_line2: Vector3D,
    pub minimum_distance: f64,
    pub parameter_line1: f64,
    pub parameter_line2: f64,
    pub are_parallel: bool,
    pub are_skew: bool,
    pub are_coincident: bool,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn distance_to(&self, other: &Vector3D) -> f64 {
        self.subtract(other).magnitude()
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
    }

    pub fn are_parallel(&self, other: &Vector3D) -> bool {
        let cross = Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        };
        cross.magnitude() < EPSILON
    }

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Line3D {
    pub fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
    }

    pub fn is_parallel_to(&self, other: &Line3D) -> bool {
        self.direction.are_parallel(&other.direction)
    }

    pub fn is_valid(&self) -> bool {
        self.point.is_valid() && self.direction.is_valid()
    }
}

fn closest_points_skew_lines(
    line1: &Line3D,
    line2: &Line3D,
) -> (f64, f64, Vector3D, Vector3D, f64) {
    let d1 = &line1.direction;
    let d2 = &line2.direction;
    let w = line1.point.subtract(&line2.point);

    let a = d1.dot(d1);
    let b = d1.dot(d2);
    let c = d2.dot(d2);
    let d = d1.dot(&w);
    let e = d2.dot(&w);

    let denominator = a * c - b * b;

    let (t1, t2) = if denominator.abs() < EPSILON {
        // Lines are parallel (shouldn't happen here, but safety check)
        (0.0, 0.0)
    } else {
        let t1 = (b * e - c * d) / denominator;
        let t2 = (a * e - b * d) / denominator;
        (t1, t2)
    };

    let closest1 = line1.point_at_parameter(t1);
    let closest2 = line2.point_at_parameter(t2);
    let distance = closest1.distance_to(&closest2);

    (t1, t2, closest1, closest2, distance)
}

fn closest_points_parallel_lines(line1: &Line3D, line2: &Line3D) -> (f64, f64, f64) {
    let w = line2.point.subtract(&line1.point);
    let d1 = &line1.direction;

    let t1 = d1.dot(&w) / d1.dot(d1);
    let closest1 = line1.point_at_parameter(t1);
    let distance = closest1.distance_to(&line2.point);

    (t1, 0.0, distance)
}

pub fn line_intersection_logic(
    input: LineIntersectionInput,
) -> Result<LineIntersectionResult, String> {
    // Input validation
    if !input.line1.is_valid() {
        return Err("Line1 contains invalid values (NaN or Infinite)".to_string());
    }
    if !input.line2.is_valid() {
        return Err("Line2 contains invalid values (NaN or Infinite)".to_string());
    }

    // Validate direction vectors
    if input.line1.direction.is_zero() {
        return Err("Line1 direction vector cannot be zero".to_string());
    }
    if input.line2.direction.is_zero() {
        return Err("Line2 direction vector cannot be zero".to_string());
    }

    let are_parallel = input.line1.is_parallel_to(&input.line2);

    if are_parallel {
        // Check if lines are coincident (same line)
        let point_diff = input.line2.point.subtract(&input.line1.point);
        let are_coincident =
            point_diff.are_parallel(&input.line1.direction) || point_diff.is_zero();

        if are_coincident {
            return Ok(LineIntersectionResult {
                intersection_type: "coincident".to_string(),
                intersects: true,
                intersection_point: Some(input.line1.point.clone()),
                closest_point_line1: input.line1.point.clone(),
                closest_point_line2: input.line2.point.clone(),
                minimum_distance: 0.0,
                parameter_line1: 0.0,
                parameter_line2: 0.0,
                are_parallel: true,
                are_skew: false,
                are_coincident: true,
            });
        } else {
            // Parallel but not coincident - find closest points
            let (t1, _t2, dist) = closest_points_parallel_lines(&input.line1, &input.line2);
            let closest1 = input.line1.point_at_parameter(t1);

            return Ok(LineIntersectionResult {
                intersection_type: "parallel".to_string(),
                intersects: false,
                intersection_point: None,
                closest_point_line1: closest1.clone(),
                closest_point_line2: closest1.clone(), // Project onto line2
                minimum_distance: dist,
                parameter_line1: t1,
                parameter_line2: 0.0,
                are_parallel: true,
                are_skew: false,
                are_coincident: false,
            });
        }
    }

    // Lines are not parallel - find closest points
    let (t1, t2, closest1, closest2, distance) =
        closest_points_skew_lines(&input.line1, &input.line2);

    let intersects = distance < EPSILON;
    let intersection_point = if intersects {
        Some(closest1.clone())
    } else {
        None
    };

    let intersection_type = if intersects {
        "intersecting".to_string()
    } else {
        "skew".to_string()
    };

    Ok(LineIntersectionResult {
        intersection_type,
        intersects,
        intersection_point,
        closest_point_line1: closest1,
        closest_point_line2: closest2,
        minimum_distance: distance,
        parameter_line1: t1,
        parameter_line2: t2,
        are_parallel: false,
        are_skew: !intersects,
        are_coincident: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_vector(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D { x, y, z }
    }

    fn create_line(point: Vector3D, direction: Vector3D) -> Line3D {
        Line3D { point, direction }
    }

    #[test]
    fn test_intersecting_lines() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(0.0, 1.0, 0.0), create_vector(0.0, -1.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersection_point.is_some());
        assert!(!result.are_parallel);
        assert!(!result.are_skew);
        assert!(!result.are_coincident);
        assert!(result.minimum_distance < EPSILON);
    }

    #[test]
    fn test_parallel_lines() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(0.0, 1.0, 0.0), create_vector(1.0, 0.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "parallel");
        assert!(result.intersection_point.is_none());
        assert!(result.are_parallel);
        assert!(!result.are_skew);
        assert!(!result.are_coincident);
        assert_eq!(result.minimum_distance, 1.0);
    }

    #[test]
    fn test_coincident_lines() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(1.0, 0.0, 0.0), create_vector(2.0, 0.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(result.intersects);
        assert_eq!(result.intersection_type, "coincident");
        assert!(result.intersection_point.is_some());
        assert!(result.are_parallel);
        assert!(!result.are_skew);
        assert!(result.are_coincident);
        assert!(result.minimum_distance < EPSILON);
    }

    #[test]
    fn test_skew_lines() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(0.0, 1.0, 1.0), create_vector(0.0, 0.0, 1.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "skew");
        assert!(result.intersection_point.is_none());
        assert!(!result.are_parallel);
        assert!(result.are_skew);
        assert!(!result.are_coincident);
        assert!(result.minimum_distance > EPSILON);
    }

    #[test]
    fn test_zero_direction_vector_error() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(0.0, 0.0, 0.0));
        let line2 = create_line(create_vector(1.0, 1.0, 1.0), create_vector(1.0, 0.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input);

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("direction vector cannot be zero")
        );
    }

    #[test]
    fn test_invalid_line_coordinates_nan() {
        let line1 = create_line(
            create_vector(f64::NAN, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = create_line(create_vector(0.0, 1.0, 0.0), create_vector(0.0, -1.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_invalid_line_coordinates_infinite() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(
            create_vector(0.0, f64::INFINITY, 0.0),
            create_vector(0.0, -1.0, 0.0),
        );

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_perpendicular_intersecting_lines() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(2.0, -1.0, 0.0), create_vector(0.0, 1.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(result.intersects);
        assert_eq!(result.intersection_type, "intersecting");
        assert!(result.intersection_point.is_some());

        let intersection = result.intersection_point.unwrap();
        assert!((intersection.x - 2.0).abs() < EPSILON);
        assert!(intersection.y.abs() < EPSILON);
        assert!(intersection.z.abs() < EPSILON);
    }

    #[test]
    fn test_closest_points_calculation() {
        let line1 = create_line(create_vector(0.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(0.5, 1.0, 1.0), create_vector(0.0, 0.0, 1.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(!result.intersects);
        assert_eq!(result.intersection_type, "skew");

        // Closest point on line1 should be (0.5, 0, 0)
        assert!((result.closest_point_line1.x - 0.5).abs() < EPSILON);
        assert!(result.closest_point_line1.y.abs() < EPSILON);
        assert!(result.closest_point_line1.z.abs() < EPSILON);

        // Closest point on line2 should be (0.5, 1, 0)
        assert!((result.closest_point_line2.x - 0.5).abs() < EPSILON);
        assert!((result.closest_point_line2.y - 1.0).abs() < EPSILON);
        assert!(result.closest_point_line2.z.abs() < EPSILON);

        assert!((result.minimum_distance - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_line_parameters() {
        let line1 = create_line(create_vector(1.0, 0.0, 0.0), create_vector(1.0, 0.0, 0.0));
        let line2 = create_line(create_vector(0.0, 1.0, 0.0), create_vector(0.0, -1.0, 0.0));

        let input = LineIntersectionInput { line1, line2 };
        let result = line_intersection_logic(input).unwrap();

        assert!(result.intersects);
        // line1 at t=1: (1,0,0) + 1*(1,0,0) = (2,0,0)
        // line2 at t=1: (0,1,0) + 1*(0,-1,0) = (0,0,0)
        // They should intersect, verify parameters make sense
        assert!(result.parameter_line1.is_finite());
        assert!(result.parameter_line2.is_finite());
    }

    #[test]
    fn test_vector_operations() {
        let v1 = create_vector(1.0, 2.0, 3.0);
        let v2 = create_vector(4.0, 5.0, 6.0);

        // Test dot product
        let dot = v1.dot(&v2);
        assert_eq!(dot, 32.0); // 1*4 + 2*5 + 3*6 = 32

        // Test magnitude
        let mag = v1.magnitude();
        assert!((mag - (14.0_f64.sqrt())).abs() < EPSILON);

        // Test addition
        let sum = v1.add(&v2);
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);

        // Test subtraction
        let diff = v2.subtract(&v1);
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);

        // Test scaling
        let scaled = v1.scale(2.0);
        assert_eq!(scaled.x, 2.0);
        assert_eq!(scaled.y, 4.0);
        assert_eq!(scaled.z, 6.0);
    }

    #[test]
    fn test_parallel_detection() {
        let v1 = create_vector(1.0, 0.0, 0.0);
        let v2 = create_vector(2.0, 0.0, 0.0);
        let v3 = create_vector(0.0, 1.0, 0.0);

        assert!(v1.are_parallel(&v2));
        assert!(!v1.are_parallel(&v3));
    }

    #[test]
    fn test_line_point_at_parameter() {
        let line = create_line(create_vector(1.0, 2.0, 3.0), create_vector(1.0, 0.0, 0.0));

        let point = line.point_at_parameter(5.0);
        assert_eq!(point.x, 6.0);
        assert_eq!(point.y, 2.0);
        assert_eq!(point.z, 3.0);
    }

    #[test]
    fn test_line_validation() {
        let valid_line = create_line(create_vector(1.0, 2.0, 3.0), create_vector(1.0, 0.0, 0.0));
        assert!(valid_line.is_valid());

        let invalid_line = create_line(
            create_vector(f64::NAN, 2.0, 3.0),
            create_vector(1.0, 0.0, 0.0),
        );
        assert!(!invalid_line.is_valid());
    }
}
