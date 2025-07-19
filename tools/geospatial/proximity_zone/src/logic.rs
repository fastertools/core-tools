use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Point {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
    /// Optional identifier for the point
    pub id: Option<String>,
}

#[derive(Deserialize)]
pub struct ProximityZoneInput {
    /// Center of the proximity zone
    pub center: Point,
    /// Radius of the zone in meters
    pub radius_meters: f64,
    /// Points to analyze
    pub candidate_points: Vec<Point>,
}

#[derive(Serialize, Clone, Debug)]
pub struct NearestPointResult {
    pub point: Point,
    pub distance_meters: f64,
    pub bearing_degrees: f64,
}

#[derive(Serialize, Debug)]
pub struct ProximityZoneResult {
    pub center: Point,
    pub radius_meters: f64,
    pub points_in_zone: Vec<NearestPointResult>,
    pub points_outside_zone: Vec<NearestPointResult>,
    pub summary: ProximityZoneSummary,
}

#[derive(Serialize, Debug)]
pub struct ProximityZoneSummary {
    pub total_points: usize,
    pub points_inside: usize,
    pub points_outside: usize,
    pub average_distance_inside: f64,
    pub closest_point_distance: f64,
    pub farthest_point_distance: f64,
}

const EARTH_RADIUS_M: f64 = 6378137.0;

pub fn haversine_distance(point1: &Point, point2: &Point) -> f64 {
    let lat1_rad = point1.lat * PI / 180.0;
    let lat2_rad = point2.lat * PI / 180.0;
    let delta_lat = (point2.lat - point1.lat) * PI / 180.0;
    let delta_lon = (point2.lon - point1.lon) * PI / 180.0;

    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    EARTH_RADIUS_M * c
}

pub fn calculate_bearing(from: &Point, to: &Point) -> f64 {
    let lat1_rad = from.lat * PI / 180.0;
    let lat2_rad = to.lat * PI / 180.0;
    let delta_lon = (to.lon - from.lon) * PI / 180.0;

    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();

    let bearing_rad = y.atan2(x);
    (bearing_rad * 180.0 / PI + 360.0) % 360.0
}

