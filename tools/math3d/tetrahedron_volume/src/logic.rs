use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TetrahedronVolumeInput {
    pub point_a: Vector3D,
    pub point_b: Vector3D,
    pub point_c: Vector3D,
    pub point_d: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TetrahedronVolumeResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub points: [Vector3D; 4],
}

pub fn compute_tetrahedron_volume(input: TetrahedronVolumeInput) -> Result<TetrahedronVolumeResponse, String> {
    // Validate all points for NaN and infinite values
    let points = [&input.point_a, &input.point_b, &input.point_c, &input.point_d];
    for (i, point) in points.iter().enumerate() {
        if point.x.is_nan() || point.y.is_nan() || point.z.is_nan() {
            return Err(format!("Point {} contains NaN values", ['A', 'B', 'C', 'D'][i]));
        }
        if point.x.is_infinite() || point.y.is_infinite() || point.z.is_infinite() {
            return Err(format!("Point {} contains infinite values", ['A', 'B', 'C', 'D'][i]));
        }
    }
    
    let a = &input.point_a;
    let b = &input.point_b;
    let c = &input.point_c;
    let d = &input.point_d;
    
    // Calculate vectors from point A to the other three points
    let ab = Vector3D {
        x: b.x - a.x,
        y: b.y - a.y,
        z: b.z - a.z,
    };
    
    let ac = Vector3D {
        x: c.x - a.x,
        y: c.y - a.y,
        z: c.z - a.z,
    };
    
    let ad = Vector3D {
        x: d.x - a.x,
        y: d.y - a.y,
        z: d.z - a.z,
    };
    
    // Calculate the scalar triple product: AB · (AC × AD)
    let cross_ac_ad = Vector3D {
        x: ac.y * ad.z - ac.z * ad.y,
        y: ac.z * ad.x - ac.x * ad.z,
        z: ac.x * ad.y - ac.y * ad.x,
    };
    
    let scalar_triple_product = ab.x * cross_ac_ad.x + ab.y * cross_ac_ad.y + ab.z * cross_ac_ad.z;
    
    // Volume = |scalar triple product| / 6
    let volume = scalar_triple_product.abs() / 6.0;
    
    Ok(TetrahedronVolumeResponse {
        volume,
        calculation_method: "Scalar triple product".to_string(),
        points: [input.point_a, input.point_b, input.point_c, input.point_d],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_tetrahedron() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        let expected = 1.0 / 6.0; // Volume of unit tetrahedron
        assert!((result.volume - expected).abs() < 1e-15);
        assert_eq!(result.calculation_method, "Scalar triple product");
    }

    #[test]
    fn test_scaled_tetrahedron() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 2.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 2.0, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 2.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        let expected = 8.0 / 6.0; // Scaled by factor 2 in each dimension, so volume scales by 2³ = 8
        assert!((result.volume - expected).abs() < 1e-15);
    }

    #[test]
    fn test_coplanar_points_zero_volume() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 2.0, y: 0.0, z: 0.0 },
            point_d: Vector3D { x: 3.0, y: 0.0, z: 0.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        assert!((result.volume - 0.0).abs() < 1e-15);
    }

    #[test]
    fn test_arbitrary_tetrahedron() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 1.0, y: 2.0, z: 3.0 },
            point_b: Vector3D { x: 4.0, y: 5.0, z: 6.0 },
            point_c: Vector3D { x: 7.0, y: 8.0, z: 9.0 },
            point_d: Vector3D { x: 2.0, y: 3.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        // Volume should be positive
        assert!(result.volume >= 0.0);
        // This specific configuration has volume = 0 due to coplanar vectors
        assert!((result.volume - 0.0).abs() < 1e-15);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: -1.0, y: -1.0, z: -1.0 },
            point_b: Vector3D { x: 1.0, y: -1.0, z: -1.0 },
            point_c: Vector3D { x: -1.0, y: 1.0, z: -1.0 },
            point_d: Vector3D { x: -1.0, y: -1.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        let expected = 8.0 / 6.0; // Volume of tetrahedron with edge length 2
        assert!((result.volume - expected).abs() < 1e-15);
    }

    #[test]
    fn test_same_points_zero_volume() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            point_b: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            point_c: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
            point_d: Vector3D { x: 1.0, y: 1.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        assert_eq!(result.volume, 0.0);
    }

    #[test]
    fn test_large_coordinates() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 1000.0, y: 1000.0, z: 1000.0 },
            point_b: Vector3D { x: 1001.0, y: 1000.0, z: 1000.0 },
            point_c: Vector3D { x: 1000.0, y: 1001.0, z: 1000.0 },
            point_d: Vector3D { x: 1000.0, y: 1000.0, z: 1001.0 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        let expected = 1.0 / 6.0; // Unit tetrahedron volume
        assert!((result.volume - expected).abs() < 1e-15);
    }

    #[test]
    fn test_small_coordinates() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 0.001, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 0.001, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 0.001 },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        let expected = (0.001 * 0.001 * 0.001) / 6.0; // Small tetrahedron volume
        assert!((result.volume - expected).abs() < 1e-18);
    }

    #[test]
    fn test_regular_tetrahedron() {
        let edge_length = 1.0;
        let height = edge_length * (2.0_f64.sqrt() / 3.0_f64.sqrt());
        
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: edge_length, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: edge_length / 2.0, y: edge_length * 3.0_f64.sqrt() / 2.0, z: 0.0 },
            point_d: Vector3D { x: edge_length / 2.0, y: edge_length * 3.0_f64.sqrt() / 6.0, z: height },
        };
        let result = compute_tetrahedron_volume(input).unwrap();
        
        // Regular tetrahedron volume = edge³ / (6√2)
        let expected = edge_length.powi(3) / (6.0 * 2.0_f64.sqrt());
        assert!((result.volume - expected).abs() < 1e-10);
    }

    #[test]
    fn test_points_array_preserved() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 1.0, y: 2.0, z: 3.0 },
            point_b: Vector3D { x: 4.0, y: 5.0, z: 6.0 },
            point_c: Vector3D { x: 7.0, y: 8.0, z: 9.0 },
            point_d: Vector3D { x: 10.0, y: 11.0, z: 12.0 },
        };
        let result = compute_tetrahedron_volume(input.clone()).unwrap();
        assert_eq!(result.points[0].x, input.point_a.x);
        assert_eq!(result.points[1].y, input.point_b.y);
        assert_eq!(result.points[2].z, input.point_c.z);
        assert_eq!(result.points[3].x, input.point_d.x);
    }

    #[test]
    fn test_nan_point_a_error() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: f64::NAN, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Point A"));
    }

    #[test]
    fn test_infinite_point_b_error() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: f64::INFINITY, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Point B"));
    }

    #[test]
    fn test_nan_point_c_error() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: f64::NAN, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: 1.0 },
        };
        let result = compute_tetrahedron_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Point C"));
    }

    #[test]
    fn test_infinite_point_d_error() {
        let input = TetrahedronVolumeInput {
            point_a: Vector3D { x: 0.0, y: 0.0, z: 0.0 },
            point_b: Vector3D { x: 1.0, y: 0.0, z: 0.0 },
            point_c: Vector3D { x: 0.0, y: 1.0, z: 0.0 },
            point_d: Vector3D { x: 0.0, y: 0.0, z: f64::NEG_INFINITY },
        };
        let result = compute_tetrahedron_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Point D"));
    }
}