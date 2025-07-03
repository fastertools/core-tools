use serde::{Deserialize, Serialize};
use super::vector_ops::Vector3D;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Line3D {
    pub point: Vector3D,     // A point on the line
    pub direction: Vector3D, // Direction vector of the line
}

#[derive(Deserialize)]
pub struct TwoLineInput {
    pub line1: Line3D,
    pub line2: Line3D,
}

#[derive(Deserialize)]
pub struct LineSegmentInput {
    pub segment1_start: Vector3D,
    pub segment1_end: Vector3D,
    pub segment2_start: Vector3D,
    pub segment2_end: Vector3D,
}

#[derive(Serialize)]
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

#[derive(Serialize)]
pub struct LineSegmentIntersectionResult {
    pub intersects: bool,
    pub intersection_point: Option<Vector3D>,
    pub closest_point_seg1: Vector3D,
    pub closest_point_seg2: Vector3D,
    pub minimum_distance: f64,
    pub intersection_on_both_segments: bool,
}

#[derive(Serialize)]
pub struct MultipleLineIntersectionResult {
    pub best_intersection_point: Vector3D,
    pub total_squared_distance: f64,
    pub individual_distances: Vec<f64>,
    pub lines_processed: usize,
}

const EPSILON: f64 = 1e-10;

impl Line3D {
    pub fn new(point: Vector3D, direction: Vector3D) -> Result<Self, String> {
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }

    pub fn point_at_parameter(&self, t: f64) -> Vector3D {
        self.point.add(&self.direction.scale(t))
    }

    pub fn is_parallel_to(&self, other: &Line3D) -> bool {
        self.direction.are_parallel(&other.direction)
    }
}

