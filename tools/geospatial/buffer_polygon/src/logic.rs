use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize, Serialize, Clone, Copy, Debug)]
pub struct Point {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct CircularBufferInput {
    /// Center point for the buffer
    pub center: Point,
    /// Buffer radius in meters
    pub radius_meters: f64,
    /// Number of points to approximate circle (8-360, default 32)
    pub num_points: Option<usize>,
}

#[derive(Serialize, Debug)]
pub struct BufferResult {
    pub buffer_polygon: Vec<Point>,
    pub area_square_meters: f64,
    pub perimeter_meters: f64,
    pub algorithm_used: String,
}

const EARTH_RADIUS_M: f64 = 6378137.0; // WGS84 equatorial radius

pub fn create_circular_buffer(
    center: Point,
    radius_meters: f64,
    num_points: Option<usize>,
) -> Result<BufferResult, String> {
    if radius_meters <= 0.0 {
        return Err("Radius must be positive".to_string());
    }

    if center.lat < -90.0 || center.lat > 90.0 {
        return Err(format!(
            "Invalid latitude: {}. Must be between -90 and 90",
            center.lat
        ));
    }
    if center.lon < -180.0 || center.lon > 180.0 {
        return Err(format!(
            "Invalid longitude: {}. Must be between -180 and 180",
            center.lon
        ));
    }

    let num_points = num_points.unwrap_or(32).max(8).min(360);
    let mut buffer_points = Vec::new();

    let lat_rad = center.lat * PI / 180.0;
    let lon_rad = center.lon * PI / 180.0;

    // Angular distance
    let angular_distance = radius_meters / EARTH_RADIUS_M;

    for i in 0..num_points {
        let bearing = 2.0 * PI * i as f64 / num_points as f64;

        // Calculate destination point using spherical trigonometry
        let dest_lat_rad = (lat_rad.sin() * angular_distance.cos()
            + lat_rad.cos() * angular_distance.sin() * bearing.cos())
        .asin();

        let dest_lon_rad = lon_rad
            + (bearing.sin() * angular_distance.sin() * lat_rad.cos())
                .atan2(angular_distance.cos() - lat_rad.sin() * dest_lat_rad.sin());

        buffer_points.push(Point {
            lat: dest_lat_rad * 180.0 / PI,
            lon: dest_lon_rad * 180.0 / PI,
        });
    }

    // Calculate area (approximately πr²)
    let area = PI * radius_meters * radius_meters;

    // Calculate perimeter (2πr)
    let perimeter = 2.0 * PI * radius_meters;

    Ok(BufferResult {
        buffer_polygon: buffer_points,
        area_square_meters: area,
        perimeter_meters: perimeter,
        algorithm_used: "circular_geodesic".to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_buffer_basic() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
        }; // New York City
        let radius = 1000.0; // 1km

        let result = create_circular_buffer(center, radius, Some(16)).unwrap();

        assert_eq!(result.buffer_polygon.len(), 16);
        assert!((result.area_square_meters - PI * radius * radius).abs() < 1.0);
        assert!((result.perimeter_meters - 2.0 * PI * radius).abs() < 1.0);
        assert_eq!(result.algorithm_used, "circular_geodesic");
    }

    #[test]
    fn test_circular_buffer_default_points() {
        let center = Point { lat: 0.0, lon: 0.0 };
        let radius = 500.0;

        let result = create_circular_buffer(center, radius, None).unwrap();

        assert_eq!(result.buffer_polygon.len(), 32); // Default value
        assert!((result.area_square_meters - PI * radius * radius).abs() < 1.0);
    }

    #[test]
    fn test_circular_buffer_point_constraints() {
        let center = Point {
            lat: 45.0,
            lon: 0.0,
        };
        let radius = 1000.0;

        // Test minimum points constraint
        let result = create_circular_buffer(center, radius, Some(4)).unwrap();
        assert_eq!(result.buffer_polygon.len(), 8); // Should be clamped to 8

        // Test maximum points constraint
        let result = create_circular_buffer(center, radius, Some(500)).unwrap();
        assert_eq!(result.buffer_polygon.len(), 360); // Should be clamped to 360
    }

    #[test]
    fn test_circular_buffer_equator() {
        let center = Point { lat: 0.0, lon: 0.0 }; // Equator
        let radius = 1000.0;

        let result = create_circular_buffer(center, radius, Some(8)).unwrap();

        assert_eq!(result.buffer_polygon.len(), 8);
        // Verify points are distributed around the center
        let first_point = &result.buffer_polygon[0];
        assert!(first_point.lat.abs() < 0.1); // Should be close to equator
    }

    #[test]
    fn test_circular_buffer_poles() {
        let center = Point {
            lat: 89.0,
            lon: 0.0,
        }; // Near North Pole
        let radius = 1000.0;

        let result = create_circular_buffer(center, radius, Some(8)).unwrap();

        assert_eq!(result.buffer_polygon.len(), 8);
        // All points should be close to the center latitude
        for point in result.buffer_polygon {
            assert!(
                (point.lat - center.lat).abs() < 1.0,
                "Point latitude {} too far from center {}",
                point.lat,
                center.lat
            );
        }
    }

    #[test]
    fn test_circular_buffer_small_radius() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
        };
        let radius = 1.0; // 1 meter

