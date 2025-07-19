use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub struct Point {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct PointInPolygonInput {
    /// Point to test
    #[allow(dead_code)]
    pub point: Point,
    /// Polygon vertices
    #[allow(dead_code)]
    pub polygon: Vec<Point>,
}

#[derive(Serialize, Debug)]
pub struct PointInPolygonResult {
    pub is_inside: bool,
    pub algorithm_used: String,
    pub on_boundary: bool,
}

const EPSILON: f64 = 1e-10;

pub fn ray_casting_algorithm(point: &Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let x = point.lon;
    let y = point.lat;
    let mut inside = false;
    let n = polygon.len();

    let mut j = n - 1;
    for i in 0..n {
        let xi = polygon[i].lon;
        let yi = polygon[i].lat;
        let xj = polygon[j].lon;
        let yj = polygon[j].lat;

        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }

    inside
}

pub fn is_on_boundary(point: &Point, polygon: &[Point]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let n = polygon.len();

    for i in 0..n {
        let j = (i + 1) % n;
        if is_point_on_segment(point, &polygon[i], &polygon[j]) {
            return true;
        }
    }

    false
}

pub fn is_point_on_segment(point: &Point, seg_start: &Point, seg_end: &Point) -> bool {
    let cross_product = (point.lat - seg_start.lat) * (seg_end.lon - seg_start.lon)
        - (point.lon - seg_start.lon) * (seg_end.lat - seg_start.lat);

    if cross_product.abs() > EPSILON {
        return false;
    }

    let dot_product = (point.lon - seg_start.lon) * (seg_end.lon - seg_start.lon)
        + (point.lat - seg_start.lat) * (seg_end.lat - seg_start.lat);

    let squared_length = (seg_end.lon - seg_start.lon) * (seg_end.lon - seg_start.lon)
        + (seg_end.lat - seg_start.lat) * (seg_end.lat - seg_start.lat);

    dot_product >= 0.0 && dot_product <= squared_length
}

pub fn point_in_polygon_check(
    point: Point,
    polygon: Vec<Point>,
) -> Result<PointInPolygonResult, String> {
    if polygon.len() < 3 {
        return Err("Polygon must have at least 3 vertices".to_string());
    }

    // Validate coordinates
    for poly_point in &polygon {
        if poly_point.lat.is_nan() || poly_point.lat.is_infinite() {
            return Err("Polygon vertex latitude cannot be NaN or infinite".to_string());
        }
        if poly_point.lon.is_nan() || poly_point.lon.is_infinite() {
            return Err("Polygon vertex longitude cannot be NaN or infinite".to_string());
        }
        if poly_point.lat < -90.0 || poly_point.lat > 90.0 {
            return Err(format!(
                "Invalid latitude: {}. Must be between -90 and 90",
                poly_point.lat
            ));
        }
        if poly_point.lon < -180.0 || poly_point.lon > 180.0 {
            return Err(format!(
                "Invalid longitude: {}. Must be between -180 and 180",
                poly_point.lon
            ));
        }
    }

    if point.lat.is_nan() || point.lat.is_infinite() {
        return Err("Point latitude cannot be NaN or infinite".to_string());
    }
    if point.lon.is_nan() || point.lon.is_infinite() {
        return Err("Point longitude cannot be NaN or infinite".to_string());
    }
    if point.lat < -90.0 || point.lat > 90.0 {
        return Err(format!(
            "Invalid point latitude: {}. Must be between -90 and 90",
            point.lat
        ));
    }
    if point.lon < -180.0 || point.lon > 180.0 {
        return Err(format!(
            "Invalid point longitude: {}. Must be between -180 and 180",
            point.lon
        ));
    }

    let on_boundary = is_on_boundary(&point, &polygon);
    let is_inside = ray_casting_algorithm(&point, &polygon);

    Ok(PointInPolygonResult {
        is_inside,
        algorithm_used: "ray_casting".to_string(),
        on_boundary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_square() -> Vec<Point> {
        vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 0.0, lon: 1.0 },
            Point { lat: 1.0, lon: 1.0 },
            Point { lat: 1.0, lon: 0.0 },
        ]
    }

    fn create_triangle() -> Vec<Point> {
        vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 1.0, lon: 0.0 },
            Point { lat: 0.5, lon: 1.0 },
        ]
    }

    #[test]
    fn test_point_in_polygon_inside_square() {
        let square = create_square();
        let point = Point { lat: 0.5, lon: 0.5 };

        let result = point_in_polygon_check(point, square).unwrap();

        assert!(result.is_inside);
        assert!(!result.on_boundary);
        assert_eq!(result.algorithm_used, "ray_casting");
    }

    #[test]
    fn test_point_in_polygon_outside_square() {
        let square = create_square();
        let point = Point { lat: 2.0, lon: 2.0 };

        let result = point_in_polygon_check(point, square).unwrap();

        assert!(!result.is_inside);
        assert!(!result.on_boundary);
        assert_eq!(result.algorithm_used, "ray_casting");
    }

    #[test]
    fn test_point_in_polygon_on_boundary() {
        let square = create_square();
        let point = Point { lat: 0.0, lon: 0.5 }; // On bottom edge

        let result = point_in_polygon_check(point, square).unwrap();

        assert_eq!(result.algorithm_used, "ray_casting");
        assert!(result.on_boundary);
    }

    #[test]
    fn test_point_in_polygon_at_vertex() {
        let square = create_square();
        let point = Point { lat: 0.0, lon: 0.0 }; // At corner

        let result = point_in_polygon_check(point, square).unwrap();

        assert_eq!(result.algorithm_used, "ray_casting");
        assert!(result.on_boundary);
    }

    #[test]
    fn test_point_in_triangle() {
        let triangle = create_triangle();
        let point_inside = Point { lat: 0.5, lon: 0.3 };
        let point_outside = Point { lat: 0.5, lon: 1.5 }; // Clearly outside

        let result_inside = point_in_polygon_check(point_inside, triangle.clone()).unwrap();
        assert!(result_inside.is_inside);
        assert!(!result_inside.on_boundary);

        let result_outside = point_in_polygon_check(point_outside, triangle).unwrap();
        assert!(!result_outside.is_inside);
        assert!(!result_outside.on_boundary);
    }

    #[test]
    fn test_ray_casting_algorithm_simple() {
        let square = create_square();

        assert!(ray_casting_algorithm(
            &Point { lat: 0.5, lon: 0.5 },
            &square
        ));
        assert!(!ray_casting_algorithm(
            &Point { lat: 2.0, lon: 2.0 },
            &square
        ));
        assert!(!ray_casting_algorithm(
            &Point {
                lat: -1.0,
                lon: -1.0
            },
            &square
        ));
    }

    #[test]
    fn test_ray_casting_complex_polygon() {
        // L-shaped polygon
        let l_shape = vec![
            Point { lat: 0.0, lon: 0.0 },
            Point { lat: 0.0, lon: 3.0 },
            Point { lat: 1.0, lon: 3.0 },
            Point { lat: 1.0, lon: 1.0 },
            Point { lat: 2.0, lon: 1.0 },
            Point { lat: 2.0, lon: 0.0 },
        ];

        assert!(ray_casting_algorithm(
            &Point { lat: 0.5, lon: 0.5 },
            &l_shape
        ));
        assert!(ray_casting_algorithm(
            &Point { lat: 0.5, lon: 2.0 },
            &l_shape
        ));
        assert!(!ray_casting_algorithm(
            &Point { lat: 1.5, lon: 2.0 },
            &l_shape
        ));
        assert!(!ray_casting_algorithm(
            &Point { lat: 3.0, lon: 1.5 },
            &l_shape
        )); // Clearly outside
    }

    #[test]
    fn test_is_point_on_segment() {
        let start = Point { lat: 0.0, lon: 0.0 };
        let end = Point { lat: 1.0, lon: 1.0 };

        // Point on segment
        assert!(is_point_on_segment(
            &Point { lat: 0.5, lon: 0.5 },
            &start,
            &end
        ));

        // Point at start
        assert!(is_point_on_segment(
            &Point { lat: 0.0, lon: 0.0 },
            &start,
            &end
        ));

        // Point at end
        assert!(is_point_on_segment(
            &Point { lat: 1.0, lon: 1.0 },
            &start,
            &end
        ));

        // Point not on segment
        assert!(!is_point_on_segment(
            &Point { lat: 0.5, lon: 0.6 },
            &start,
            &end
        ));

        // Point on line but outside segment
        assert!(!is_point_on_segment(
            &Point { lat: 2.0, lon: 2.0 },
            &start,
            &end
        ));
        assert!(!is_point_on_segment(
            &Point {
                lat: -0.5,
                lon: -0.5
            },
            &start,
            &end
        ));
    }

    #[test]
    fn test_is_on_boundary_square() {
        let square = create_square();

        // Points on edges
        assert!(is_on_boundary(&Point { lat: 0.0, lon: 0.5 }, &square));
        assert!(is_on_boundary(&Point { lat: 0.5, lon: 0.0 }, &square));
        assert!(is_on_boundary(&Point { lat: 1.0, lon: 0.5 }, &square));
        assert!(is_on_boundary(&Point { lat: 0.5, lon: 1.0 }, &square));

        // Points at vertices
        assert!(is_on_boundary(&Point { lat: 0.0, lon: 0.0 }, &square));
        assert!(is_on_boundary(&Point { lat: 1.0, lon: 1.0 }, &square));

        // Points not on boundary
        assert!(!is_on_boundary(&Point { lat: 0.5, lon: 0.5 }, &square));
        assert!(!is_on_boundary(&Point { lat: 2.0, lon: 2.0 }, &square));
    }

    #[test]
    fn test_point_in_polygon_insufficient_vertices() {
        let line = vec![Point { lat: 0.0, lon: 0.0 }, Point { lat: 1.0, lon: 1.0 }];

        let point = Point { lat: 0.5, lon: 0.5 };
        let result = point_in_polygon_check(point, line);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Polygon must have at least 3 vertices");
    }

    #[test]
    fn test_point_in_polygon_invalid_point_coordinates() {
        let square = create_square();

        // Invalid latitude
        let invalid_point = Point {
            lat: 91.0,
            lon: 0.0,
        };
        let result = point_in_polygon_check(invalid_point, square.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid point latitude"));

        // Invalid longitude
        let invalid_point = Point {
            lat: 0.0,
            lon: 181.0,
        };
        let result = point_in_polygon_check(invalid_point, square);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid point longitude"));
    }

    #[test]
    fn test_point_in_polygon_invalid_polygon_coordinates() {
        let mut invalid_polygon = create_square();
        invalid_polygon[0].lat = 91.0; // Invalid latitude

        let point = Point { lat: 0.5, lon: 0.5 };
        let result = point_in_polygon_check(point, invalid_polygon);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid latitude"));
    }

    #[test]
    fn test_point_in_polygon_nan_coordinates() {
        let square = create_square();

        // NaN point coordinates
        let nan_point = Point {
            lat: f64::NAN,
            lon: 0.0,
        };
        let result = point_in_polygon_check(nan_point, square.clone());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Point latitude cannot be NaN or infinite"
        );

        // NaN polygon coordinates
        let mut nan_polygon = create_square();
        nan_polygon[0].lon = f64::NAN;
        let point = Point { lat: 0.5, lon: 0.5 };
        let result = point_in_polygon_check(point, nan_polygon);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Polygon vertex longitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_point_in_polygon_infinite_coordinates() {
        let square = create_square();

        // Infinite point coordinates
        let inf_point = Point {
            lat: f64::INFINITY,
            lon: 0.0,
        };
        let result = point_in_polygon_check(inf_point, square.clone());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Point latitude cannot be NaN or infinite"
        );

        // Infinite polygon coordinates
        let mut inf_polygon = create_square();
        inf_polygon[1].lat = f64::NEG_INFINITY;
        let point = Point { lat: 0.5, lon: 0.5 };
        let result = point_in_polygon_check(point, inf_polygon);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Polygon vertex latitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_point_in_polygon_boundary_coordinates() {
        // Test with boundary valid coordinates
        let boundary_polygon = vec![
            Point {
                lat: -90.0,
                lon: -180.0,
            },
            Point {
                lat: -90.0,
                lon: 180.0,
            },
            Point {
                lat: 90.0,
                lon: 180.0,
            },
            Point {
                lat: 90.0,
                lon: -180.0,
            },
        ];

        let point = Point { lat: 0.0, lon: 0.0 };
        let result = point_in_polygon_check(point, boundary_polygon);

        assert!(result.is_ok());
        assert!(result.unwrap().is_inside);
    }

    #[test]
    fn test_point_in_polygon_real_world_coordinates() {
        // Manhattan-like polygon (rough approximation)
        let manhattan = vec![
            Point {
                lat: 40.700,
                lon: -74.025,
            },
            Point {
                lat: 40.700,
                lon: -73.930,
            },
            Point {
                lat: 40.820,
                lon: -73.930,
            },
            Point {
                lat: 40.820,
                lon: -74.025,
            },
        ];

        // Point in Times Square
        let times_square = Point {
            lat: 40.758,
            lon: -73.985,
        };
        let result = point_in_polygon_check(times_square, manhattan.clone()).unwrap();
        assert!(result.is_inside);

        // Point in Brooklyn (outside)
        let brooklyn = Point {
            lat: 40.650,
            lon: -73.950,
        };
        let result = point_in_polygon_check(brooklyn, manhattan).unwrap();
        assert!(!result.is_inside);
    }

    #[test]
    fn test_point_in_polygon_edge_cases() {
        let square = create_square();

        // Point very close to boundary but not on it
        let near_boundary = Point {
            lat: 0.0000001,
            lon: 0.5,
        };
        let result = point_in_polygon_check(near_boundary, square.clone()).unwrap();
        assert!(result.is_inside);
        assert!(!result.on_boundary);

        // Point just outside
        let just_outside = Point {
            lat: -0.0000001,
            lon: 0.5,
        };
        let result = point_in_polygon_check(just_outside, square).unwrap();
        assert!(!result.is_inside);
        assert!(!result.on_boundary);
    }

    #[test]
    fn test_ray_casting_with_fewer_than_three_points() {
        let line = vec![Point { lat: 0.0, lon: 0.0 }, Point { lat: 1.0, lon: 1.0 }];

        let point = Point { lat: 0.5, lon: 0.5 };
        assert!(!ray_casting_algorithm(&point, &line));

        let single_point = vec![Point { lat: 0.0, lon: 0.0 }];
        assert!(!ray_casting_algorithm(&point, &single_point));

        let empty: Vec<Point> = vec![];
        assert!(!ray_casting_algorithm(&point, &empty));
    }

    #[test]
    fn test_is_on_boundary_with_insufficient_points() {
        let line = vec![Point { lat: 0.0, lon: 0.0 }, Point { lat: 1.0, lon: 1.0 }];

        let point = Point { lat: 0.5, lon: 0.5 };
        assert!(!is_on_boundary(&point, &line));
    }
}
