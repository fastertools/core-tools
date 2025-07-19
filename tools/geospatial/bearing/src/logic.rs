use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingInput {
    pub lat1: f64,
    pub lon1: f64,
    pub lat2: f64,
    pub lon2: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BearingResult {
    pub bearing_degrees: f64,
    pub bearing_radians: f64,
    pub compass_direction: String,
}

pub fn calculate_bearing_between_points(input: BearingInput) -> Result<BearingResult, String> {
    // Validate input - check for invalid values
    if input.lat1.is_nan()
        || input.lat1.is_infinite()
        || input.lon1.is_nan()
        || input.lon1.is_infinite()
        || input.lat2.is_nan()
        || input.lat2.is_infinite()
        || input.lon2.is_nan()
        || input.lon2.is_infinite()
    {
        return Err("Input contains invalid values (NaN or Infinite)".to_string());
    }

    // Validate latitude range
    if input.lat1 < -90.0 || input.lat1 > 90.0 || input.lat2 < -90.0 || input.lat2 > 90.0 {
        return Err("Latitude must be between -90 and 90 degrees".to_string());
    }

    // Validate longitude range
    if input.lon1 < -180.0 || input.lon1 > 180.0 || input.lon2 < -180.0 || input.lon2 > 180.0 {
        return Err("Longitude must be between -180 and 180 degrees".to_string());
    }

    let bearing_deg = calculate_bearing(input.lat1, input.lon1, input.lat2, input.lon2);
    let bearing_rad = bearing_deg * PI / 180.0;
    let compass = degrees_to_compass(bearing_deg);

    Ok(BearingResult {
        bearing_degrees: bearing_deg,
        bearing_radians: bearing_rad,
        compass_direction: compass,
    })
}

fn calculate_bearing(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let lat1_rad = lat1 * PI / 180.0;
    let lat2_rad = lat2 * PI / 180.0;
    let delta_lon = (lon2 - lon1) * PI / 180.0;

    let y = delta_lon.sin() * lat2_rad.cos();
    let x = lat1_rad.cos() * lat2_rad.sin() - lat1_rad.sin() * lat2_rad.cos() * delta_lon.cos();

    let bearing_rad = y.atan2(x);

    (bearing_rad * 180.0 / PI + 360.0) % 360.0
}

fn degrees_to_compass(degrees: f64) -> String {
    let directions = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];

    let index = ((degrees + 11.25) / 22.5) as usize % 16;
    directions[index].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_north_bearing() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 0.0,
            lat2: 1.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 0.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "N");
    }

    #[test]
    fn test_east_bearing() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 0.0,
            lat2: 0.0,
            lon2: 1.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 90.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "E");
    }

    #[test]
    fn test_south_bearing() {
        let input = BearingInput {
            lat1: 1.0,
            lon1: 0.0,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 180.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "S");
    }

    #[test]
    fn test_west_bearing() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 1.0,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 270.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "W");
    }

    #[test]
    fn test_northeast_bearing() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 0.0,
            lat2: 1.0,
            lon2: 1.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!(result.bearing_degrees > 0.0 && result.bearing_degrees < 90.0);
        assert!(result.compass_direction.contains("N") && result.compass_direction.contains("E"));
    }

    #[test]
    fn test_same_point() {
        let input = BearingInput {
            lat1: 45.0,
            lon1: -122.0,
            lat2: 45.0,
            lon2: -122.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        // Bearing from a point to itself should be 0 (North)
        assert!((result.bearing_degrees - 0.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "N");
    }

    #[test]
    fn test_radians_conversion() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 0.0,
            lat2: 0.0,
            lon2: 1.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_radians - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_compass_directions() {
        // Test that compass directions map correctly
        assert_eq!(degrees_to_compass(0.0), "N");
        assert_eq!(degrees_to_compass(22.5), "NNE");
        assert_eq!(degrees_to_compass(45.0), "NE");
        assert_eq!(degrees_to_compass(90.0), "E");
        assert_eq!(degrees_to_compass(135.0), "SE");
        assert_eq!(degrees_to_compass(180.0), "S");
        assert_eq!(degrees_to_compass(225.0), "SW");
        assert_eq!(degrees_to_compass(270.0), "W");
        assert_eq!(degrees_to_compass(315.0), "NW");
        assert_eq!(degrees_to_compass(360.0), "N");
    }

    #[test]
    fn test_invalid_latitude() {
        let input = BearingInput {
            lat1: 91.0,
            lon1: 0.0,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Latitude must be between -90 and 90 degrees"
        );
    }

    #[test]
    fn test_invalid_longitude() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: 181.0,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Longitude must be between -180 and 180 degrees"
        );
    }

    #[test]
    fn test_nan_input_error() {
        let input = BearingInput {
            lat1: f64::NAN,
            lon1: 0.0,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_infinite_input_error() {
        let input = BearingInput {
            lat1: 0.0,
            lon1: f64::INFINITY,
            lat2: 0.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Input contains invalid values (NaN or Infinite)"
        );
    }

    #[test]
    fn test_real_world_coordinates() {
        // New York to Los Angeles
        let input = BearingInput {
            lat1: 40.7128,
            lon1: -74.0060, // NYC
            lat2: 34.0522,
            lon2: -118.2437, // LA
        };
        let result = calculate_bearing_between_points(input).unwrap();
        // Should be westward bearing (between 180 and 360 degrees)
        assert!(result.bearing_degrees > 180.0 && result.bearing_degrees < 360.0);
        assert!(result.compass_direction.contains("W") || result.compass_direction.contains("S"));
    }

    #[test]
    fn test_pole_to_pole() {
        // North pole to south pole
        let input = BearingInput {
            lat1: 90.0,
            lon1: 0.0,
            lat2: -90.0,
            lon2: 0.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 180.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "S");
    }

    #[test]
    fn test_cross_dateline() {
        // Test crossing the international date line
        let input = BearingInput {
            lat1: 0.0,
            lon1: 179.0,
            lat2: 0.0,
            lon2: -179.0,
        };
        let result = calculate_bearing_between_points(input).unwrap();
        assert!((result.bearing_degrees - 90.0).abs() < 1e-10);
        assert_eq!(result.compass_direction, "E");
    }
}
