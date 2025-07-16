use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct Coordinate {
    /// Latitude in decimal degrees
    pub lat: f64,
    /// Longitude in decimal degrees
    pub lon: f64,
}

#[derive(Deserialize)]
pub struct PolygonInput {
    /// Array of coordinates defining the polygon
    pub coordinates: Vec<Coordinate>,
}

#[derive(Serialize, Debug)]
pub struct PolygonAreaResult {
    pub area_square_meters: f64,
    pub area_square_kilometers: f64,
    pub area_square_miles: f64,
    pub area_hectares: f64,
    pub area_acres: f64,
}

pub fn calculate_polygon_area(coordinates: &[Coordinate]) -> Result<f64, String> {
    if coordinates.len() < 3 {
        return Err("Polygon must have at least 3 coordinates".to_string());
    }
    
    const EARTH_RADIUS_M: f64 = 6378137.0; // WGS84 equatorial radius in meters
    
    let mut area = 0.0;
    let n = coordinates.len();
    
    for i in 0..n {
        let j = (i + 1) % n;
        let lat1 = coordinates[i].lat * PI / 180.0;
        let lat2 = coordinates[j].lat * PI / 180.0;
        let lon1 = coordinates[i].lon * PI / 180.0;
        let lon2 = coordinates[j].lon * PI / 180.0;
        
        area += (lon2 - lon1) * (2.0 + lat1.sin() + lat2.sin());
    }
    
    area = area.abs() * EARTH_RADIUS_M * EARTH_RADIUS_M / 2.0;
    
    Ok(area)
}