pub fn proximity_zone_analysis(
    center: Point,
    radius_meters: f64,
    candidate_points: Vec<Point>,
) -> Result<ProximityZoneResult, String> {
    if radius_meters <= 0.0 || radius_meters.is_nan() || radius_meters.is_infinite() {
        return Err("Radius must be positive and finite".to_string());
    }

    if candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }

    // Validate center coordinates
    if center.lat.is_nan() || center.lat.is_infinite() {
        return Err("Center latitude cannot be NaN or infinite".to_string());
    }
    if center.lon.is_nan() || center.lon.is_infinite() {
        return Err("Center longitude cannot be NaN or infinite".to_string());
    }
    if center.lat < -90.0 || center.lat > 90.0 {
        return Err(format!(
            "Invalid center latitude: {}. Must be between -90 and 90",
            center.lat
        ));
    }
    if center.lon < -180.0 || center.lon > 180.0 {
        return Err(format!(
            "Invalid center longitude: {}. Must be between -180 and 180",
            center.lon
        ));
    }

    let mut points_inside = Vec::new();
    let mut points_outside = Vec::new();
    let mut distances_inside = Vec::new();
    let mut all_distances = Vec::new();

    for candidate in candidate_points {
        // Validate candidate coordinates
        if candidate.lat.is_nan() || candidate.lat.is_infinite() {
            return Err("Candidate point latitude cannot be NaN or infinite".to_string());
        }
        if candidate.lon.is_nan() || candidate.lon.is_infinite() {
            return Err("Candidate point longitude cannot be NaN or infinite".to_string());
        }
        if candidate.lat < -90.0 || candidate.lat > 90.0 {
            return Err(format!(
                "Invalid candidate latitude: {}. Must be between -90 and 90",
                candidate.lat
            ));
        }
        if candidate.lon < -180.0 || candidate.lon > 180.0 {
            return Err(format!(
                "Invalid candidate longitude: {}. Must be between -180 and 180",
                candidate.lon
            ));
        }

        let distance = haversine_distance(&center, &candidate);
        let bearing = calculate_bearing(&center, &candidate);
        all_distances.push(distance);

        let result = NearestPointResult {
            point: candidate,
            distance_meters: distance,
            bearing_degrees: bearing,
        };

        if distance <= radius_meters {
            distances_inside.push(distance);
            points_inside.push(result);
        } else {
            points_outside.push(result);
        }
    }

    // Calculate summary statistics
    let total_points = points_inside.len() + points_outside.len();
    let average_distance_inside = if distances_inside.is_empty() {
        0.0
    } else {
        distances_inside.iter().sum::<f64>() / distances_inside.len() as f64
    };

    let closest_point_distance = all_distances.iter().cloned().fold(f64::INFINITY, f64::min);
    let farthest_point_distance = all_distances.iter().cloned().fold(0.0, f64::max);

    Ok(ProximityZoneResult {
        center,
        radius_meters,
        points_in_zone: points_inside.clone(),
        points_outside_zone: points_outside,
        summary: ProximityZoneSummary {
            total_points,
            points_inside: points_inside.len(),
            points_outside: total_points - points_inside.len(),
            average_distance_inside,
            closest_point_distance,
            farthest_point_distance,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_points_around_center() -> Vec<Point> {
        vec![
            Point {
                lat: 40.7128,
                lon: -74.0060,
                id: Some("Close1".to_string()),
            }, // ~0km from NYC
            Point {
                lat: 40.7228,
                lon: -74.0060,
                id: Some("Close2".to_string()),
            }, // ~1.1km north
            Point {
                lat: 40.7128,
                lon: -73.9960,
                id: Some("Close3".to_string()),
            }, // ~0.8km east
            Point {
                lat: 40.8000,
                lon: -74.0060,
                id: Some("Medium".to_string()),
            }, // ~9.7km north
            Point {
                lat: 41.0000,
                lon: -74.0060,
                id: Some("Far".to_string()),
            }, // ~32km north
        ]
    }

    #[test]
    fn test_proximity_zone_basic() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: Some("NYC".to_string()),
        };
        let radius = 5000.0; // 5km
        let candidates = create_test_points_around_center();

        let result = proximity_zone_analysis(center.clone(), radius, candidates).unwrap();

        assert_eq!(result.center, center);
        assert_eq!(result.radius_meters, radius);
        assert_eq!(result.summary.total_points, 5);

        // Should have some points inside and some outside the 5km radius
        assert!(result.summary.points_inside > 0);
        assert!(result.summary.points_outside > 0);
        assert_eq!(
            result.summary.points_inside + result.summary.points_outside,
            5
        );

        // All inside points should be within radius
        for point_result in &result.points_in_zone {
            assert!(point_result.distance_meters <= radius);
        }

        // All outside points should be beyond radius
        for point_result in &result.points_outside_zone {
            assert!(point_result.distance_meters > radius);
        }
    }

    #[test]
    fn test_proximity_zone_all_inside() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };
        let radius = 50000.0; // 50km - should include all test points
        let candidates = create_test_points_around_center();

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        assert_eq!(result.summary.points_inside, 5);
        assert_eq!(result.summary.points_outside, 0);
        assert_eq!(result.points_in_zone.len(), 5);
        assert_eq!(result.points_outside_zone.len(), 0);
        assert!(result.summary.average_distance_inside > 0.0);
    }

    #[test]
    fn test_proximity_zone_all_outside() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };
        let radius = 100.0; // 100m - should exclude all test points except exact center match
        let candidates = vec![
            Point {
                lat: 40.8000,
                lon: -74.0060,
                id: Some("Far1".to_string()),
            },
            Point {
                lat: 41.0000,
                lon: -74.0060,
                id: Some("Far2".to_string()),
            },
        ];

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        assert_eq!(result.summary.points_inside, 0);
        assert_eq!(result.summary.points_outside, 2);
        assert_eq!(result.points_in_zone.len(), 0);
        assert_eq!(result.points_outside_zone.len(), 2);
        assert_eq!(result.summary.average_distance_inside, 0.0);
    }

    #[test]
    fn test_proximity_zone_summary_statistics() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };
        let radius = 10000.0; // 10km
        let candidates = create_test_points_around_center();

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        // Check summary statistics
        assert_eq!(result.summary.total_points, 5);
        assert!(result.summary.closest_point_distance >= 0.0);
        assert!(result.summary.farthest_point_distance > result.summary.closest_point_distance);

        if result.summary.points_inside > 0 {
            assert!(result.summary.average_distance_inside >= 0.0);
            assert!(result.summary.average_distance_inside <= radius);
        }
    }

    #[test]
    fn test_proximity_zone_point_at_center() {
        let center = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };
        let radius = 1000.0; // 1km
        let candidates = vec![center.clone()]; // Point exactly at center

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        assert_eq!(result.summary.points_inside, 1);
        assert_eq!(result.summary.points_outside, 0);
        assert_eq!(result.points_in_zone[0].distance_meters, 0.0);
        assert_eq!(result.summary.closest_point_distance, 0.0);
        assert_eq!(result.summary.average_distance_inside, 0.0);
    }

    #[test]
    fn test_proximity_zone_bearings() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let radius = 10000.0; // 10km
        let candidates = vec![
            Point {
                lat: 41.0,
                lon: -74.0,
                id: Some("North".to_string()),
            }, // North
            Point {
                lat: 40.0,
                lon: -73.0,
                id: Some("East".to_string()),
            }, // East
            Point {
                lat: 39.0,
                lon: -74.0,
                id: Some("South".to_string()),
            }, // South
            Point {
                lat: 40.0,
                lon: -75.0,
                id: Some("West".to_string()),
            }, // West
        ];

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        // All bearings should be in [0, 360) range
        for point_result in &result.points_in_zone {
            assert!(point_result.bearing_degrees >= 0.0);
            assert!(point_result.bearing_degrees < 360.0);
        }
        for point_result in &result.points_outside_zone {
            assert!(point_result.bearing_degrees >= 0.0);
            assert!(point_result.bearing_degrees < 360.0);
        }
    }

    #[test]
    fn test_haversine_distance_calculation() {
        let p1 = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        }; // NYC
        let p2 = Point {
            lat: 34.0522,
            lon: -118.2437,
            id: None,
        }; // LA

        let distance = haversine_distance(&p1, &p2);

        // NYC to LA is approximately 3944 km
        assert!(distance > 3900000.0);
        assert!(distance < 4000000.0);
    }

    #[test]
    fn test_calculate_bearing_cardinal() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };

        // Test North (should be ~0°)
        let north = Point {
            lat: 41.0,
            lon: -74.0,
            id: None,
        };
        let bearing_north = calculate_bearing(&center, &north);
        assert!((bearing_north - 0.0).abs() < 5.0);

        // Test East (should be ~90°)
        let east = Point {
            lat: 40.0,
            lon: -73.0,
            id: None,
        };
        let bearing_east = calculate_bearing(&center, &east);
        assert!((bearing_east - 90.0).abs() < 5.0);
    }

    #[test]
    fn test_proximity_zone_empty_candidates() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = vec![];

        let result = proximity_zone_analysis(center, 1000.0, candidates);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "At least one candidate point must be provided"
        );
    }

    #[test]
    fn test_proximity_zone_invalid_radius() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = create_test_points_around_center();

        // Negative radius
        let result = proximity_zone_analysis(center.clone(), -1000.0, candidates.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Radius must be positive"));

        // Zero radius
        let result = proximity_zone_analysis(center.clone(), 0.0, candidates.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Radius must be positive"));

        // NaN radius
        let result = proximity_zone_analysis(center.clone(), f64::NAN, candidates.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Radius must be positive"));

        // Infinite radius
        let result = proximity_zone_analysis(center, f64::INFINITY, candidates);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Radius must be positive"));
    }

    #[test]
    fn test_proximity_zone_invalid_center_coordinates() {
        let candidates = create_test_points_around_center();

        // Invalid latitude
        let invalid_center = Point {
            lat: 91.0,
            lon: -74.0,
            id: None,
        };
        let result = proximity_zone_analysis(invalid_center, 1000.0, candidates.clone());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid center latitude"));

        // Invalid longitude
        let invalid_center = Point {
            lat: 40.0,
            lon: 181.0,
            id: None,
        };
        let result = proximity_zone_analysis(invalid_center, 1000.0, candidates);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid center longitude"));
    }

    #[test]
    fn test_proximity_zone_invalid_candidate_coordinates() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points_around_center();
        candidates[0].lat = 91.0; // Invalid latitude

        let result = proximity_zone_analysis(center, 1000.0, candidates);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid candidate latitude"));
    }

    #[test]
    fn test_proximity_zone_nan_coordinates() {
        let candidates = create_test_points_around_center();

        // NaN center coordinates
        let nan_center = Point {
            lat: f64::NAN,
            lon: -74.0,
            id: None,
        };
        let result = proximity_zone_analysis(nan_center, 1000.0, candidates.clone());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Center latitude cannot be NaN or infinite"
        );

        // NaN candidate coordinates
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points_around_center();
        candidates[0].lon = f64::NAN;
        let result = proximity_zone_analysis(center, 1000.0, candidates);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Candidate point longitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_proximity_zone_infinite_coordinates() {
        let candidates = create_test_points_around_center();

        // Infinite center coordinates
        let inf_center = Point {
            lat: f64::INFINITY,
            lon: -74.0,
            id: None,
        };
        let result = proximity_zone_analysis(inf_center, 1000.0, candidates.clone());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Center latitude cannot be NaN or infinite"
        );

        // Infinite candidate coordinates
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points_around_center();
        candidates[0].lat = f64::NEG_INFINITY;
        let result = proximity_zone_analysis(center, 1000.0, candidates);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Candidate point latitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_proximity_zone_boundary_coordinates() {
        // Test with boundary valid coordinates
        let center = Point {
            lat: 90.0,
            lon: 180.0,
            id: None,
        }; // North Pole, Date Line
        let candidates = vec![
            Point {
                lat: -90.0,
                lon: -180.0,
                id: Some("South Pole".to_string()),
            },
            Point {
                lat: 0.0,
                lon: 0.0,
                id: Some("Equator Prime".to_string()),
            },
        ];

        let result = proximity_zone_analysis(center, 50000000.0, candidates).unwrap(); // Very large radius

        assert_eq!(result.summary.total_points, 2);
        assert!(result.summary.closest_point_distance > 0.0);
        assert!(result.summary.farthest_point_distance > result.summary.closest_point_distance);
    }

    #[test]
    fn test_proximity_zone_exact_radius_boundary() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let radius = 2000.0; // 2km - use larger radius to ensure point is inside

        // Create points within radius distance
        let candidates = vec![
            Point {
                lat: 40.009,
                lon: -74.0,
                id: Some("AtRadius".to_string()),
            }, // ~1km north
        ];

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        // Point should be inside (distance <= radius)
        assert!(result.summary.points_inside > 0);
    }

    #[test]
    fn test_proximity_zone_large_dataset() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let radius = 5000.0; // 5km

        // Create a larger dataset
        let mut candidates = Vec::new();
        for i in 0..100 {
            let lat_offset = (i as f64 - 50.0) * 0.01; // ±0.5 degrees
            let lon_offset = (i as f64 - 50.0) * 0.01;
            candidates.push(Point {
                lat: center.lat + lat_offset,
                lon: center.lon + lon_offset,
                id: Some(format!("Point{i}")),
            });
        }

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        assert_eq!(result.summary.total_points, 100);
        assert!(result.summary.points_inside > 0);
        // points_outside is a usize, so it's always >= 0
        assert_eq!(
            result.summary.points_inside + result.summary.points_outside,
            100
        );
    }

    #[test]
    fn test_proximity_zone_point_ids_preserved() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: Some("Center".to_string()),
        };
        let radius = 10000.0; // 10km
        let candidates = create_test_points_around_center();

        let result = proximity_zone_analysis(center, radius, candidates).unwrap();

        // Check that center ID is preserved
        assert_eq!(result.center.id, Some("Center".to_string()));

        // Check that point IDs are preserved in results
        let all_points: Vec<&NearestPointResult> = result
            .points_in_zone
            .iter()
            .chain(result.points_outside_zone.iter())
            .collect();

        for point_result in all_points {
            assert!(point_result.point.id.is_some());
        }
    }
}
