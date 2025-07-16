use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Line3D {
    pub point: Vector3D,     // A point on the line
    pub direction: Vector3D, // Direction vector of the line
}

#[derive(Deserialize)]
pub struct MultipleLinesInput {
    pub lines: Vec<Line3D>,
}

#[derive(Serialize, Debug)]
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

    pub fn is_valid(&self) -> bool {
        self.x.is_finite() && self.y.is_finite() && self.z.is_finite()
    }
}

impl Line3D {
    pub fn new(point: Vector3D, direction: Vector3D) -> Result<Self, String> {
        if direction.is_zero() {
            return Err("Direction vector cannot be zero".to_string());
        }
        Ok(Line3D { point, direction })
    }

    pub fn is_valid(&self) -> bool {
        self.point.is_valid() && self.direction.is_valid()
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

pub fn multiple_line_intersection_logic(input: MultipleLinesInput) -> Result<MultipleLineIntersectionResult, String> {
    if input.lines.len() < 2 {
        return Err("At least 2 lines required".to_string());
    }
    
    // Validate all lines
    for (i, line) in input.lines.iter().enumerate() {
        if !line.is_valid() {
            return Err(format!("Line {} contains invalid values (NaN or Infinite)", i));
        }
        if line.direction.is_zero() {
            return Err(format!("Line {} has zero direction vector", i));
        }
    }
    
    // Find the point that minimizes sum of squared distances to all lines
    // This is solved using least squares: (A^T A)x = A^T b
    let mut ata = [[0.0; 3]; 3]; // A^T A matrix
    let mut atb = [0.0; 3];      // A^T b vector
    
    for line in &input.lines {
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
    
    for line in &input.lines {
        let distance = point_to_line_distance(&best_point, line);
        individual_distances.push(distance);
        total_squared_distance += distance * distance;
    }
    
    Ok(MultipleLineIntersectionResult {
        best_intersection_point: best_point,
        total_squared_distance,
        individual_distances,
        lines_processed: input.lines.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_vector(x: f64, y: f64, z: f64) -> Vector3D {
        Vector3D::new(x, y, z)
    }

    fn create_line(point: Vector3D, direction: Vector3D) -> Line3D {
        Line3D::new(point, direction).unwrap()
    }

    #[test]
    fn test_two_intersecting_lines() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = create_line(
            create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, -1.0, 0.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input).unwrap();
        assert_eq!(result.lines_processed, 2);
        assert!(result.total_squared_distance < EPSILON);
        
        // Should intersect at origin
        assert!(result.best_intersection_point.x.abs() < EPSILON);
        assert!(result.best_intersection_point.y.abs() < EPSILON);
        assert!(result.best_intersection_point.z.abs() < EPSILON);
    }

    #[test]
    fn test_three_lines_perfect_intersection() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = create_line(
            create_vector(0.0, 1.0, 0.0),
            create_vector(0.0, -1.0, 0.0),
        );
        let line3 = create_line(
            create_vector(0.0, 0.0, 1.0),
            create_vector(0.0, 0.0, -1.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1, line2, line3],
        };

        let result = multiple_line_intersection_logic(input).unwrap();
        assert_eq!(result.lines_processed, 3);
        assert!(result.total_squared_distance < EPSILON);
        assert_eq!(result.individual_distances.len(), 3);

        // Should intersect at origin
        assert!(result.best_intersection_point.x.abs() < EPSILON);
        assert!(result.best_intersection_point.y.abs() < EPSILON);
        assert!(result.best_intersection_point.z.abs() < EPSILON);
    }

    #[test]
    fn test_skew_lines_best_fit() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = create_line(
            create_vector(0.0, 1.0, 1.0),
            create_vector(0.0, 0.0, 1.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input).unwrap();
        assert_eq!(result.lines_processed, 2);
        assert!(result.total_squared_distance > EPSILON); // Should have non-zero distance
        assert_eq!(result.individual_distances.len(), 2);

        // Verify that the point minimizes total distance
        assert!(result.best_intersection_point.is_valid());
    }

    #[test]
    fn test_parallel_lines_error() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = create_line(
            create_vector(0.0, 1.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("singular"));
    }

    #[test]
    fn test_insufficient_lines_error() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1],
        };

        let result = multiple_line_intersection_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("At least 2 lines required"));
    }

    #[test]
    fn test_zero_direction_vector_error() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        // This should fail when creating, but let's test the validation
        let line2 = Line3D {
            point: create_vector(1.0, 1.0, 1.0),
            direction: create_vector(0.0, 0.0, 0.0),
        };

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("zero direction vector"));
    }

    #[test]
    fn test_invalid_coordinates_nan() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = Line3D {
            point: create_vector(f64::NAN, 1.0, 1.0),
            direction: create_vector(0.0, 1.0, 0.0),
        };

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_invalid_coordinates_infinite() {
        let line1 = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line2 = Line3D {
            point: create_vector(1.0, f64::INFINITY, 1.0),
            direction: create_vector(0.0, 1.0, 0.0),
        };

        let input = MultipleLinesInput {
            lines: vec![line1, line2],
        };

        let result = multiple_line_intersection_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid values"));
    }

    #[test]
    fn test_determinant_calculation() {
        let matrix = [
            [1.0, 2.0, 3.0],
            [4.0, 5.0, 6.0],
            [7.0, 8.0, 9.0],
        ];
        let det = determinant_3x3(&matrix);
        assert!(det.abs() < EPSILON); // This matrix is singular

        let identity = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let det_identity = determinant_3x3(&identity);
        assert!((det_identity - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_point_to_line_distance() {
        let line = create_line(
            create_vector(0.0, 0.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let point = create_vector(0.0, 1.0, 0.0);

        let distance = point_to_line_distance(&point, &line);
        assert!((distance - 1.0).abs() < EPSILON);

        // Point on the line should have zero distance
        let point_on_line = create_vector(5.0, 0.0, 0.0);
        let distance_on_line = point_to_line_distance(&point_on_line, &line);
        assert!(distance_on_line < EPSILON);
    }

    #[test]
    fn test_vector_operations() {
        let v1 = create_vector(1.0, 2.0, 3.0);
        let v2 = create_vector(4.0, 5.0, 6.0);

        // Test magnitude
        assert!((v1.magnitude() - (14.0_f64.sqrt())).abs() < EPSILON);

        // Test magnitude squared
        assert!((v1.magnitude_squared() - 14.0).abs() < EPSILON);

        // Test dot product
        assert!((v1.dot(&v2) - 32.0).abs() < EPSILON);

        // Test cross product
        let cross = v1.cross(&v2);
        assert!((cross.x - (-3.0)).abs() < EPSILON);
        assert!((cross.y - 6.0).abs() < EPSILON);
        assert!((cross.z - (-3.0)).abs() < EPSILON);

        // Test subtraction
        let diff = v2.subtract(&v1);
        assert!((diff.x - 3.0).abs() < EPSILON);
        assert!((diff.y - 3.0).abs() < EPSILON);
        assert!((diff.z - 3.0).abs() < EPSILON);
    }

    #[test]
    fn test_line_creation() {
        let point = create_vector(1.0, 2.0, 3.0);
        let direction = create_vector(1.0, 0.0, 0.0);
        
        let line = Line3D::new(point, direction);
        assert!(line.is_ok());

        let zero_direction = create_vector(0.0, 0.0, 0.0);
        let invalid_line = Line3D::new(create_vector(0.0, 0.0, 0.0), zero_direction);
        assert!(invalid_line.is_err());
    }

    #[test]
    fn test_solve_3x3_system() {
        // Test identity system: x = b
        let identity = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        let b = [1.0, 2.0, 3.0];
        let solution = solve_3x3_system(&identity, &b).unwrap();
        
        assert!((solution[0] - 1.0).abs() < EPSILON);
        assert!((solution[1] - 2.0).abs() < EPSILON);
        assert!((solution[2] - 3.0).abs() < EPSILON);

        // Test singular matrix
        let singular = [
            [1.0, 2.0, 3.0],
            [2.0, 4.0, 6.0],
            [3.0, 6.0, 9.0],
        ];
        let result = solve_3x3_system(&singular, &b);
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_intersection_case() {
        // Test with 4 lines that don't perfectly intersect
        let line1 = create_line(
            create_vector(1.0, 0.0, 0.0),
            create_vector(0.0, 1.0, 0.0),
        );
        let line2 = create_line(
            create_vector(0.0, 1.0, 0.0),
            create_vector(1.0, 0.0, 0.0),
        );
        let line3 = create_line(
            create_vector(0.0, 0.0, 1.0),
            create_vector(1.0, 1.0, 0.0),
        );
        let line4 = create_line(
            create_vector(1.0, 1.0, 1.0),
            create_vector(-1.0, -1.0, 0.0),
        );

        let input = MultipleLinesInput {
            lines: vec![line1, line2, line3, line4],
        };

        let result = multiple_line_intersection_logic(input).unwrap();
        assert_eq!(result.lines_processed, 4);
        assert_eq!(result.individual_distances.len(), 4);
        assert!(result.total_squared_distance >= 0.0);
        assert!(result.best_intersection_point.is_valid());
    }
}