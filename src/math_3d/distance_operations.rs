use serde::{Deserialize, Serialize};
use super::vector_ops::Vector3D;
use super::line_intersection::Line3D;
use super::plane_operations::Plane3D;

#[derive(Deserialize)]
pub struct PointLineInput {
    pub point: Vector3D,
    pub line: Line3D,
}

#[derive(Deserialize)]
pub struct PointPlaneInput {
    pub point: Vector3D,
    pub plane: Plane3D,
}

#[derive(Deserialize)]
pub struct LinePlaneInput {
    pub line: Line3D,
    pub plane: Plane3D,
}

#[derive(Deserialize)]
pub struct VectorProjectionInput {
    pub vector: Vector3D,
    pub onto_vector: Vector3D,
}

#[derive(Deserialize)]
pub struct PointProjectionInput {
    pub point: Vector3D,
    pub onto_line: Line3D,
}

#[derive(Deserialize)]
pub struct PlaneProjectionInput {
    pub point: Vector3D,
    pub onto_plane: Plane3D,
}

#[derive(Serialize)]
pub struct PointLineDistanceResult {
    pub distance: f64,
    pub closest_point_on_line: Vector3D,
    pub parameter_on_line: f64,
    pub perpendicular_vector: Vector3D,
    pub point_is_on_line: bool,
}

#[derive(Serialize)]
pub struct PointPlaneDistanceResult {
    pub distance: f64,
    pub signed_distance: f64,
    pub closest_point_on_plane: Vector3D,
    pub point_is_on_plane: bool,
    pub side_of_plane: String, // "positive", "negative", or "on_plane"
}

#[derive(Serialize)]
pub struct LinePlaneDistanceResult {
    pub distance: f64,
    pub line_is_parallel: bool,
    pub line_intersects_plane: bool,
    pub intersection_point: Option<Vector3D>,
    pub closest_point_on_line: Vector3D,
    pub closest_point_on_plane: Vector3D,
}

#[derive(Serialize)]
pub struct VectorProjectionResult {
    pub scalar_projection: f64,
    pub vector_projection: Vector3D,
    pub rejection_vector: Vector3D,
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub vectors_are_parallel: bool,
    pub vectors_are_perpendicular: bool,
}

#[derive(Serialize)]
pub struct PointProjectionResult {
    pub projected_point: Vector3D,
    pub parameter_on_line: f64,
    pub distance_to_projection: f64,
    pub is_on_line: bool,
}

#[derive(Serialize)]
pub struct PlaneProjectionResult {
    pub projected_point: Vector3D,
    pub distance_to_projection: f64,
    pub is_on_plane: bool,
    pub projection_direction: Vector3D,
}

const EPSILON: f64 = 1e-10;

pub fn compute_point_line_distance(input: PointLineInput) -> Result<PointLineDistanceResult, String> {
    let point = &input.point;
    let line = &input.line;

    // Validate line direction
    if line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }

    // Vector from line point to query point
    let to_point = point.subtract(&line.point);
    
    // Project this vector onto the line direction to find the closest point
    let line_dir_mag_sq = line.direction.magnitude_squared();
    let t = to_point.dot(&line.direction) / line_dir_mag_sq;
    
    // Find closest point on line
    let closest_point_on_line = line.point_at_parameter(t);
    
    // Calculate distance and perpendicular vector
    let perpendicular_vector = point.subtract(&closest_point_on_line);
    let distance = perpendicular_vector.magnitude();
    
    // Check if point is on line
    let point_is_on_line = distance < EPSILON;

    Ok(PointLineDistanceResult {
        distance,
        closest_point_on_line,
        parameter_on_line: t,
        perpendicular_vector,
        point_is_on_line,
    })
}

pub fn compute_point_plane_distance(input: PointPlaneInput) -> Result<PointPlaneDistanceResult, String> {
    let point = &input.point;
    let plane = &input.plane;

    // Validate plane normal
    if plane.normal.is_zero() {
        return Err("Plane normal vector cannot be zero".to_string());
    }

    // Calculate distances
    let distance = plane.distance_to_point(point);
    let signed_distance = plane.signed_distance_to_point(point);
    let closest_point_on_plane = plane.project_point(point);
    let point_is_on_plane = distance < EPSILON;

    let side_of_plane = if point_is_on_plane {
        "on_plane".to_string()
    } else if signed_distance > 0.0 {
        "positive".to_string()
    } else {
        "negative".to_string()
    };

    Ok(PointPlaneDistanceResult {
        distance,
        signed_distance,
        closest_point_on_plane,
        point_is_on_plane,
        side_of_plane,
    })
}

