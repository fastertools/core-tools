use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DecimalDegreesInput {
    /// Latitude in decimal degrees
    pub latitude: f64,
    /// Longitude in decimal degrees
    pub longitude: f64,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct DMSCoordinate {
    pub degrees: i32,
    pub minutes: i32,
    pub seconds: f64,
    pub direction: String,
}

#[derive(Serialize, Debug)]
pub struct DMSResult {
    pub latitude: DMSCoordinate,
    pub longitude: DMSCoordinate,
}

pub fn decimal_to_dms(decimal: f64, is_latitude: bool) -> DMSCoordinate {
    let abs_decimal = decimal.abs();
    let degrees = abs_decimal.floor() as i32;
    let minutes_float = (abs_decimal - degrees as f64) * 60.0;
    let minutes = minutes_float.floor() as i32;
    let seconds = (minutes_float - minutes as f64) * 60.0;

    let direction = if is_latitude {
        if decimal >= 0.0 {
            "N".to_string()
        } else {
            "S".to_string()
        }
    } else if decimal >= 0.0 {
        "E".to_string()
    } else {
        "W".to_string()
    };

    DMSCoordinate {
        degrees,
        minutes,
        seconds,
        direction,
    }
}

pub fn convert_to_dms(latitude: f64, longitude: f64) -> Result<DMSResult, String> {
    if latitude.is_nan() || latitude.is_infinite() {
        return Err("Latitude cannot be NaN or infinite".to_string());
    }
    if longitude.is_nan() || longitude.is_infinite() {
        return Err("Longitude cannot be NaN or infinite".to_string());
    }

    if latitude < -90.0 || latitude > 90.0 {
        return Err("Latitude must be between -90 and 90".to_string());
    }
    if longitude < -180.0 || longitude > 180.0 {
        return Err("Longitude must be between -180 and 180".to_string());
    }

    Ok(DMSResult {
        latitude: decimal_to_dms(latitude, true),
        longitude: decimal_to_dms(longitude, false),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_to_dms_basic() {
        let result = convert_to_dms(40.7128, -74.0060).unwrap();

        // Check latitude (40°42'46.08"N)
        assert_eq!(result.latitude.degrees, 40);
        assert_eq!(result.latitude.minutes, 42);
        assert!((result.latitude.seconds - 46.08).abs() < 0.01);
        assert_eq!(result.latitude.direction, "N");

        // Check longitude (74°0'21.6"W)
        assert_eq!(result.longitude.degrees, 74);
        assert_eq!(result.longitude.minutes, 0);
        assert!((result.longitude.seconds - 21.6).abs() < 0.01);
        assert_eq!(result.longitude.direction, "W");
    }

    #[test]
    fn test_convert_to_dms_zero_coordinates() {
        let result = convert_to_dms(0.0, 0.0).unwrap();

        assert_eq!(result.latitude.degrees, 0);
        assert_eq!(result.latitude.minutes, 0);
        assert_eq!(result.latitude.seconds, 0.0);
        assert_eq!(result.latitude.direction, "N");

        assert_eq!(result.longitude.degrees, 0);
        assert_eq!(result.longitude.minutes, 0);
        assert_eq!(result.longitude.seconds, 0.0);
        assert_eq!(result.longitude.direction, "E");
    }

    #[test]
    fn test_convert_to_dms_negative_coordinates() {
        let result = convert_to_dms(-33.8688, -151.2093).unwrap(); // Sydney

        // Check latitude (33°52'7.68"S)
        assert_eq!(result.latitude.degrees, 33);
        assert_eq!(result.latitude.minutes, 52);
        assert!((result.latitude.seconds - 7.68).abs() < 0.01);
        assert_eq!(result.latitude.direction, "S");

        // Check longitude (151°12'33.48"W)
        assert_eq!(result.longitude.degrees, 151);
        assert_eq!(result.longitude.minutes, 12);
        assert!((result.longitude.seconds - 33.48).abs() < 0.01);
        assert_eq!(result.longitude.direction, "W");
    }

    #[test]
    fn test_convert_to_dms_boundary_coordinates() {
        // Test North Pole
        let result = convert_to_dms(90.0, 0.0).unwrap();
        assert_eq!(result.latitude.degrees, 90);
        assert_eq!(result.latitude.direction, "N");

        // Test South Pole
        let result = convert_to_dms(-90.0, 0.0).unwrap();
        assert_eq!(result.latitude.degrees, 90);
        assert_eq!(result.latitude.direction, "S");

        // Test Date Line
        let result = convert_to_dms(0.0, 180.0).unwrap();
        assert_eq!(result.longitude.degrees, 180);
        assert_eq!(result.longitude.direction, "E");

        let result = convert_to_dms(0.0, -180.0).unwrap();
        assert_eq!(result.longitude.degrees, 180);
        assert_eq!(result.longitude.direction, "W");
    }

    #[test]
    fn test_convert_to_dms_precise_coordinates() {
        let result = convert_to_dms(51.4778, -0.0014).unwrap(); // London

        // Check precise conversion
        assert_eq!(result.latitude.degrees, 51);
        assert_eq!(result.latitude.minutes, 28);
        assert!((result.latitude.seconds - 40.08).abs() < 0.01);
        assert_eq!(result.latitude.direction, "N");

        assert_eq!(result.longitude.degrees, 0);
        assert_eq!(result.longitude.minutes, 0);
        assert!((result.longitude.seconds - 5.04).abs() < 0.01);
        assert_eq!(result.longitude.direction, "W");
    }

    #[test]
    fn test_decimal_to_dms_latitude_north() {
        let dms = decimal_to_dms(45.5, true);

        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);
        assert_eq!(dms.seconds, 0.0);
        assert_eq!(dms.direction, "N");
    }

    #[test]
    fn test_decimal_to_dms_latitude_south() {
        let dms = decimal_to_dms(-45.5, true);

        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);
        assert_eq!(dms.seconds, 0.0);
        assert_eq!(dms.direction, "S");
    }

    #[test]
    fn test_decimal_to_dms_longitude_east() {
        let dms = decimal_to_dms(123.25, false);

        assert_eq!(dms.degrees, 123);
        assert_eq!(dms.minutes, 15);
        assert_eq!(dms.seconds, 0.0);
        assert_eq!(dms.direction, "E");
    }

    #[test]
    fn test_decimal_to_dms_longitude_west() {
        let dms = decimal_to_dms(-123.25, false);

        assert_eq!(dms.degrees, 123);
        assert_eq!(dms.minutes, 15);
        assert_eq!(dms.seconds, 0.0);
        assert_eq!(dms.direction, "W");
    }

    #[test]
    fn test_decimal_to_dms_complex_seconds() {
        let dms = decimal_to_dms(40.446195, true);

        assert_eq!(dms.degrees, 40);
        assert_eq!(dms.minutes, 26);
        assert!((dms.seconds - 46.302).abs() < 0.01);
        assert_eq!(dms.direction, "N");
    }

    #[test]
    fn test_convert_to_dms_invalid_latitude() {
        let result = convert_to_dms(91.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Latitude must be between -90 and 90");

        let result = convert_to_dms(-91.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Latitude must be between -90 and 90");
    }

    #[test]
    fn test_convert_to_dms_invalid_longitude() {
        let result = convert_to_dms(0.0, 181.0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Longitude must be between -180 and 180"
        );

        let result = convert_to_dms(0.0, -181.0);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Longitude must be between -180 and 180"
        );
    }

    #[test]
    fn test_convert_to_dms_nan_coordinates() {
        let result = convert_to_dms(f64::NAN, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Latitude cannot be NaN or infinite");

        let result = convert_to_dms(0.0, f64::NAN);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Longitude cannot be NaN or infinite");
    }

    #[test]
    fn test_convert_to_dms_infinite_coordinates() {
        let result = convert_to_dms(f64::INFINITY, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Latitude cannot be NaN or infinite");

        let result = convert_to_dms(0.0, f64::NEG_INFINITY);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Longitude cannot be NaN or infinite");
    }

    #[test]
    fn test_convert_to_dms_very_small_values() {
        let result = convert_to_dms(0.000001, 0.000001).unwrap();

        assert_eq!(result.latitude.degrees, 0);
        assert_eq!(result.latitude.minutes, 0);
        assert!((result.latitude.seconds - 0.0036).abs() < 0.0001);
        assert_eq!(result.latitude.direction, "N");

        assert_eq!(result.longitude.degrees, 0);
        assert_eq!(result.longitude.minutes, 0);
        assert!((result.longitude.seconds - 0.0036).abs() < 0.0001);
        assert_eq!(result.longitude.direction, "E");
    }

    #[test]
    fn test_convert_to_dms_edge_case_minutes_boundary() {
        // Test a value that produces many seconds
        let result = convert_to_dms(40.999722, 0.0).unwrap();

        assert_eq!(result.latitude.degrees, 40);
        assert_eq!(result.latitude.minutes, 59);
        // Allow for floating point precision differences
        assert!(result.latitude.seconds >= 58.0 && result.latitude.seconds <= 60.0);
        assert_eq!(result.latitude.direction, "N");
    }
}
