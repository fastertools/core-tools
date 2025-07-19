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
pub struct NearestPointsInput {
    /// Point to search from
    pub query_point: Point,
    /// Points to search among
    pub candidate_points: Vec<Point>,
    /// Maximum number of results to return
    pub max_results: Option<usize>,
    /// Only return points within this distance (meters)
    pub max_distance_meters: Option<f64>,
}

#[derive(Serialize, Debug)]
pub struct NearestPointResult {
    pub point: Point,
    pub distance_meters: f64,
    pub bearing_degrees: f64,
}

#[derive(Serialize, Debug)]
pub struct NearestPointsResult {
    pub query_point: Point,
    pub nearest_points: Vec<NearestPointResult>,
    pub total_candidates: usize,
    pub results_returned: usize,
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

pub fn find_nearest_points(
    query_point: Point,
    candidate_points: Vec<Point>,
    max_results: Option<usize>,
    max_distance_meters: Option<f64>,
) -> Result<NearestPointsResult, String> {
    if candidate_points.is_empty() {
        return Err("At least one candidate point must be provided".to_string());
    }

    // Validate query point
    if query_point.lat.is_nan() || query_point.lat.is_infinite() {
        return Err("Query point latitude cannot be NaN or infinite".to_string());
    }
    if query_point.lon.is_nan() || query_point.lon.is_infinite() {
        return Err("Query point longitude cannot be NaN or infinite".to_string());
    }
    if query_point.lat < -90.0 || query_point.lat > 90.0 {
        return Err(format!(
            "Invalid query point latitude: {}. Must be between -90 and 90",
            query_point.lat
        ));
    }
    if query_point.lon < -180.0 || query_point.lon > 180.0 {
        return Err(format!(
            "Invalid query point longitude: {}. Must be between -180 and 180",
            query_point.lon
        ));
    }

    // Validate max_distance_meters
    if let Some(max_dist) = max_distance_meters {
        if max_dist < 0.0 || max_dist.is_nan() || max_dist.is_infinite() {
            return Err("Max distance must be positive and finite".to_string());
        }
    }

    let mut distances: Vec<(usize, f64)> = Vec::new();

    for (i, candidate) in candidate_points.iter().enumerate() {
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

        let distance = haversine_distance(&query_point, candidate);

        // Apply distance filter if specified
        if let Some(max_dist) = max_distance_meters {
            if distance <= max_dist {
                distances.push((i, distance));
            }
        } else {
            distances.push((i, distance));
        }
    }

    // Sort by distance
    distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // Apply result limit
    let max_results = max_results.unwrap_or(distances.len()).min(distances.len());

    let mut nearest_points = Vec::new();
    for &(idx, distance) in distances.iter().take(max_results) {
        let candidate = &candidate_points[idx];
        let bearing = calculate_bearing(&query_point, candidate);

        nearest_points.push(NearestPointResult {
            point: candidate.clone(),
            distance_meters: distance,
            bearing_degrees: bearing,
        });
    }

    Ok(NearestPointsResult {
        query_point,
        nearest_points,
        total_candidates: candidate_points.len(),
        results_returned: max_results,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_points() -> Vec<Point> {
        vec![
            Point {
                lat: 40.7128,
                lon: -74.0060,
                id: Some("NYC".to_string()),
            }, // New York
            Point {
                lat: 34.0522,
                lon: -118.2437,
                id: Some("LA".to_string()),
            }, // Los Angeles
            Point {
                lat: 41.8781,
                lon: -87.6298,
                id: Some("CHI".to_string()),
            }, // Chicago
            Point {
                lat: 29.7604,
                lon: -95.3698,
                id: Some("HOU".to_string()),
            }, // Houston
            Point {
                lat: 33.4484,
                lon: -112.0740,
                id: Some("PHX".to_string()),
            }, // Phoenix
        ]
    }

    #[test]
    fn test_proximity_search_basic() {
        let query_point = Point {
            lat: 40.7589,
            lon: -73.9851,
            id: Some("Times Square".to_string()),
        }; // Times Square
        let candidates = create_test_points();

        let result = find_nearest_points(query_point.clone(), candidates, None, None).unwrap();

        assert_eq!(result.query_point, query_point);
        assert_eq!(result.total_candidates, 5);
        assert_eq!(result.results_returned, 5);
        assert_eq!(result.nearest_points.len(), 5);

        // NYC should be closest to Times Square
        assert_eq!(result.nearest_points[0].point.id, Some("NYC".to_string()));

        // Distances should be in ascending order
        for i in 1..result.nearest_points.len() {
            assert!(
                result.nearest_points[i - 1].distance_meters
                    <= result.nearest_points[i].distance_meters
            );
        }
    }

    #[test]
    fn test_proximity_search_with_max_results() {
        let query_point = Point {
            lat: 40.7589,
            lon: -73.9851,
            id: None,
        };
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, Some(3), None).unwrap();

        assert_eq!(result.nearest_points.len(), 3);
        assert_eq!(result.results_returned, 3);
        assert_eq!(result.total_candidates, 5);
    }

    #[test]
    fn test_proximity_search_with_max_distance() {
        let query_point = Point {
            lat: 40.7589,
            lon: -73.9851,
            id: None,
        };
        let candidates = create_test_points();

        // Use small distance to filter out most points
        let result = find_nearest_points(query_point, candidates, None, Some(50000.0)).unwrap(); // 50km

        assert!(result.nearest_points.len() <= result.total_candidates);
        assert_eq!(result.total_candidates, 5);

        // All returned points should be within max distance
        for nearest in &result.nearest_points {
            assert!(nearest.distance_meters <= 50000.0);
        }
    }

    #[test]
    fn test_proximity_search_with_both_limits() {
        let query_point = Point {
            lat: 40.7589,
            lon: -73.9851,
            id: None,
        };
        let candidates = create_test_points();

        let result =
            find_nearest_points(query_point, candidates, Some(2), Some(1000000.0)).unwrap(); // 1000km

        assert!(result.nearest_points.len() <= 2);
        assert!(result.results_returned <= 2);

        // All returned points should be within max distance
        for nearest in &result.nearest_points {
            assert!(nearest.distance_meters <= 1000000.0);
        }
    }

    #[test]
    fn test_haversine_distance_known_values() {
        let nyc = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };
        let la = Point {
            lat: 34.0522,
            lon: -118.2437,
            id: None,
        };

        let distance = haversine_distance(&nyc, &la);

        // NYC to LA is approximately 3944 km
        assert!(distance > 3900000.0);
        assert!(distance < 4000000.0);
    }

    #[test]
    fn test_haversine_distance_same_point() {
        let point = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        };

        let distance = haversine_distance(&point, &point);

        assert_eq!(distance, 0.0);
    }

