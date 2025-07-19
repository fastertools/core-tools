use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyramidInput {
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyramidResponse {
    pub volume: f64,
    pub calculation_method: String,
    pub base_area: f64,
    pub height: f64,
    pub base_points: Vec<Vector3D>,
    pub apex: Vector3D,
}

pub fn compute_pyramid_volume(input: PyramidInput) -> Result<PyramidResponse, String> {
    // Validate input
    if input.base_points.len() < 3 {
        return Err("At least 3 points are required for the base".to_string());
    }

    // Validate apex for NaN and infinite values
    if input.apex.x.is_nan() || input.apex.y.is_nan() || input.apex.z.is_nan() {
        return Err("Apex coordinates cannot contain NaN values".to_string());
    }
    if input.apex.x.is_infinite() || input.apex.y.is_infinite() || input.apex.z.is_infinite() {
        return Err("Apex coordinates cannot contain infinite values".to_string());
    }

    // Validate base points for NaN and infinite values
    for (i, point) in input.base_points.iter().enumerate() {
        if point.x.is_nan() || point.y.is_nan() || point.z.is_nan() {
            return Err(format!("Base point {i} contains NaN values"));
        }
        if point.x.is_infinite() || point.y.is_infinite() || point.z.is_infinite() {
            return Err(format!("Base point {i} contains infinite values"));
        }
    }

    // Calculate the area of the base polygon using the shoelace formula
    let base_area = calculate_polygon_area(&input.base_points)?;

    // Calculate the height by finding the distance from apex to the base plane
    let height = calculate_point_to_plane_distance(&input.apex, &input.base_points)?;

    // Volume = (1/3) * base_area * height
    let volume = (1.0 / 3.0) * base_area * height;

    Ok(PyramidResponse {
        volume,
        calculation_method: "Pyramid formula: (1/3) × base_area × height".to_string(),
        base_area,
        height,
        base_points: input.base_points,
        apex: input.apex,
    })
}

fn calculate_polygon_area(points: &[Vector3D]) -> Result<f64, String> {
    if points.len() < 3 {
        return Err("At least 3 points required for polygon area".to_string());
    }

    // Calculate the normal vector of the plane containing the polygon
    let v1 = Vector3D {
        x: points[1].x - points[0].x,
        y: points[1].y - points[0].y,
        z: points[1].z - points[0].z,
    };

    let v2 = Vector3D {
        x: points[2].x - points[0].x,
        y: points[2].y - points[0].y,
        z: points[2].z - points[0].z,
    };

    let normal = Vector3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };

    let normal_magnitude = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();

    if normal_magnitude < 1e-10 {
        return Err("Points are collinear, cannot form a polygon".to_string());
    }

    // Project the polygon onto the plane with the largest normal component
    let abs_nx = normal.x.abs();
    let abs_ny = normal.y.abs();
    let abs_nz = normal.z.abs();

    let mut area = 0.0;

    if abs_nz >= abs_nx && abs_nz >= abs_ny {
        // Project onto XY plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].y - points[j].x * points[i].y;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_nz);
    } else if abs_ny >= abs_nx {
        // Project onto XZ plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].x * points[j].z - points[j].x * points[i].z;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_ny);
    } else {
        // Project onto YZ plane
        for i in 0..points.len() {
            let j = (i + 1) % points.len();
            area += points[i].y * points[j].z - points[j].y * points[i].z;
        }
        area = area.abs() * normal_magnitude / (2.0 * abs_nx);
    }

    Ok(area)
}

