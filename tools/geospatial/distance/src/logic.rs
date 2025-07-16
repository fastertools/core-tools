use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceInput {
    pub lat1: f64,
    pub lon1: f64,
    pub lat2: f64,
    pub lon2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceResult {
    pub distance_km: f64,
    pub distance_miles: f64,
    pub distance_nautical_miles: f64,
}

pub fn calculate_distance_between_points(input: DistanceInput) -> Result<DistanceResult, String> {
    // Validate input - check for invalid values
    if input.lat1.is_nan() || input.lat1.is_infinite() ||
       input.lon1.is_nan() || input.lon1.is_infinite() ||
       input.lat2.is_nan() || input.lat2.is_infinite() ||
       input.lon2.is_nan() || input.lon2.is_infinite() {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }
    
    // Validate latitude range
    if input.lat1 < -90.0 || input.lat1 > 90.0 ||
       input.lat2 < -90.0 || input.lat2 > 90.0 {
        return Err("Latitude must be between -90 and 90 degrees".to_string());
    }
    
    // Validate longitude range  
    if input.lon1 < -180.0 || input.lon1 > 180.0 ||
       input.lon2 < -180.0 || input.lon2 > 180.0 {
        return Err("Longitude must be between -180 and 180 degrees".to_string());
    }
    
    let distance_km = haversine_distance(input.lat1, input.lon1, input.lat2, input.lon2);
    
    Ok(DistanceResult {
        distance_km,
        distance_miles: distance_km * 0.621371,
        distance_nautical_miles: distance_km * 0.539957,
    })
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const EARTH_RADIUS_KM: f64 = 6371.0;
    
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let delta_lat = (lat2 - lat1) * PI / 180.0;
    let delta_lon = (lon2 - lon1) * PI / 180.0;
    
    let a = (delta_lat / 2.0).sin().powi(2) + 
            lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    
    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
    
    EARTH_RADIUS_KM * c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_point() {
        let input = DistanceInput {
            lat1: 40.7128, lon1: -74.0060,
            lat2: 40.7128, lon2: -74.0060,
        };
        let result = calculate_distance_between_points(input).unwrap();
        assert_eq!(result.distance_km, 0.0);
        assert_eq!(result.distance_miles, 0.0);
        assert_eq!(result.distance_nautical_miles, 0.0);
    }

    #[test]
    fn test_equator_distance() {
        // 1 degree longitude at equator ≈ 111.32 km
        let input = DistanceInput {
            lat1: 0.0, lon1: 0.0,
            lat2: 0.0, lon2: 1.0,
        };
        let result = calculate_distance_between_points(input).unwrap();
        assert!((result.distance_km - 111.32).abs() < 1.0);
    }

    #[test]
    fn test_new_york_to_london() {
        let input = DistanceInput {
            lat1: 40.7128, lon1: -74.0060,  // NYC
            lat2: 51.5074, lon2: -0.1278,   // London
        };
        let result = calculate_distance_between_points(input).unwrap();
        // Distance should be approximately 5585 km
        assert!((result.distance_km - 5585.0).abs() < 50.0);
        // Check conversions
        assert!(result.distance_miles > 3000.0 && result.distance_miles < 4000.0);
        assert!(result.distance_nautical_miles > 2500.0 && result.distance_nautical_miles < 3500.0);
    }

    #[test]
    fn test_north_south_distance() {
        // 1 degree latitude ≈ 111.32 km everywhere
        let input = DistanceInput {
            lat1: 0.0, lon1: 0.0,
            lat2: 1.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input).unwrap();
        assert!((result.distance_km - 111.32).abs() < 1.0);
    }

    #[test]
    fn test_pole_to_pole() {
        // North pole to south pole (half circumference)
        let input = DistanceInput {
            lat1: 90.0, lon1: 0.0,
            lat2: -90.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input).unwrap();
        // Should be approximately 20015 km (half Earth's circumference)
        assert!((result.distance_km - 20015.0).abs() < 100.0);
    }

    #[test]
    fn test_cross_dateline() {
        // Test crossing the international date line
        let input = DistanceInput {
            lat1: 0.0, lon1: 179.0,
            lat2: 0.0, lon2: -179.0,
        };
        let result = calculate_distance_between_points(input).unwrap();
        // Should be about 2 degrees longitude distance ≈ 222.6 km
        assert!((result.distance_km - 222.6).abs() < 10.0);
    }

    #[test]
    fn test_southern_hemisphere() {
        // Sydney to Cape Town
        let input = DistanceInput {
            lat1: -33.8688, lon1: 151.2093,  // Sydney
            lat2: -33.9249, lon2: 18.4241,   // Cape Town
        };
        let result = calculate_distance_between_points(input).unwrap();
        // Distance should be approximately 11000+ km
        assert!(result.distance_km > 10000.0);
        assert!(result.distance_km < 12000.0);
    }

    #[test]
    fn test_unit_conversions() {
        let input = DistanceInput {
            lat1: 40.7128, lon1: -74.0060,  // NYC
            lat2: 51.5074, lon2: -0.1278,   // London
        };
        let result = calculate_distance_between_points(input).unwrap();
        
        // Verify conversion factors
        let expected_miles = result.distance_km * 0.621371;
        let expected_nautical = result.distance_km * 0.539957;
        
        assert!((result.distance_miles - expected_miles).abs() < 0.001);
        assert!((result.distance_nautical_miles - expected_nautical).abs() < 0.001);
    }

    #[test]
    fn test_invalid_latitude() {
        let input = DistanceInput {
            lat1: 91.0, lon1: 0.0,
            lat2: 0.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Latitude must be between -90 and 90 degrees");
    }

    #[test]
    fn test_invalid_longitude() {
        let input = DistanceInput {
            lat1: 0.0, lon1: 181.0,
            lat2: 0.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Longitude must be between -180 and 180 degrees");
    }

    #[test]
    fn test_nan_input_error() {
        let input = DistanceInput {
            lat1: f64::NAN, lon1: 0.0,
            lat2: 0.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_infinite_input_error() {
        let input = DistanceInput {
            lat1: 0.0, lon1: f64::INFINITY,
            lat2: 0.0, lon2: 0.0,
        };
        let result = calculate_distance_between_points(input);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Input contains invalid values (NaN or Infinite)");
    }

    #[test]
    fn test_very_small_distance() {
        // Points very close together
        let input = DistanceInput {
            lat1: 40.7128, lon1: -74.0060,
            lat2: 40.7129, lon2: -74.0061,
        };
        let result = calculate_distance_between_points(input).unwrap();
        assert!(result.distance_km > 0.0);
        assert!(result.distance_km < 0.2);  // Should be less than 200m
    }

    #[test]
    fn test_maximum_distance() {
        // Antipodal points (maximum possible distance on sphere)
        let input = DistanceInput {
            lat1: 0.0, lon1: 0.0,
            lat2: 0.0, lon2: 180.0,
        };
        let result = calculate_distance_between_points(input).unwrap();
        // Should be approximately half Earth's circumference at equator
        assert!((result.distance_km - 20015.0).abs() < 100.0);
    }
}