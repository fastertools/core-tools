use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod common;
mod geospatial;
mod coordinate_utils;
mod geofencing;
mod math_3d;

use common::ErrorResponse;

/// Geospatial Tools API
#[http_component]
fn handle_tool(req: Request) -> anyhow::Result<impl IntoResponse> {
    let method = req.method().to_string();
    let path = req.header("spin-path-info")
        .map(|v| v.as_str().unwrap_or(""))
        .unwrap_or("");
    
    match method.as_str() {
        "GET" => {
            match path {
                "/health" => {
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(r#"{"status": "healthy", "service": "Geospatial & 3D Mathematics API"}"#)
                        .build())
                }
                "/info" | "/" => {
                    let info = r#"{
                        "service": "Geospatial & 3D Mathematics API",
                        "endpoints": {
                            "distance": "POST /distance - Calculate distance between two GPS coordinates",
                            "bearing": "POST /bearing - Calculate bearing between two GPS coordinates", 
                            "convert_to_dms": "POST /convert/to-dms - Convert decimal degrees to DMS format",
                            "convert_to_decimal": "POST /convert/to-decimal - Convert DMS to decimal degrees",
                            "polygon_area": "POST /polygon/area - Calculate area of GPS polygon",
                            "validate": "POST /validate - Validate GPS coordinates",
                            "point_in_polygon": "POST /geofence/point-in-polygon - Check if point is inside polygon",
                            "multi_point_check": "POST /geofence/multi-point - Check multiple points against polygon",
                            "circular_buffer": "POST /buffer/circular - Create circular buffer around point",
                            "polygon_buffer": "POST /buffer/polygon - Create buffer around polygon",
                            "multi_buffer": "POST /buffer/multi-distance - Create multiple distance buffers",
                            "nearest_points": "POST /proximity/nearest - Find nearest points to a location",
                            "distance_to_polygon": "POST /proximity/distance-to-polygon - Calculate distance from point to polygon",
                            "proximity_zone": "POST /proximity/zone - Analyze points within proximity zone",
                            "dot_product": "POST /3d/dot-product - Calculate dot product of two 3D vectors",
                            "cross_product": "POST /3d/cross-product - Calculate cross product of two 3D vectors",
                            "vector_magnitude": "POST /3d/vector-magnitude - Calculate magnitude and unit vector",
                            "vector_angle": "POST /3d/vector-angle - Calculate angle between two vectors",
                            "line_intersection": "POST /3d/line-intersection - Find intersection of two 3D lines",
                            "line_segment_intersection": "POST /3d/segment-intersection - Find intersection of two 3D line segments",
                            "multiple_line_intersection": "POST /3d/multi-line-intersection - Find best intersection point for multiple lines",
                            "line_plane_intersection": "POST /3d/line-plane - Find intersection of line and plane",
                            "plane_plane_intersection": "POST /3d/plane-plane - Find intersection of two planes",
                            "point_plane_distance": "POST /3d/point-plane-distance - Calculate distance from point to plane",
                            "tetrahedron_volume": "POST /3d/volume/tetrahedron - Calculate volume of tetrahedron from 4 points",
                            "sphere_volume": "POST /3d/volume/sphere - Calculate volume of sphere",
                            "cylinder_volume": "POST /3d/volume/cylinder - Calculate volume of cylinder",
                            "aabb_volume": "POST /3d/volume/aabb - Calculate volume of axis-aligned bounding box",
                            "pyramid_volume": "POST /3d/volume/pyramid - Calculate volume of pyramid",
                            "convex_hull_volume": "POST /3d/volume/convex-hull - Calculate volume of convex hull"
                        }
                    }"#;
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(info)
                        .build())
                }
                _ => {
                    let error_response = ErrorResponse {
                        error: "Endpoint not found. Use GET / for available endpoints".to_string(),
                    };
                    Ok(Response::builder()
                        .status(404)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&error_response)?)
                        .build())
                }
            }
        }
        "POST" => {
            let body = req.body();
            
            match path {
                "/distance" => {
                    let input: geospatial::distance::CoordinateInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    if let Err(e) = common::validate_coordinates(input.lat1, input.lon1) {
                        let error_response = ErrorResponse { error: e };
                        return Ok(Response::builder()
                            .status(400)
                            .header("content-type", "application/json")
                            .body(serde_json::to_string(&error_response)?)
                            .build());
                    }
                    
                    if let Err(e) = common::validate_coordinates(input.lat2, input.lon2) {
                        let error_response = ErrorResponse { error: e };
                        return Ok(Response::builder()
                            .status(400)
                            .header("content-type", "application/json")
                            .body(serde_json::to_string(&error_response)?)
                            .build());
                    }
                    
                    let result = geospatial::distance::calculate_distance(input);
                    
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&result)?)
                        .build())
                }
                "/bearing" => {
                    let input: geospatial::bearing::BearingInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    if let Err(e) = common::validate_coordinates(input.lat1, input.lon1) {
                        let error_response = ErrorResponse { error: e };
                        return Ok(Response::builder()
                            .status(400)
                            .header("content-type", "application/json")
                            .body(serde_json::to_string(&error_response)?)
                            .build());
                    }
                    
                    if let Err(e) = common::validate_coordinates(input.lat2, input.lon2) {
                        let error_response = ErrorResponse { error: e };
                        return Ok(Response::builder()
                            .status(400)
                            .header("content-type", "application/json")
                            .body(serde_json::to_string(&error_response)?)
                            .build());
                    }
                    
                    let result = geospatial::bearing::get_bearing(input);
                    
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&result)?)
                        .build())
                }
                "/convert/to-dms" => {
                    let input: coordinate_utils::coordinate_conversion::DecimalDegreesInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match coordinate_utils::coordinate_conversion::convert_to_dms(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/convert/to-decimal" => {
                    let input: coordinate_utils::coordinate_conversion::DMSInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match coordinate_utils::coordinate_conversion::convert_to_decimal(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/polygon/area" => {
                    let input: geospatial::polygon_area::PolygonInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geospatial::polygon_area::get_polygon_area(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/validate" => {
                    let input: coordinate_utils::validation::CoordinateValidationInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    let result = coordinate_utils::validation::validate_coordinate_input(input);
                    
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&result)?)
                        .build())
                }
                "/geofence/point-in-polygon" => {
                    let input: geofencing::point_in_polygon::PointInPolygonInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::point_in_polygon::point_in_polygon_check(input, false) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/geofence/multi-point" => {
                    let input: geofencing::point_in_polygon::MultiPointInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::point_in_polygon::multi_point_check(input, false) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/buffer/circular" => {
                    let input: geofencing::buffer_zones::CircularBufferInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::buffer_zones::create_circular_buffer(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/proximity/nearest" => {
                    let input: geofencing::proximity::NearestPointsInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::proximity::find_nearest_points(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/buffer/polygon" => {
                    let input: geofencing::buffer_zones::PolygonBufferInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::buffer_zones::create_polygon_buffer(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/buffer/multi-distance" => {
                    #[derive(serde::Deserialize)]
                    struct MultiDistanceInput {
                        center: geofencing::buffer_zones::Point,
                        distances: Vec<f64>,
                    }
                    
                    let input: MultiDistanceInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::buffer_zones::create_multi_distance_buffers(input.center, input.distances) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/proximity/distance-to-polygon" => {
                    let input: geofencing::proximity::DistanceToPolygonInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::proximity::distance_to_polygon(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/proximity/zone" => {
                    let input: geofencing::proximity::ProximityZoneInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match geofencing::proximity::proximity_zone_analysis(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/3d/dot-product" => {
                    let input: math_3d::vector_ops::TwoVectorInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::vector_ops::compute_dot_product(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/3d/cross-product" => {
                    let input: math_3d::vector_ops::TwoVectorInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    let result = math_3d::vector_ops::compute_cross_product(input);
                    
                    Ok(Response::builder()
                        .status(200)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&result)?)
                        .build())
                }
                "/3d/line-intersection" => {
                    let input: math_3d::line_intersection::TwoLineInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::line_intersection::detect_line_intersection(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            let error_response = ErrorResponse { error: e };
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/tetrahedron" => {
                    let input: math_3d::volume_calculations::TetrahedronInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_tetrahedron_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/sphere" => {
                    let input: math_3d::volume_calculations::SphereInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_sphere_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/cylinder" => {
                    let input: math_3d::volume_calculations::CylinderInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_cylinder_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/aabb" => {
                    let input: math_3d::volume_calculations::BoundingBoxInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_aabb_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/pyramid" => {
                    let input: math_3d::volume_calculations::PyramidInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_pyramid_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                "/3d/volume/convex-hull" => {
                    let input: math_3d::volume_calculations::ConvexHullInput = match serde_json::from_slice(body) {
                        Ok(input) => input,
                        Err(e) => {
                            let error_response = ErrorResponse {
                                error: format!("Invalid JSON input: {}", e),
                            };
                            return Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&error_response)?)
                                .build());
                        }
                    };
                    
                    match math_3d::volume_calculations::handle_convex_hull_volume(input) {
                        Ok(result) => {
                            Ok(Response::builder()
                                .status(200)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&result)?)
                                .build())
                        }
                        Err(e) => {
                            Ok(Response::builder()
                                .status(400)
                                .header("content-type", "application/json")
                                .body(serde_json::to_string(&e)?)
                                .build())
                        }
                    }
                }
                _ => {
                    let error_response = ErrorResponse {
                        error: "Endpoint not found. Use GET / for available endpoints".to_string(),
                    };
                    Ok(Response::builder()
                        .status(404)
                        .header("content-type", "application/json")
                        .body(serde_json::to_string(&error_response)?)
                        .build())
                }
            }
        }
        _ => {
            let error_response = ErrorResponse {
                error: "Method not allowed. Use GET for info or POST for operations".to_string(),
            };
            Ok(Response::builder()
                .status(405)
                .header("content-type", "application/json")
                .body(serde_json::to_string(&error_response)?)
                .build())
        }
    }
}