        let result = create_circular_buffer(center, radius, Some(16)).unwrap();

        assert_eq!(result.buffer_polygon.len(), 16);
        assert!((result.area_square_meters - PI * radius * radius).abs() < 1e-6);

        // Points should be very close to the center
        for point in result.buffer_polygon {
            assert!((point.lat - center.lat).abs() < 0.001);
            assert!((point.lon - center.lon).abs() < 0.001);
        }
    }

    #[test]
    fn test_circular_buffer_large_radius() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
        };
        let radius = 100000.0; // 100km

        let result = create_circular_buffer(center, radius, Some(16)).unwrap();

        assert_eq!(result.buffer_polygon.len(), 16);
        assert!((result.area_square_meters - PI * radius * radius).abs() < 1000.0);

        // Points should be farther from the center
        for point in result.buffer_polygon {
            let lat_diff = (point.lat - center.lat).abs();
            let lon_diff = (point.lon - center.lon).abs();
            assert!(lat_diff > 0.1 || lon_diff > 0.1); // Should be significantly displaced
        }
    }

    #[test]
    fn test_circular_buffer_negative_radius() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
        };
        let radius = -1000.0;

        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius must be positive");
    }

    #[test]
    fn test_circular_buffer_zero_radius() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
        };
        let radius = 0.0;

        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Radius must be positive");
    }

    #[test]
    fn test_circular_buffer_invalid_latitude() {
        let center = Point {
            lat: 91.0,
            lon: 0.0,
        }; // Invalid latitude
        let radius = 1000.0;

        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid latitude"));

        let center = Point {
            lat: -91.0,
            lon: 0.0,
        }; // Invalid latitude
        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid latitude"));
    }

    #[test]
    fn test_circular_buffer_invalid_longitude() {
        let center = Point {
            lat: 40.0,
            lon: 181.0,
        }; // Invalid longitude
        let radius = 1000.0;

        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid longitude"));

        let center = Point {
            lat: 40.0,
            lon: -181.0,
        }; // Invalid longitude
        let result = create_circular_buffer(center, radius, Some(16));

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid longitude"));
    }

    #[test]
    fn test_circular_buffer_boundary_coordinates() {
        // Test boundary latitude values
        let center = Point {
            lat: 90.0,
            lon: 0.0,
        }; // North Pole
        let radius = 1000.0;
        let result = create_circular_buffer(center, radius, Some(8));
        assert!(result.is_ok());

        let center = Point {
            lat: -90.0,
            lon: 0.0,
        }; // South Pole
        let result = create_circular_buffer(center, radius, Some(8));
        assert!(result.is_ok());

        // Test boundary longitude values
        let center = Point {
            lat: 0.0,
            lon: 180.0,
        }; // Date line
        let result = create_circular_buffer(center, radius, Some(8));
        assert!(result.is_ok());

        let center = Point {
            lat: 0.0,
            lon: -180.0,
        }; // Date line
        let result = create_circular_buffer(center, radius, Some(8));
        assert!(result.is_ok());
    }

    #[test]
    fn test_circular_buffer_point_distribution() {
        let center = Point {
            lat: 45.0,
            lon: 0.0,
        };
        let radius = 1000.0;

        let result = create_circular_buffer(center, radius, Some(4)).unwrap();

        // With 4 points (clamped to 8), verify they form a reasonable polygon
        assert_eq!(result.buffer_polygon.len(), 8);

        // Check that points are distributed around the center
        let points = &result.buffer_polygon;
        let mut has_north = false;
        let mut has_south = false;
        let mut has_east = false;
        let mut has_west = false;

        for point in points {
            if point.lat > center.lat {
                has_north = true;
            }
            if point.lat < center.lat {
                has_south = true;
            }
            if point.lon > center.lon {
                has_east = true;
            }
            if point.lon < center.lon {
                has_west = true;
            }
        }

        assert!(
            has_north && has_south && has_east && has_west,
            "Points should be distributed in all directions"
        );
    }
}