pub fn get_polygon_area(coordinates: Vec<Coordinate>) -> Result<PolygonAreaResult, String> {
    // Validate coordinates
    for coord in &coordinates {
        if coord.lat.is_nan() || coord.lat.is_infinite() {
            return Err("Coordinate latitude cannot be NaN or infinite".to_string());
        }
        if coord.lon.is_nan() || coord.lon.is_infinite() {
            return Err("Coordinate longitude cannot be NaN or infinite".to_string());
        }
        if coord.lat < -90.0 || coord.lat > 90.0 {
            return Err(format!("Invalid latitude: {}. Must be between -90 and 90", coord.lat));
        }
        if coord.lon < -180.0 || coord.lon > 180.0 {
            return Err(format!("Invalid longitude: {}. Must be between -180 and 180", coord.lon));
        }
    }
    
    let area_m2 = calculate_polygon_area(&coordinates)?;
    
    Ok(PolygonAreaResult {
        area_square_meters: area_m2,
        area_square_kilometers: area_m2 / 1_000_000.0,
        area_square_miles: area_m2 / 2_589_988.11,
        area_hectares: area_m2 / 10_000.0,
        area_acres: area_m2 / 4_046.86,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_unit_square() -> Vec<Coordinate> {
        vec![
            Coordinate { lat: 0.0, lon: 0.0 },
            Coordinate { lat: 0.0, lon: 1.0 },
            Coordinate { lat: 1.0, lon: 1.0 },
            Coordinate { lat: 1.0, lon: 0.0 },
        ]
    }

    fn create_triangle() -> Vec<Coordinate> {
        vec![
            Coordinate { lat: 0.0, lon: 0.0 },
            Coordinate { lat: 1.0, lon: 0.0 },
            Coordinate { lat: 0.5, lon: 1.0 },
        ]
    }

    #[test]
    fn test_polygon_area_basic_square() {
        let square = create_unit_square();
        let result = get_polygon_area(square).unwrap();
        
        // Should be approximately the area of a 1°x1° square at equator
        assert!(result.area_square_meters > 10_000_000_000.0); // > 10 billion m²
        assert!(result.area_square_meters < 15_000_000_000.0); // < 15 billion m²
        
        // Verify unit conversions
        assert!((result.area_square_kilometers - result.area_square_meters / 1_000_000.0).abs() < 1.0);
        assert!((result.area_hectares - result.area_square_meters / 10_000.0).abs() < 1.0);
        assert!((result.area_acres - result.area_square_meters / 4_046.86).abs() < 1.0);
        assert!((result.area_square_miles - result.area_square_meters / 2_589_988.11).abs() < 1.0);
    }

    #[test]
    fn test_polygon_area_triangle() {
        let triangle = create_triangle();
        let result = get_polygon_area(triangle).unwrap();
        
        // Triangle should have roughly half the area of the unit square
        assert!(result.area_square_meters > 5_000_000_000.0);
        assert!(result.area_square_meters < 8_000_000_000.0);
        
        // All conversions should be positive
        assert!(result.area_square_kilometers > 0.0);
        assert!(result.area_square_miles > 0.0);
        assert!(result.area_hectares > 0.0);
        assert!(result.area_acres > 0.0);
    }

    #[test]
    fn test_polygon_area_small_polygon() {
        // Very small polygon (100m x 100m approximately)
        let small_polygon = vec![
            Coordinate { lat: 40.7128, lon: -74.0060 }, // NYC
            Coordinate { lat: 40.7128, lon: -74.0050 },
            Coordinate { lat: 40.7138, lon: -74.0050 },
            Coordinate { lat: 40.7138, lon: -74.0060 },
        ];
        
        let result = get_polygon_area(small_polygon).unwrap();
        
        // Should be roughly 10,000 square meters (1 hectare)
        assert!(result.area_square_meters > 5_000.0);
        assert!(result.area_square_meters < 20_000.0);
        assert!((result.area_hectares - 1.0).abs() < 1.0);
    }

    #[test]
    fn test_polygon_area_large_polygon() {
        // Large polygon covering several degrees
        let large_polygon = vec![
            Coordinate { lat: 40.0, lon: -75.0 },
            Coordinate { lat: 40.0, lon: -70.0 },
            Coordinate { lat: 45.0, lon: -70.0 },
            Coordinate { lat: 45.0, lon: -75.0 },
        ];
        
        let result = get_polygon_area(large_polygon).unwrap();
        
        // Should be a very large area
        assert!(result.area_square_meters > 100_000_000_000.0); // > 100 billion m²
        assert!(result.area_square_kilometers > 100_000.0);
    }

    #[test]
    fn test_calculate_polygon_area_basic() {
        let square = create_unit_square();
        let area = calculate_polygon_area(&square).unwrap();
        
        assert!(area > 0.0);
        assert!(area > 10_000_000_000.0); // Should be substantial for 1° square
    }

    #[test]
    fn test_calculate_polygon_area_clockwise_vs_counterclockwise() {
        let square_ccw = create_unit_square();
        let mut square_cw = square_ccw.clone();
        square_cw.reverse(); // Reverse to make clockwise
        
        let area_ccw = calculate_polygon_area(&square_ccw).unwrap();
        let area_cw = calculate_polygon_area(&square_cw).unwrap();
        
        // Areas should be equal (algorithm uses abs())
        assert!((area_ccw - area_cw).abs() < 1000.0);
    }

    #[test]
    fn test_polygon_area_at_poles() {
        // Polygon near north pole
        let polar_polygon = vec![
            Coordinate { lat: 89.0, lon: -1.0 },
            Coordinate { lat: 89.0, lon: 1.0 },
            Coordinate { lat: 89.5, lon: 1.0 },
            Coordinate { lat: 89.5, lon: -1.0 },
        ];
        
        let result = get_polygon_area(polar_polygon).unwrap();
        
        // Should still calculate an area, though small due to polar convergence
        assert!(result.area_square_meters > 0.0);
        assert!(result.area_square_meters < 1_000_000_000.0); // Should be much smaller than equatorial
    }

    #[test]
    fn test_polygon_area_crossing_dateline() {
        // Polygon crossing the international date line
        let dateline_polygon = vec![
            Coordinate { lat: 0.0, lon: 179.0 },
            Coordinate { lat: 0.0, lon: -179.0 },
            Coordinate { lat: 1.0, lon: -179.0 },
            Coordinate { lat: 1.0, lon: 179.0 },
        ];
        
        let result = get_polygon_area(dateline_polygon).unwrap();
        
        assert!(result.area_square_meters > 0.0);
        // Should be roughly equivalent to a 2° wide polygon
        assert!(result.area_square_meters > 1_000_000_000.0);
    }

    #[test]
    fn test_polygon_area_equatorial_vs_polar() {
        let equatorial = vec![
            Coordinate { lat: 0.0, lon: 0.0 },
            Coordinate { lat: 0.0, lon: 1.0 },
            Coordinate { lat: 1.0, lon: 1.0 },
            Coordinate { lat: 1.0, lon: 0.0 },
        ];
        
        let polar = vec![
            Coordinate { lat: 80.0, lon: 0.0 },
            Coordinate { lat: 80.0, lon: 1.0 },
            Coordinate { lat: 81.0, lon: 1.0 },
            Coordinate { lat: 81.0, lon: 0.0 },
        ];
        
        let eq_result = get_polygon_area(equatorial).unwrap();
        let polar_result = get_polygon_area(polar).unwrap();
        
        // Equatorial polygon should be larger due to less convergence
        assert!(eq_result.area_square_meters > polar_result.area_square_meters);
    }

    #[test]
    fn test_polygon_area_insufficient_coordinates() {
        let line = vec![
            Coordinate { lat: 0.0, lon: 0.0 },
            Coordinate { lat: 1.0, lon: 1.0 },
        ];
        
        let result = get_polygon_area(line);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Polygon must have at least 3 coordinates");
        
        let single_point = vec![Coordinate { lat: 0.0, lon: 0.0 }];
        let result = get_polygon_area(single_point);
        assert!(result.is_err());
        
        let empty: Vec<Coordinate> = vec![];
        let result = get_polygon_area(empty);
        assert!(result.is_err());
    }

    #[test]
    fn test_polygon_area_invalid_coordinates() {
        let mut invalid_polygon = create_unit_square();
        invalid_polygon[0].lat = 91.0; // Invalid latitude
        
        let result = get_polygon_area(invalid_polygon);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid latitude"));
        
        let mut invalid_polygon = create_unit_square();
        invalid_polygon[1].lon = 181.0; // Invalid longitude
        
        let result = get_polygon_area(invalid_polygon);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Invalid longitude"));
    }

    #[test]
    fn test_polygon_area_boundary_coordinates() {
        // Test with boundary valid coordinates
        let boundary_polygon = vec![
            Coordinate { lat: -90.0, lon: -180.0 },
            Coordinate { lat: -90.0, lon: 180.0 },
            Coordinate { lat: 90.0, lon: 180.0 },
            Coordinate { lat: 90.0, lon: -180.0 },
        ];
        
        let result = get_polygon_area(boundary_polygon);
        assert!(result.is_ok());
        
        // Should be approximately the surface area of Earth
        let area = result.unwrap().area_square_meters;
        assert!(area > 400_000_000_000_000.0); // > 400 trillion m²
    }

    #[test]
    fn test_polygon_area_nan_coordinates() {
        let mut nan_polygon = create_unit_square();
        nan_polygon[0].lat = f64::NAN;
        
        let result = get_polygon_area(nan_polygon);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coordinate latitude cannot be NaN or infinite");
        
        let mut nan_polygon = create_unit_square();
        nan_polygon[1].lon = f64::NAN;
        
        let result = get_polygon_area(nan_polygon);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coordinate longitude cannot be NaN or infinite");
    }

    #[test]
    fn test_polygon_area_infinite_coordinates() {
        let mut inf_polygon = create_unit_square();
        inf_polygon[0].lat = f64::INFINITY;
        
        let result = get_polygon_area(inf_polygon);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coordinate latitude cannot be NaN or infinite");
        
        let mut inf_polygon = create_unit_square();
        inf_polygon[1].lon = f64::NEG_INFINITY;
        
        let result = get_polygon_area(inf_polygon);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coordinate longitude cannot be NaN or infinite");
    }

    #[test]
    fn test_polygon_area_unit_conversions() {
        let square = create_unit_square();
        let result = get_polygon_area(square).unwrap();
        
        // Verify conversion formulas
        let expected_km2 = result.area_square_meters / 1_000_000.0;
        let expected_miles2 = result.area_square_meters / 2_589_988.11;
        let expected_hectares = result.area_square_meters / 10_000.0;
        let expected_acres = result.area_square_meters / 4_046.86;
        
        assert!((result.area_square_kilometers - expected_km2).abs() < 0.01);
        assert!((result.area_square_miles - expected_miles2).abs() < 0.01);
        assert!((result.area_hectares - expected_hectares).abs() < 0.01);
        assert!((result.area_acres - expected_acres).abs() < 0.01);
    }

    #[test]
    fn test_polygon_area_complex_shape() {
        // Pentagon shape
        let pentagon = vec![
            Coordinate { lat: 40.0, lon: -74.0 },
            Coordinate { lat: 40.5, lon: -73.5 },
            Coordinate { lat: 40.8, lon: -74.0 },
            Coordinate { lat: 40.5, lon: -74.5 },
            Coordinate { lat: 40.0, lon: -74.3 },
        ];
        
        let result = get_polygon_area(pentagon).unwrap();
        
        assert!(result.area_square_meters > 0.0);
        assert!(result.area_square_kilometers > 0.0);
        assert!(result.area_square_miles > 0.0);
        assert!(result.area_hectares > 0.0);
        assert!(result.area_acres > 0.0);
        
        // Should be a reasonable area for this size polygon
        assert!(result.area_square_meters > 1_000_000.0); // > 1 km²
        assert!(result.area_square_meters < 10_000_000_000.0); // < 10,000 km²
    }
}