use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
pub struct Line3D {
    pub point: Vector3D,     // A point on the line
    pub direction: Vector3D, // Direction vector of the line
}

#[derive(Deserialize, JsonSchema)]
pub struct MultipleLinesInput {
    pub lines: Vec<Line3D>,
}

#[derive(Serialize, JsonSchema)]
pub struct MultipleLineIntersectionResult {
    pub best_intersection_point: Vector3D,
    pub total_squared_distance: f64,
    pub individual_distances: Vec<f64>,
    pub lines_processed: usize,
}

const EPSILON: f64 = 1e-10;

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn is_zero(&self) -> bool {
        self.magnitude() < EPSILON
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
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }
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

fn find_multiple_line_intersection(lines: Vec<Line3D>) -> Result<MultipleLineIntersectionResult, String> {
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

#[tool]
pub fn multiple_line_intersection(input: MultipleLinesInput) -> Result<MultipleLineIntersectionResult, String> {
    find_multiple_line_intersection(input.lines)
}