fn calculate_point_to_plane_distance(
    point: &Vector3D,
    plane_points: &[Vector3D],
) -> Result<f64, String> {
    if plane_points.len() < 3 {
        return Err("At least 3 points required to define a plane".to_string());
    }

    // Calculate plane normal
    let v1 = Vector3D {
        x: plane_points[1].x - plane_points[0].x,
        y: plane_points[1].y - plane_points[0].y,
        z: plane_points[1].z - plane_points[0].z,
    };

    let v2 = Vector3D {
        x: plane_points[2].x - plane_points[0].x,
        y: plane_points[2].y - plane_points[0].y,
        z: plane_points[2].z - plane_points[0].z,
    };

    let normal = Vector3D {
        x: v1.y * v2.z - v1.z * v2.y,
        y: v1.z * v2.x - v1.x * v2.z,
        z: v1.x * v2.y - v1.y * v2.x,
    };

    let normal_magnitude = (normal.x * normal.x + normal.y * normal.y + normal.z * normal.z).sqrt();

    if normal_magnitude < 1e-10 {
        return Err("Points are collinear, cannot define a plane".to_string());
    }

    // Normalize the normal vector
    let unit_normal = Vector3D {
        x: normal.x / normal_magnitude,
        y: normal.y / normal_magnitude,
        z: normal.z / normal_magnitude,
    };

    // Vector from plane point to the test point
    let plane_to_point = Vector3D {
        x: point.x - plane_points[0].x,
        y: point.y - plane_points[0].y,
        z: point.z - plane_points[0].z,
    };

    // Distance is the dot product with the unit normal
    let distance = (plane_to_point.x * unit_normal.x
        + plane_to_point.y * unit_normal.y
        + plane_to_point.z * unit_normal.z)
        .abs();

    Ok(distance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangular_pyramid() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 1.0,
                y: 1.0,
                z: 3.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Base area = 0.5 * 2 * 2 = 2.0, Height = 3.0, Volume = (1/3) * 2 * 3 = 2.0
        assert!((result.volume - 2.0).abs() < 1e-10);
        assert!((result.base_area - 2.0).abs() < 1e-10);
        assert!((result.height - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_square_pyramid() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 2.0,
                    y: 2.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 2.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 1.0,
                y: 1.0,
                z: 3.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Base area = 2 * 2 = 4.0, Height = 3.0, Volume = (1/3) * 4 * 3 = 4.0
        assert!((result.volume - 4.0).abs() < 1e-10);
        assert!((result.base_area - 4.0).abs() < 1e-10);
        assert!((result.height - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_unit_cube_pyramid() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 1.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.5,
                y: 0.5,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Base area = 1.0, Height = 1.0, Volume = (1/3) * 1 * 1 = 1/3
        let expected = 1.0 / 3.0;
        assert!((result.volume - expected).abs() < 1e-15);
        assert!((result.base_area - 1.0).abs() < 1e-15);
        assert!((result.height - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_pentagon_pyramid() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.309,
                    y: 0.951,
                    z: 0.0,
                },
                Vector3D {
                    x: -0.809,
                    y: 0.588,
                    z: 0.0,
                },
                Vector3D {
                    x: -0.809,
                    y: -0.588,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.309,
                    y: -0.951,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Regular pentagon area ≈ 2.377, Height = 2.0
        let expected_area = 2.377; // Approximate
        let expected_volume = (1.0 / 3.0) * expected_area * 2.0;
        assert!((result.base_area - expected_area).abs() < 0.01);
        assert!((result.volume - expected_volume).abs() < 0.01);
        assert!((result.height - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_zero_height_pyramid() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.5,
                y: 0.5,
                z: 0.0,
            }, // Apex in same plane as base
        };
        let result = compute_pyramid_volume(input).unwrap();
        assert!((result.volume - 0.0).abs() < 1e-10);
        assert!((result.height - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_negative_coordinates() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: -1.0,
                    y: -1.0,
                    z: -1.0,
                },
                Vector3D {
                    x: 1.0,
                    y: -1.0,
                    z: -1.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 1.0,
                    z: -1.0,
                },
                Vector3D {
                    x: -1.0,
                    y: 1.0,
                    z: -1.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 2.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Base area = 2*2 = 4.0, Height = 3.0, Volume = (1/3) * 4 * 3 = 4.0
        assert!((result.volume - 4.0).abs() < 1e-10);
        assert!((result.height - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_large_coordinates() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 1000.0,
                    y: 1000.0,
                    z: 1000.0,
                },
                Vector3D {
                    x: 1001.0,
                    y: 1000.0,
                    z: 1000.0,
                },
                Vector3D {
                    x: 1000.0,
                    y: 1001.0,
                    z: 1000.0,
                },
            ],
            apex: Vector3D {
                x: 1000.5,
                y: 1000.5,
                z: 1001.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        // Base area = 0.5, Height = 1.0, Volume = (1/3) * 0.5 * 1 = 1/6
        let expected = 1.0 / 6.0;
        assert!((result.volume - expected).abs() < 1e-10);
        assert!((result.height - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculation_method_field() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input).unwrap();
        assert_eq!(
            result.calculation_method,
            "Pyramid formula: (1/3) × base_area × height"
        );
    }

    #[test]
    fn test_insufficient_base_points_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "At least 3 points are required for the base"
        );
    }

    #[test]
    fn test_collinear_base_points_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 2.0,
                    y: 0.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("collinear"));
    }

    #[test]
    fn test_nan_apex_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: f64::NAN,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Apex"));
    }

    #[test]
    fn test_infinite_apex_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: f64::INFINITY,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Apex"));
    }

    #[test]
    fn test_nan_base_point_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: f64::NAN,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 1.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Base point 0"));
    }

    #[test]
    fn test_infinite_base_point_error() {
        let input = PyramidInput {
            base_points: vec![
                Vector3D {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: f64::INFINITY,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3D {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            ],
            apex: Vector3D {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        };
        let result = compute_pyramid_volume(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Base point 1"));
    }
}