pub fn compute_line_plane_distance(input: LinePlaneInput) -> Result<LinePlaneDistanceResult, String> {
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
        // Line is parallel to plane - distance is from line point to plane
        let distance = plane.distance_to_point(&line.point);
        let closest_point_on_plane = plane.project_point(&line.point);
        
        Ok(LinePlaneDistanceResult {
            distance,
            line_is_parallel: true,
            line_intersects_plane: distance < EPSILON,
            intersection_point: if distance < EPSILON { Some(line.point.clone()) } else { None },
            closest_point_on_line: line.point.clone(),
            closest_point_on_plane,
        })
    } else {
        // Line intersects plane - distance is 0
        let to_plane_point = plane.point.subtract(&line.point);
        let t = to_plane_point.dot(&normal_unit) / direction_dot_normal;
        let intersection_point = line.point_at_parameter(t);

        Ok(LinePlaneDistanceResult {
            distance: 0.0,
            line_is_parallel: false,
            line_intersects_plane: true,
            intersection_point: Some(intersection_point.clone()),
            closest_point_on_line: intersection_point.clone(),
            closest_point_on_plane: intersection_point,
        })
    }
}

pub fn compute_vector_projection(input: VectorProjectionInput) -> Result<VectorProjectionResult, String> {
    let vector = &input.vector;
    let onto_vector = &input.onto_vector;

    // Validate onto_vector
    if onto_vector.is_zero() {
        return Err("Cannot project onto zero vector".to_string());
    }

    // Calculate scalar projection: v · u / |u|
    let scalar_projection = vector.dot(onto_vector) / onto_vector.magnitude();
    
    // Calculate vector projection: (v · u / |u|²) * u
    let onto_mag_sq = onto_vector.magnitude_squared();
    let projection_scale = vector.dot(onto_vector) / onto_mag_sq;
    let vector_projection = onto_vector.scale(projection_scale);
    
    // Calculate rejection (perpendicular component): v - proj
    let rejection_vector = vector.subtract(&vector_projection);
    
    // Calculate angle between vectors
    let angle_radians = if vector.is_zero() {
        0.0
    } else {
        vector.angle_with(onto_vector).unwrap_or(0.0)
    };
    let angle_degrees = angle_radians.to_degrees();
    
    // Check relationships
    let vectors_are_parallel = vector.are_parallel(onto_vector);
    let vectors_are_perpendicular = vector.are_perpendicular(onto_vector);

    Ok(VectorProjectionResult {
        scalar_projection,
        vector_projection,
        rejection_vector,
        angle_radians,
        angle_degrees,
        vectors_are_parallel,
        vectors_are_perpendicular,
    })
}

pub fn compute_point_projection_on_line(input: PointProjectionInput) -> Result<PointProjectionResult, String> {
    let point = &input.point;
    let line = &input.onto_line;

    // Validate line direction
    if line.direction.is_zero() {
        return Err("Line direction vector cannot be zero".to_string());
    }

    // Vector from line point to query point
    let to_point = point.subtract(&line.point);
    
    // Project this vector onto the line direction
    let line_dir_mag_sq = line.direction.magnitude_squared();
    let t = to_point.dot(&line.direction) / line_dir_mag_sq;
    
    // Find projected point on line
    let projected_point = line.point_at_parameter(t);
    
    // Calculate distance from original point to projection
    let distance_to_projection = point.distance_to(&projected_point);
    
    // Check if point is on line
    let is_on_line = distance_to_projection < EPSILON;

    Ok(PointProjectionResult {
        projected_point,
        parameter_on_line: t,
        distance_to_projection,
        is_on_line,
    })
}

pub fn compute_point_projection_on_plane(input: PlaneProjectionInput) -> Result<PlaneProjectionResult, String> {
    let point = &input.point;
    let plane = &input.onto_plane;

    // Validate plane normal
    if plane.normal.is_zero() {
        return Err("Plane normal vector cannot be zero".to_string());
    }

    // Project point onto plane
    let projected_point = plane.project_point(point);
    
    // Calculate distance from original point to projection
    let distance_to_projection = point.distance_to(&projected_point);
    
    // Check if point is on plane
    let is_on_plane = distance_to_projection < EPSILON;
    
    // Calculate projection direction (from point to projected point)
    let projection_direction = if is_on_plane {
        Vector3D::new(0.0, 0.0, 0.0)
    } else {
        projected_point.subtract(point)
    };

    Ok(PlaneProjectionResult {
        projected_point,
        distance_to_projection,
        is_on_plane,
        projection_direction,
    })
}