    #[test]
    fn test_calculate_bearing_cardinal_directions() {
        let center = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };

        // North
        let north = Point {
            lat: 41.0,
            lon: -74.0,
            id: None,
        };
        let bearing_north = calculate_bearing(&center, &north);
        assert!((bearing_north - 0.0).abs() < 1.0); // Should be close to 0° (North)

        // East
        let east = Point {
            lat: 40.0,
            lon: -73.0,
            id: None,
        };
        let bearing_east = calculate_bearing(&center, &east);
        assert!((bearing_east - 90.0).abs() < 1.0); // Should be close to 90° (East)

        // South
        let south = Point {
            lat: 39.0,
            lon: -74.0,
            id: None,
        };
        let bearing_south = calculate_bearing(&center, &south);
        assert!((bearing_south - 180.0).abs() < 1.0); // Should be close to 180° (South)

        // West
        let west = Point {
            lat: 40.0,
            lon: -75.0,
            id: None,
        };
        let bearing_west = calculate_bearing(&center, &west);
        assert!((bearing_west - 270.0).abs() < 1.0); // Should be close to 270° (West)
    }

    #[test]
    fn test_calculate_bearing_same_point() {
        let point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };

        let bearing = calculate_bearing(&point, &point);

        // Bearing to same point should be 0 (though mathematically undefined)
        assert!(bearing.is_finite());
    }

    #[test]
    fn test_proximity_search_empty_candidates() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = vec![];

        let result = find_nearest_points(query_point, candidates, None, None);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "At least one candidate point must be provided"
        );
    }

    #[test]
    fn test_proximity_search_invalid_query_coordinates() {
        let candidates = create_test_points();

        // Invalid latitude
        let invalid_query = Point {
            lat: 91.0,
            lon: -74.0,
            id: None,
        };
        let result = find_nearest_points(invalid_query, candidates.clone(), None, None);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid query point latitude"));

        // Invalid longitude
        let invalid_query = Point {
            lat: 40.0,
            lon: 181.0,
            id: None,
        };
        let result = find_nearest_points(invalid_query, candidates, None, None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("Invalid query point longitude")
        );
    }

    #[test]
    fn test_proximity_search_invalid_candidate_coordinates() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points();
        candidates[0].lat = 91.0; // Invalid latitude

        let result = find_nearest_points(query_point, candidates, None, None);

        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid candidate latitude"));
    }

    #[test]
    fn test_proximity_search_nan_coordinates() {
        let candidates = create_test_points();

        // NaN query point
        let nan_query = Point {
            lat: f64::NAN,
            lon: -74.0,
            id: None,
        };
        let result = find_nearest_points(nan_query, candidates.clone(), None, None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Query point latitude cannot be NaN or infinite"
        );

        // NaN candidate point
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points();
        candidates[0].lon = f64::NAN;
        let result = find_nearest_points(query_point, candidates, None, None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Candidate point longitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_proximity_search_infinite_coordinates() {
        let candidates = create_test_points();

        // Infinite query point
        let inf_query = Point {
            lat: f64::INFINITY,
            lon: -74.0,
            id: None,
        };
        let result = find_nearest_points(inf_query, candidates.clone(), None, None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Query point latitude cannot be NaN or infinite"
        );

        // Infinite candidate point
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let mut candidates = create_test_points();
        candidates[0].lat = f64::NEG_INFINITY;
        let result = find_nearest_points(query_point, candidates, None, None);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Candidate point latitude cannot be NaN or infinite"
        );
    }

    #[test]
    fn test_proximity_search_invalid_max_distance() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = create_test_points();

        // Negative max distance
        let result =
            find_nearest_points(query_point.clone(), candidates.clone(), None, Some(-1000.0));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Max distance must be positive and finite"
        );

        // NaN max distance
        let result = find_nearest_points(
            query_point.clone(),
            candidates.clone(),
            None,
            Some(f64::NAN),
        );
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Max distance must be positive and finite"
        );

        // Infinite max distance
        let result = find_nearest_points(query_point, candidates, None, Some(f64::INFINITY));
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Max distance must be positive and finite"
        );
    }

    #[test]
    fn test_proximity_search_boundary_coordinates() {
        // Test with boundary valid coordinates
        let query_point = Point {
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

        let result = find_nearest_points(query_point, candidates, None, None).unwrap();

        assert_eq!(result.nearest_points.len(), 2);
        assert!(result.nearest_points[0].distance_meters > 0.0);
        assert!(result.nearest_points[1].distance_meters > 0.0);
    }

    #[test]
    fn test_proximity_search_zero_max_results() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, Some(0), None).unwrap();

        assert_eq!(result.nearest_points.len(), 0);
        assert_eq!(result.results_returned, 0);
        assert_eq!(result.total_candidates, 5);
    }

    #[test]
    fn test_proximity_search_large_max_results() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, Some(100), None).unwrap();

        // Should return all available candidates
        assert_eq!(result.nearest_points.len(), 5);
        assert_eq!(result.results_returned, 5);
        assert_eq!(result.total_candidates, 5);
    }

    #[test]
    fn test_proximity_search_very_small_max_distance() {
        let query_point = Point {
            lat: 40.7128,
            lon: -74.0060,
            id: None,
        }; // NYC
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, None, Some(1.0)).unwrap(); // 1 meter

        // Should match only the NYC point (distance to itself is 0)
        assert_eq!(result.nearest_points.len(), 1);
        assert_eq!(result.nearest_points[0].point.id, Some("NYC".to_string()));
        assert_eq!(result.nearest_points[0].distance_meters, 0.0);
    }

    #[test]
    fn test_proximity_search_bearing_consistency() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: None,
        };
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, None, None).unwrap();

        // All bearings should be in [0, 360) range
        for nearest in &result.nearest_points {
            assert!(nearest.bearing_degrees >= 0.0);
            assert!(nearest.bearing_degrees < 360.0);
        }
    }

    #[test]
    fn test_proximity_search_point_ids() {
        let query_point = Point {
            lat: 40.0,
            lon: -74.0,
            id: Some("Query".to_string()),
        };
        let candidates = create_test_points();

        let result = find_nearest_points(query_point, candidates, None, None).unwrap();

        // Check that IDs are preserved
        assert_eq!(result.query_point.id, Some("Query".to_string()));

        // Check that candidate IDs are preserved
        let ids: Vec<&Option<String>> = result.nearest_points.iter().map(|n| &n.point.id).collect();
        assert!(ids.contains(&&Some("NYC".to_string())));
        assert!(ids.contains(&&Some("LA".to_string())));
    }

    #[test]
    fn test_proximity_search_crossing_date_line() {
        let query_point = Point {
            lat: 0.0,
            lon: 179.0,
            id: None,
        }; // Near date line
        let candidates = vec![
            Point {
                lat: 0.0,
                lon: -179.0,
                id: Some("West of date line".to_string()),
            },
            Point {
                lat: 0.0,
                lon: 178.0,
                id: Some("East of query".to_string()),
            },
        ];

        let result = find_nearest_points(query_point, candidates, None, None).unwrap();

        assert_eq!(result.nearest_points.len(), 2);

        // Both points should have reasonable distances and bearings
        for nearest in &result.nearest_points {
            assert!(nearest.distance_meters > 0.0);
            assert!(nearest.distance_meters < 500000.0); // Should be less than 500km
            assert!(nearest.bearing_degrees >= 0.0);
            assert!(nearest.bearing_degrees < 360.0);
        }
    }
}
