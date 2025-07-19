use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct Point {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct PolygonSimplificationInput {
    pub polygon: Vec<Point>,
    pub tolerance_meters: f64,
    pub algorithm: Option<String>, // "douglas_peucker" or "visvalingam" (default: douglas_peucker)
}

#[derive(Serialize, Debug)]
pub struct PolygonSimplificationResult {
    pub original_polygon: Vec<Point>,
    pub simplified_polygon: Vec<Point>,
    pub original_vertex_count: usize,
    pub simplified_vertex_count: usize,
    pub reduction_percentage: f64,
    pub algorithm_used: String,
    pub tolerance_used_meters: f64,
}

const EARTH_RADIUS_M: f64 = 6378137.0;

pub fn haversine_distance(point1: &Point, point2: &Point) -> f64 {
    let lat1_rad = point1.lat.to_radians();
    let lat2_rad = point2.lat.to_radians();
    let delta_lat = (point2.lat - point1.lat).to_radians();
    let delta_lon = (point2.lon - point1.lon).to_radians();

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

pub fn perpendicular_distance(point: &Point, line_start: &Point, line_end: &Point) -> f64 {
    // Calculate perpendicular distance from point to line segment using cross product
    let line_length = haversine_distance(line_start, line_end);

    if line_length == 0.0 {
        return haversine_distance(point, line_start);
    }

    // Convert to approximate Cartesian coordinates for calculation
    let x0 = point.lon;
    let y0 = point.lat;
    let x1 = line_start.lon;
    let y1 = line_start.lat;
    let x2 = line_end.lon;
    let y2 = line_end.lat;

    // Calculate perpendicular distance using cross product formula
    let numerator = ((y2 - y1) * x0 - (x2 - x1) * y0 + x2 * y1 - y2 * x1).abs();
    let denominator = ((y2 - y1).powi(2) + (x2 - x1).powi(2)).sqrt();

    if denominator == 0.0 {
        return haversine_distance(point, line_start);
    }

    // Convert back to meters (approximate)
    let distance_degrees = numerator / denominator;
    distance_degrees * 111320.0 // Approximate meters per degree at equator
}

pub fn douglas_peucker_simplify(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() <= 2 {
        return points.to_vec();
    }

    let mut max_distance = 0.0;
    let mut max_index = 0;

    // Find the point with maximum distance from the line between first and last points
    for i in 1..points.len() - 1 {
        let distance = perpendicular_distance(&points[i], &points[0], &points[points.len() - 1]);
        if distance > max_distance {
            max_distance = distance;
            max_index = i;
        }
    }

    // If the maximum distance is greater than tolerance, recursively simplify
    if max_distance > tolerance {
        // Recursively simplify the two segments
        let left_segment = douglas_peucker_simplify(&points[0..=max_index], tolerance);
        let right_segment = douglas_peucker_simplify(&points[max_index..], tolerance);

        // Combine the results (avoiding duplicate middle point)
        let mut result = left_segment;
        result.extend(right_segment.into_iter().skip(1));
        result
    } else {
        // All points are within tolerance, return only endpoints
        vec![points[0], points[points.len() - 1]]
    }
}

pub fn visvalingam_simplify(points: &[Point], tolerance: f64) -> Vec<Point> {
    if points.len() <= 3 {
        return points.to_vec();
    }

    let mut result = points.to_vec();
    let mut areas: Vec<f64> = Vec::new();

    // Calculate initial effective areas for all points
    for i in 1..result.len() - 1 {
        let area = triangle_area(&result[i - 1], &result[i], &result[i + 1]);
        areas.push(area);
    }

    // Convert tolerance to area threshold (approximate)
    let area_threshold = tolerance * tolerance;

    // Remove points with smallest effective areas iteratively
    while areas.len() > 1 {
        // Find minimum area
        let (min_index, &min_area) = areas
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .unwrap();

        if min_area > area_threshold {
            break;
        }

        // Remove the point with minimum area
        let point_index = min_index + 1; // Account for first point not having area
        result.remove(point_index);
        areas.remove(min_index);

        // Update areas for neighboring points
        if min_index > 0 && min_index < areas.len() {
            let new_area = triangle_area(
                &result[min_index - 1],
                &result[min_index],
                &result[min_index + 1],
            );
            areas[min_index - 1] = new_area;
        }
        if min_index < areas.len() && min_index + 2 < result.len() {
            let new_area = triangle_area(
                &result[min_index],
                &result[min_index + 1],
                &result[min_index + 2],
            );
            areas[min_index] = new_area;
        }
    }

    result
}

pub fn triangle_area(p1: &Point, p2: &Point, p3: &Point) -> f64 {
    // Calculate area of triangle using cross product (approximate for small areas)
    let x1 = p1.lon;
    let y1 = p1.lat;
    let x2 = p2.lon;
    let y2 = p2.lat;
    let x3 = p3.lon;
    let y3 = p3.lat;

    0.5 * ((x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2)).abs())
}

pub fn polygon_simplification_logic(
    input: PolygonSimplificationInput,
) -> Result<PolygonSimplificationResult, String> {
    if input.polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }

    if input.tolerance_meters <= 0.0
        || input.tolerance_meters.is_nan()
        || input.tolerance_meters.is_infinite()
    {
        return Err("Tolerance must be positive and finite".to_string());
    }

    // Validate coordinates
    for point in &input.polygon {
        if point.lat.is_nan() || point.lat.is_infinite() {
            return Err("Point latitude cannot be NaN or infinite".to_string());
        }
        if point.lon.is_nan() || point.lon.is_infinite() {
            return Err("Point longitude cannot be NaN or infinite".to_string());
        }
        if point.lat < -90.0 || point.lat > 90.0 {
            return Err(format!(
                "Invalid latitude: {}. Must be between -90 and 90",
                point.lat
            ));
        }
        if point.lon < -180.0 || point.lon > 180.0 {
            return Err(format!(
                "Invalid longitude: {}. Must be between -180 and 180",
                point.lon
            ));
        }
    }

    let algorithm = input.algorithm.as_deref().unwrap_or("douglas_peucker");

    let simplified = match algorithm {
        "douglas_peucker" => douglas_peucker_simplify(&input.polygon, input.tolerance_meters),
        "visvalingam" => visvalingam_simplify(&input.polygon, input.tolerance_meters),
        _ => return Err("Algorithm must be 'douglas_peucker' or 'visvalingam'".to_string()),
    };

    let original_count = input.polygon.len();
    let simplified_count = simplified.len();
    let reduction_percentage = if original_count > 0 {
        ((original_count - simplified_count) as f64 / original_count as f64) * 100.0
    } else {
        0.0
    };

    Ok(PolygonSimplificationResult {
        original_polygon: input.polygon,
        simplified_polygon: simplified,
        original_vertex_count: original_count,
        simplified_vertex_count: simplified_count,
        reduction_percentage,
        algorithm_used: algorithm.to_string(),
        tolerance_used_meters: input.tolerance_meters,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_line_polygon() -> Vec<Point> {
        vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 0.1, lon: 0.1 },
            Point { lat: 0.2, lon: 0.2 },
            Point { lat: 0.3, lon: 0.3 },
            Point { lat: 1.0, lon: 1.0 },
        ]
    }

    fn create_complex_polygon() -> Vec<Point> {
        vec![
            Point { lat: 0.0, lon: 0.0 },
            Point {
                lat: 0.001,
                lon: 0.001,
            }, // Close to line
            Point { lat: 0.1, lon: 0.1 },
            Point {
                lat: 0.2,
                lon: 0.15,
            }, // Slight deviation
            Point { lat: 0.3, lon: 0.3 },
            Point { lat: 0.5, lon: 0.5 },
            Point { lat: 1.0, lon: 1.0 },
        ]
    }

    #[test]
    fn test_polygon_simplification_douglas_peucker() {
        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: 1000.0,
            algorithm: Some("douglas_peucker".to_string()),
        };

        let result = polygon_simplification_logic(input).unwrap();

        assert_eq!(result.algorithm_used, "douglas_peucker");
        assert_eq!(result.original_vertex_count, 5);
        assert!(result.simplified_vertex_count <= result.original_vertex_count);
        assert!(result.simplified_vertex_count >= 2); // At least start and end
        assert!(result.reduction_percentage >= 0.0);
        assert_eq!(result.tolerance_used_meters, 1000.0);
    }

    #[test]
    fn test_polygon_simplification_visvalingam() {
        let input = PolygonSimplificationInput {
            polygon: create_complex_polygon(),
            tolerance_meters: 500.0,
            algorithm: Some("visvalingam".to_string()),
        };

        let result = polygon_simplification_logic(input).unwrap();

        assert_eq!(result.algorithm_used, "visvalingam");
        assert_eq!(result.original_vertex_count, 7);
        assert!(result.simplified_vertex_count <= result.original_vertex_count);
        assert!(result.simplified_vertex_count >= 3); // Minimum for Visvalingam
    }

    #[test]
    fn test_polygon_simplification_default_algorithm() {
        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: 1000.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input).unwrap();

        assert_eq!(result.algorithm_used, "douglas_peucker"); // Default
    }

    #[test]
    fn test_douglas_peucker_simplify_basic() {
        let points = create_line_polygon();
        let simplified = douglas_peucker_simplify(&points, 1000.0);

        // Should reduce points significantly for a line
        assert!(simplified.len() <= points.len());
        assert!(simplified.len() >= 2); // At least start and end points
    }

    #[test]
    fn test_douglas_peucker_simplify_no_reduction() {
        let points = vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 1.0, lon: 0.0 },
            Point { lat: 0.5, lon: 1.0 }, // Forms significant triangle
        ];
        let simplified = douglas_peucker_simplify(&points, 10.0); // Very small tolerance

        // Should keep all points due to significant deviations
        assert_eq!(simplified.len(), points.len());
    }

    #[test]
    fn test_douglas_peucker_simplify_two_points() {
        let points = vec![Point { lat: 0.0, lon: 0.0 }, Point { lat: 1.0, lon: 1.0 }];
        let simplified = douglas_peucker_simplify(&points, 1000.0);

        assert_eq!(simplified.len(), 2);
        assert_eq!(simplified, points);
    }

    #[test]
    fn test_visvalingam_simplify_basic() {
        let points = create_complex_polygon();
        let simplified = visvalingam_simplify(&points, 500.0);

        assert!(simplified.len() <= points.len());
        assert!(simplified.len() >= 3); // Minimum for Visvalingam
    }

    #[test]
    fn test_visvalingam_simplify_three_points() {
        let points = vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 1.0, lon: 0.0 },
            Point { lat: 0.5, lon: 1.0 },
        ];
        let simplified = visvalingam_simplify(&points, 1000.0);

        assert_eq!(simplified.len(), 3);
        assert_eq!(simplified, points);
    }

    #[test]
    fn test_haversine_distance() {
        let p1 = Point { lat: 0.0, lon: 0.0 };
        let p2 = Point { lat: 0.0, lon: 1.0 }; // 1 degree longitude difference at equator

        let distance = haversine_distance(&p1, &p2);

        // Should be approximately 111 km at equator
        assert!(distance > 100_000.0);
        assert!(distance < 120_000.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let p1 = Point {
            lat: 40.7128,
            lon: -74.0060,
        };
        let p2 = Point {
            lat: 40.7128,
            lon: -74.0060,
        };

        let distance = haversine_distance(&p1, &p2);

        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_perpendicular_distance() {
        let point = Point { lat: 0.5, lon: 0.5 };
        let line_start = Point { lat: 0.0, lon: 0.0 };
        let line_end = Point { lat: 1.0, lon: 1.0 };

        let distance = perpendicular_distance(&point, &line_start, &line_end);

        // Point is on the line, so distance should be small
        assert!(distance < 1000.0); // Less than 1km
    }

    #[test]
    fn test_perpendicular_distance_zero_length_line() {
        let point = Point { lat: 0.5, lon: 0.5 };
        let line_start = Point { lat: 0.0, lon: 0.0 };
        let line_end = Point { lat: 0.0, lon: 0.0 }; // Same point

        let distance = perpendicular_distance(&point, &line_start, &line_end);

        // Should return distance from point to line_start
        let expected = haversine_distance(&point, &line_start);
        assert!((distance - expected).abs() < 1.0);
    }

    #[test]
    fn test_triangle_area() {
        let p1 = Point { lat: 0.0, lon: 0.0 };
        let p2 = Point { lat: 1.0, lon: 0.0 };
        let p3 = Point { lat: 0.0, lon: 1.0 };

        let area = triangle_area(&p1, &p2, &p3);

        // Should be 0.5 for unit triangle
        assert!((area - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_triangle_area_collinear_points() {
        let p1 = Point { lat: 0.0, lon: 0.0 };
        let p2 = Point { lat: 0.5, lon: 0.5 };
        let p3 = Point { lat: 1.0, lon: 1.0 };

        let area = triangle_area(&p1, &p2, &p3);

        // Collinear points should have zero area
        assert!(area < 0.01);
    }

    #[test]
    fn test_polygon_simplification_insufficient_vertices() {
        let input = PolygonSimplificationInput {
            polygon: vec![Point { lat: 0.0, lon: 0.0 }, Point { lat: 1.0, lon: 1.0 }],
            tolerance_meters: 1000.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Polygon must have at least 3 vertices");
    }

    #[test]
    fn test_polygon_simplification_invalid_tolerance() {
        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: -100.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Tolerance must be positive"));

        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: 0.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Tolerance must be positive"));
    }

    #[test]
    fn test_polygon_simplification_invalid_algorithm() {
        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: 1000.0,
            algorithm: Some("invalid_algorithm".to_string()),
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Algorithm must be 'douglas_peucker' or 'visvalingam'"
        );
    }

    #[test]
    fn test_polygon_simplification_invalid_coordinates() {
        let mut invalid_polygon = create_line_polygon();
        invalid_polygon[0].lat = 91.0; // Invalid latitude

        let input = PolygonSimplificationInput {
            polygon: invalid_polygon,
            tolerance_meters: 1000.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid latitude"));
    }

    #[test]
    fn test_polygon_simplification_nan_coordinates() {
        let mut nan_polygon = create_line_polygon();
        nan_polygon[0].lat = f64::NAN;

        let input = PolygonSimplificationInput {
            polygon: nan_polygon,
            tolerance_meters: 1000.0,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Point latitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_polygon_simplification_infinite_tolerance() {
        let input = PolygonSimplificationInput {
            polygon: create_line_polygon(),
            tolerance_meters: f64::INFINITY,
            algorithm: None,
        };

        let result = polygon_simplification_logic(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Tolerance must be positive and finite");
    }

    #[test]
    fn test_reduction_percentage_calculation() {
        let input = PolygonSimplificationInput {
            polygon: create_complex_polygon(),
            tolerance_meters: 10000.0, // High tolerance for significant reduction
            algorithm: Some("douglas_peucker".to_string()),
        };

        let result = polygon_simplification_logic(input).unwrap();

        // Should have some reduction
        assert!(result.reduction_percentage >= 0.0);
        assert!(result.reduction_percentage <= 100.0);

        let expected_percentage = ((result.original_vertex_count - result.simplified_vertex_count)
            as f64
            / result.original_vertex_count as f64)
            * 100.0;
        assert!((result.reduction_percentage - expected_percentage).abs() < 0.01);
    }

    #[test]
    fn test_algorithm_output_consistency() {
        let input = PolygonSimplificationInput {
            polygon: create_complex_polygon(),
            tolerance_meters: 1000.0,
            algorithm: Some("douglas_peucker".to_string()),
        };

        let result = polygon_simplification_logic(input).unwrap();

        // Simplified polygon should preserve first and last points for closed polygons
        assert_eq!(
            result.simplified_polygon.first(),
            result.original_polygon.first()
        );
        assert_eq!(
            result.simplified_polygon.last(),
            result.original_polygon.last()
        );
    }
}
