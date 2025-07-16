use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBoxInput {
    pub points: Vec<Vector3D>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBoxResponse {
    pub volume: f64,
    pub box_type: String,
    pub min_point: Vector3D,
    pub max_point: Vector3D,
    pub dimensions: Vector3D,
}

pub fn compute_aabb_volume(input: BoundingBoxInput) -> Result<BoundingBoxResponse, String> {
    // Validate input
    if input.points.is_empty() {
        return Err("At least one point is required".to_string());
    }
    
    // Check for NaN and infinite values
    for (i, point) in input.points.iter().enumerate() {
        if point.x.is_nan() || point.y.is_nan() || point.z.is_nan() {
            return Err(format!("Point {} contains NaN values", i));
        }
        if point.x.is_infinite() || point.y.is_infinite() || point.z.is_infinite() {
            return Err(format!("Point {} contains infinite values", i));
        }
    }
    
    let first_point = &input.points[0];
    let mut min_x = first_point.x;
    let mut max_x = first_point.x;
    let mut min_y = first_point.y;
    let mut max_y = first_point.y;
    let mut min_z = first_point.z;
    let mut max_z = first_point.z;
    
    // Find the minimum and maximum coordinates
    for point in &input.points {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
        min_z = min_z.min(point.z);
        max_z = max_z.max(point.z);
    }
    
    let dimensions = Vector3D {
        x: max_x - min_x,
        y: max_y - min_y,
        z: max_z - min_z,
    };
    
    let volume = dimensions.x * dimensions.y * dimensions.z;
    
    Ok(BoundingBoxResponse {
        volume,
        box_type: "AABB (Axis-Aligned Bounding Box)".to_string(),
        min_point: Vector3D { x: min_x, y: min_y, z: min_z },
        max_point: Vector3D { x: max_x, y: max_y, z: max_z },
        dimensions,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_point() {
        let input = BoundingBoxInput {
            points: vec![Vector3D { x: 1.0, y: 2.0, z: 3.0 }],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
        assert_eq!(result.min_point.x, 1.0);
        assert_eq!(result.min_point.y, 2.0);
        assert_eq!(result.min_point.z, 3.0);
        assert_eq!(result.max_point.x, 1.0);
        assert_eq!(result.max_point.y, 2.0);
        assert_eq!(result.max_point.z, 3.0);
    }

    #[test]
    fn test_two_points_unit_cube() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 0.0, y: 0.0, z: 0.0 },
                Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.volume, 1.0);
        assert_eq!(result.dimensions.x, 1.0);
        assert_eq!(result.dimensions.y, 1.0);
        assert_eq!(result.dimensions.z, 1.0);
    }

    #[test]
    fn test_multiple_points_rectangular_box() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 1.0, y: 2.0, z: 3.0 },
                Vector3D { x: 4.0, y: 6.0, z: 9.0 },
                Vector3D { x: 2.0, y: 3.0, z: 5.0 },
                Vector3D { x: 3.5, y: 5.5, z: 7.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        // Expected: (4-1) * (6-2) * (9-3) = 3 * 4 * 6 = 72
        assert_eq!(result.volume, 72.0);
        assert_eq!(result.min_point.x, 1.0);
        assert_eq!(result.min_point.y, 2.0);
        assert_eq!(result.min_point.z, 3.0);
        assert_eq!(result.max_point.x, 4.0);
        assert_eq!(result.max_point.y, 6.0);
        assert_eq!(result.max_point.z, 9.0);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: -2.0, y: -3.0, z: -1.0 },
                Vector3D { x: 1.0, y: 2.0, z: 3.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        // Expected: (1-(-2)) * (2-(-3)) * (3-(-1)) = 3 * 5 * 4 = 60
        assert_eq!(result.volume, 60.0);
        assert_eq!(result.min_point.x, -2.0);
        assert_eq!(result.min_point.y, -3.0);
        assert_eq!(result.min_point.z, -1.0);
        assert_eq!(result.max_point.x, 1.0);
        assert_eq!(result.max_point.y, 2.0);
        assert_eq!(result.max_point.z, 3.0);
    }

    #[test]
    fn test_all_same_points() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 5.0, y: 5.0, z: 5.0 },
                Vector3D { x: 5.0, y: 5.0, z: 5.0 },
                Vector3D { x: 5.0, y: 5.0, z: 5.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
        assert_eq!(result.min_point.x, 5.0);
        assert_eq!(result.min_point.y, 5.0);
        assert_eq!(result.min_point.z, 5.0);
        assert_eq!(result.max_point.x, 5.0);
        assert_eq!(result.max_point.y, 5.0);
        assert_eq!(result.max_point.z, 5.0);
    }

    #[test]
    fn test_zero_coordinates() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 0.0, y: 0.0, z: 0.0 },
                Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
    }

    #[test]
    fn test_large_values() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 1000000.0, y: 2000000.0, z: 3000000.0 },
                Vector3D { x: 1000001.0, y: 2000001.0, z: 3000001.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.volume, 1.0);
    }

    #[test]
    fn test_small_values() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 0.0001, y: 0.0002, z: 0.0003 },
                Vector3D { x: 0.0002, y: 0.0004, z: 0.0006 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        let expected = 0.0001 * 0.0002 * 0.0003;
        assert!((result.volume - expected).abs() < 1e-15);
    }

    #[test]
    fn test_empty_points_error() {
        let input = BoundingBoxInput {
            points: vec![],
        };
        let result = compute_aabb_volume(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "At least one point is required");
    }

    #[test]
    fn test_nan_input_error() {
        let input = BoundingBoxInput {
            points: vec![Vector3D { x: f64::NAN, y: 1.0, z: 2.0 }],
        };
        let result = compute_aabb_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("NaN values"));
    }

    #[test]
    fn test_infinite_input_error() {
        let input = BoundingBoxInput {
            points: vec![Vector3D { x: f64::INFINITY, y: 1.0, z: 2.0 }],
        };
        let result = compute_aabb_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite values"));
    }

    #[test]
    fn test_negative_infinite_input_error() {
        let input = BoundingBoxInput {
            points: vec![Vector3D { x: 1.0, y: f64::NEG_INFINITY, z: 2.0 }],
        };
        let result = compute_aabb_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("infinite values"));
    }

    #[test]
    fn test_box_type_field() {
        let input = BoundingBoxInput {
            points: vec![Vector3D { x: 0.0, y: 0.0, z: 0.0 }],
        };
        let result = compute_aabb_volume(input).unwrap();
        assert_eq!(result.box_type, "AABB (Axis-Aligned Bounding Box)");
    }

    #[test]
    fn test_scattered_points() {
        let input = BoundingBoxInput {
            points: vec![
                Vector3D { x: 10.0, y: 5.0, z: 15.0 },
                Vector3D { x: 2.0, y: 8.0, z: 3.0 },
                Vector3D { x: 7.0, y: 1.0, z: 12.0 },
                Vector3D { x: 15.0, y: 10.0, z: 1.0 },
                Vector3D { x: 5.0, y: 6.0, z: 8.0 },
            ],
        };
        let result = compute_aabb_volume(input).unwrap();
        // Min: (2, 1, 1), Max: (15, 10, 15)
        // Volume: (15-2) * (10-1) * (15-1) = 13 * 9 * 14 = 1638
        assert_eq!(result.volume, 1638.0);
        assert_eq!(result.min_point.x, 2.0);
        assert_eq!(result.min_point.y, 1.0);
        assert_eq!(result.min_point.z, 1.0);
        assert_eq!(result.max_point.x, 15.0);
        assert_eq!(result.max_point.y, 10.0);
        assert_eq!(result.max_point.z, 15.0);
    }
}