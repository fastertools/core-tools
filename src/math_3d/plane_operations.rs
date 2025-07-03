use serde::{Deserialize, Serialize};
use super::vector_ops::Vector3D;
use super::line_intersection::Line3D;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Plane3D {
    pub point: Vector3D,  // A point on the plane
    pub normal: Vector3D, // Normal vector to the plane
}

#[derive(Deserialize)]
pub struct LinePlaneInput {
    pub line: Line3D,
    pub plane: Plane3D,
}

#[derive(Deserialize)]
pub struct TwoPlaneInput {
    pub plane1: Plane3D,
    pub plane2: Plane3D,
}

#[derive(Deserialize)]
pub struct PointPlaneInput {
    pub point: Vector3D,
    pub plane: Plane3D,
}

#[derive(Serialize)]
pub struct LinePlaneIntersectionResult {
    pub intersection_type: String,
    pub intersects: bool,
    pub intersection_point: Option<Vector3D>,
    pub parameter: Option<f64>,
    pub line_is_parallel: bool,
    pub line_is_in_plane: bool,
    pub distance_to_plane: f64,
}

#[derive(Serialize)]
pub struct PlanePlaneIntersectionResult {
    pub intersection_type: String,
    pub intersects: bool,
    pub intersection_line: Option<Line3D>,
    pub are_parallel: bool,
    pub are_coincident: bool,
    pub angle_radians: f64,
    pub angle_degrees: f64,
}

#[derive(Serialize)]
pub struct PointPlaneResult {
    pub distance: f64,
    pub signed_distance: f64,
    pub closest_point_on_plane: Vector3D,
    pub is_on_plane: bool,
    pub side_of_plane: String, // "positive", "negative", or "on_plane"
}

const EPSILON: f64 = 1e-10;

impl Plane3D {
    pub fn new(point: Vector3D, normal: Vector3D) -> Result<Self, String> {
        if normal.is_zero() {
            return Err("Normal vector cannot be zero".to_string());
        }
        Ok(Plane3D { point, normal })
    }

    pub fn from_three_points(p1: Vector3D, p2: Vector3D, p3: Vector3D) -> Result<Self, String> {
        let v1 = p2.subtract(&p1);
        let v2 = p3.subtract(&p1);
        let normal = v1.cross(&v2);
        
        if normal.is_zero() {
            return Err("Points are collinear - cannot define a plane".to_string());
        }
        
        Ok(Plane3D {
            point: p1,
            normal,
        })
    }

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
        let signed_dist = self.signed_distance_to_point(point);
        let normal_unit = match self.normal.normalize() {
            Ok(n) => n,
            Err(_) => return point.clone(),
        };
        
        point.subtract(&normal_unit.scale(signed_dist))
    }

    pub fn is_parallel_to(&self, other: &Plane3D) -> bool {
        self.normal.are_parallel(&other.normal)
    }

    pub fn angle_with(&self, other: &Plane3D) -> Result<f64, String> {
        self.normal.angle_with(&other.normal)
    }
}

pub fn compute_line_plane_intersection(input: LinePlaneInput) -> Result<LinePlaneIntersectionResult, String> {
    let line = &input.line;
    let plane = &input.plane;

    // Validate inputs
    if line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }
    if plane.normal.is_zero() {
        return Err("Plane normal vector cannot be zero".to_string());
    }

    let normal_unit = plane.normal.normalize()?;
    let direction_dot_normal = line.direction.dot(&normal_unit);
    let line_is_parallel = direction_dot_normal.abs() < EPSILON;

    if line_is_parallel {
        // Line is parallel to plane - check if it's in the plane
        let distance_to_plane = plane.distance_to_point(&line.point);
        let line_is_in_plane = distance_to_plane < EPSILON;

        return Ok(LinePlaneIntersectionResult {
            intersection_type: if line_is_in_plane { "in_plane".to_string() } else { "parallel".to_string() },
            intersects: line_is_in_plane,
            intersection_point: if line_is_in_plane { Some(line.point.clone()) } else { None },
            parameter: if line_is_in_plane { Some(0.0) } else { None },
            line_is_parallel: true,
            line_is_in_plane,
            distance_to_plane,
        });
    }

    // Line intersects plane at a single point
    let to_plane_point = plane.point.subtract(&line.point);
    let t = to_plane_point.dot(&normal_unit) / direction_dot_normal;
    let intersection_point = line.point_at_parameter(t);

    Ok(LinePlaneIntersectionResult {
        intersection_type: "intersecting".to_string(),
        intersects: true,
        intersection_point: Some(intersection_point),
        parameter: Some(t),
        line_is_parallel: false,
        line_is_in_plane: false,
        distance_to_plane: 0.0,
    })
}

pub fn compute_plane_plane_intersection(input: TwoPlaneInput) -> Result<PlanePlaneIntersectionResult, String> {
    let plane1 = &input.plane1;
    let plane2 = &input.plane2;

    // Validate inputs
    if plane1.normal.is_zero() {
        return Err("Plane1 normal vector cannot be zero".to_string());
    }
    if plane2.normal.is_zero() {
        return Err("Plane2 normal vector cannot be zero".to_string());
    }

    let are_parallel = plane1.is_parallel_to(plane2);
    let angle_radians = plane1.angle_with(plane2).unwrap_or(0.0);
    let angle_degrees = angle_radians.to_degrees();

    if are_parallel {
        // Check if planes are coincident
        let distance = plane1.distance_to_point(&plane2.point);
        let are_coincident = distance < EPSILON;

        return Ok(PlanePlaneIntersectionResult {
            intersection_type: if are_coincident { "coincident".to_string() } else { "parallel".to_string() },
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
            return Err("Cannot find intersection point".to_string());
        }
        let x = (d1 * n2.y - d2 * n1.y) / det;
        let y = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, y, 0.0)
    } else if abs_dir.y >= abs_dir.x {
        // Solve for x and z, set y = 0
        let det = n1.x * n2.z - n1.z * n2.x;
        if det.abs() < EPSILON {
            return Err("Cannot find intersection point".to_string());
        }
        let x = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.x - d1 * n2.x) / det;
        Vector3D::new(x, 0.0, z)
    } else {
        // Solve for y and z, set x = 0
        let det = n1.y * n2.z - n1.z * n2.y;
        if det.abs() < EPSILON {
            return Err("Cannot find intersection point".to_string());
        }
        let y = (d1 * n2.z - d2 * n1.z) / det;
        let z = (d2 * n1.y - d1 * n2.y) / det;
        Vector3D::new(0.0, y, z)
    };

    let intersection_line = Line3D::new(intersection_point, direction)?;

    Ok(PlanePlaneIntersectionResult {
        intersection_type: "intersecting".to_string(),
        intersects: true,
        intersection_line: Some(intersection_line),
        are_parallel: false,
        are_coincident: false,
        angle_radians,
        angle_degrees,
    })
}

pub fn compute_point_plane_distance(input: PointPlaneInput) -> Result<PointPlaneResult, String> {
    let point = &input.point;
    let plane = &input.plane;

    if plane.normal.is_zero() {
        return Err("Plane normal vector cannot be zero".to_string());
    }

    let distance = plane.distance_to_point(point);
    let signed_distance = plane.signed_distance_to_point(point);
    let closest_point_on_plane = plane.project_point(point);
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