pub fn detect_line_intersection(input: TwoLineInput) -> Result<LineIntersectionResult, String> {
    let line1 = &input.line1;
    let line2 = &input.line2;

    // Validate direction vectors
    if line1.direction.is_zero() {
        return Err("Line1 direction vector cannot be zero".to_string());
    }
    if line2.direction.is_zero() {
        return Err("Line2 direction vector cannot be zero".to_string());
    }

    let are_parallel = line1.is_parallel_to(line2);
    
    if are_parallel {
        // Check if lines are coincident (same line)
        let point_diff = line2.point.subtract(&line1.point);
        let are_coincident = point_diff.are_parallel(&line1.direction) || point_diff.is_zero();
        
        if are_coincident {
            return Ok(LineIntersectionResult {
                intersection_type: "coincident".to_string(),
                intersects: true,
                intersection_point: Some(line1.point.clone()),
                closest_point_line1: line1.point.clone(),
                closest_point_line2: line2.point.clone(),
                minimum_distance: 0.0,
                parameter_line1: 0.0,
                parameter_line2: 0.0,
                are_parallel: true,
                are_skew: false,
                are_coincident: true,
            });
        } else {
            // Parallel but not coincident - find closest points
            let (t1, _t2, dist) = closest_points_parallel_lines(line1, line2);
            let closest1 = line1.point_at_parameter(t1);
            
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
    let (t1, t2, closest1, closest2, distance) = closest_points_skew_lines(line1, line2);
    
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

fn closest_points_skew_lines(line1: &Line3D, line2: &Line3D) -> (f64, f64, Vector3D, Vector3D, f64) {
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

pub fn detect_line_segment_intersection(input: LineSegmentInput) -> Result<LineSegmentIntersectionResult, String> {
    // Convert segments to lines
    let dir1 = input.segment1_end.subtract(&input.segment1_start);
    let dir2 = input.segment2_end.subtract(&input.segment2_start);
    
    if dir1.is_zero() {
        return Err("Segment 1 has zero length".to_string());
    }
    if dir2.is_zero() {
        return Err("Segment 2 has zero length".to_string());
    }
    
    let line1 = Line3D::new(input.segment1_start.clone(), dir1)?;
    let line2 = Line3D::new(input.segment2_start.clone(), dir2)?;
    
    let (t1, t2, closest1, closest2, distance) = closest_points_skew_lines(&line1, &line2);
    
    // Check if parameters are within segment bounds [0, 1]
    let t1_in_bounds = t1 >= 0.0 && t1 <= 1.0;
    let t2_in_bounds = t2 >= 0.0 && t2 <= 1.0;
    let intersection_on_both_segments = t1_in_bounds && t2_in_bounds;
    
    // Clamp parameters to segment bounds
    let t1_clamped = t1.max(0.0).min(1.0);
    let t2_clamped = t2.max(0.0).min(1.0);
    
    let final_closest1 = line1.point_at_parameter(t1_clamped);
    let final_closest2 = line2.point_at_parameter(t2_clamped);
    let final_distance = final_closest1.distance_to(&final_closest2);
    
    let intersects = intersection_on_both_segments && distance < EPSILON;
    let intersection_point = if intersects {
        Some(final_closest1.clone())
    } else {
        None
    };
    
    Ok(LineSegmentIntersectionResult {
        intersects,
        intersection_point,
        closest_point_seg1: final_closest1,
        closest_point_seg2: final_closest2,
        minimum_distance: final_distance,
        intersection_on_both_segments,
    })
}

pub fn find_multiple_line_intersection(lines: Vec<Line3D>) -> Result<MultipleLineIntersectionResult, String> {
    if lines.len() < 2 {
        return Err("At least 2 lines required".to_string());
    }
    
    // Validate all lines
    for (i, line) in lines.iter().enumerate() {
        if line.direction.is_zero() {
            return Err(format!("Line {} has zero direction vector", i));
        }
    }
    
    // Find the point that minimizes sum of squared distances to all lines
    // This is solved using least squares: (A^T A)x = A^T b
    let mut ata = [[0.0; 3]; 3]; // A^T A matrix
    let mut atb = [0.0; 3];      // A^T b vector
    
    for line in &lines {
        let d = &line.direction;
        let p = &line.point;
        
        // For each line: (I - dd^T/|d|^2) * (x - p) = 0
        // Rearranged: (I - dd^T/|d|^2) * x = (I - dd^T/|d|^2) * p
        let d_mag_sq = d.magnitude_squared();
        
        // Create projection matrix: I - dd^T/|d|^2
        let proj = [
            [1.0 - d.x * d.x / d_mag_sq, -d.x * d.y / d_mag_sq, -d.x * d.z / d_mag_sq],
            [-d.y * d.x / d_mag_sq, 1.0 - d.y * d.y / d_mag_sq, -d.y * d.z / d_mag_sq],
            [-d.z * d.x / d_mag_sq, -d.z * d.y / d_mag_sq, 1.0 - d.z * d.z / d_mag_sq],
        ];
        
        // Add to A^T A
        for i in 0..3 {
            for j in 0..3 {
                ata[i][j] += proj[i][j];
            }
        }
        
        // Add to A^T b
        let proj_p = [
            proj[0][0] * p.x + proj[0][1] * p.y + proj[0][2] * p.z,
            proj[1][0] * p.x + proj[1][1] * p.y + proj[1][2] * p.z,
            proj[2][0] * p.x + proj[2][1] * p.y + proj[2][2] * p.z,
        ];
        
        atb[0] += proj_p[0];
        atb[1] += proj_p[1];
        atb[2] += proj_p[2];
    }
    
    // Solve 3x3 system using Cramer's rule
    let det = determinant_3x3(&ata);
    if det.abs() < EPSILON {
        return Err("System is singular - lines may be parallel or coplanar".to_string());
    }
    
    let x = solve_3x3_system(&ata, &atb)?;
    let best_point = Vector3D::new(x[0], x[1], x[2]);
    
    // Calculate individual distances and total squared distance
    let mut individual_distances = Vec::new();
    let mut total_squared_distance = 0.0;
    
    for line in &lines {
        let distance = point_to_line_distance(&best_point, line);
        individual_distances.push(distance);
        total_squared_distance += distance * distance;
    }
    
    Ok(MultipleLineIntersectionResult {
        best_intersection_point: best_point,
        total_squared_distance,
        individual_distances,
        lines_processed: lines.len(),
    })
}

fn determinant_3x3(matrix: &[[f64; 3]; 3]) -> f64 {
    let m = matrix;
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

fn solve_3x3_system(a: &[[f64; 3]; 3], b: &[f64; 3]) -> Result<[f64; 3], String> {
    let det_a = determinant_3x3(a);
    if det_a.abs() < EPSILON {
        return Err("Matrix is singular".to_string());
    }
    
    // Cramer's rule
    let mut x = [0.0; 3];
    
    for i in 0..3 {
        let mut a_i = *a;
        a_i[0][i] = b[0];
        a_i[1][i] = b[1];
        a_i[2][i] = b[2];
        
        x[i] = determinant_3x3(&a_i) / det_a;
    }
    
    Ok(x)
}

fn point_to_line_distance(point: &Vector3D, line: &Line3D) -> f64 {
    let w = point.subtract(&line.point);
    let cross = w.cross(&line.direction);
    cross.magnitude() / line.direction.magnitude()
}