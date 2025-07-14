use ftl_sdk::{tool, ToolResponse};
use serde::Deserialize;
use schemars::JsonSchema;

#[derive(Deserialize, JsonSchema)]
struct PolygonAreaInput {
    /// Array of coordinates forming the polygon
    coordinates: Vec<Coordinate>,
}

#[derive(Deserialize, JsonSchema)]
struct Coordinate {
    /// Latitude
    lat: f64,
    /// Longitude
    lon: f64,
}

/// Calculate the area of a polygon defined by GPS coordinates
#[tool]
fn polygon_area(input: PolygonAreaInput) -> ToolResponse {
    use crate::geospatial::polygon_area::{Coordinate as InternalCoordinate, calculate_polygon_area};
    
    let coordinates: Vec<InternalCoordinate> = input.coordinates.into_iter().map(|c| InternalCoordinate {
        lat: c.lat,
        lon: c.lon,
    }).collect();
    
    match calculate_polygon_area(&coordinates) {
        Ok(area_square_meters) => {
            let result = serde_json::json!({
                "area_square_meters": area_square_meters,
                "area_square_kilometers": area_square_meters / 1_000_000.0,
                "area_square_miles": area_square_meters / 2_589_988.11,
                "area_hectares": area_square_meters / 10_000.0,
            });
            ToolResponse::text(serde_json::to_string(&result).unwrap())
        },
        Err(e) => ToolResponse::text(format!("Error: {}", e)),
    }
}