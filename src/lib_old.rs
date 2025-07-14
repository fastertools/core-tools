use wasmcp::{ToolHandler, json};

pub mod common;
pub mod geospatial;
pub mod coordinate_utils;
pub mod geofencing;
pub mod math_3d;
pub mod statistics;

// ErrorResponse is used indirectly through the volume_calculations and primitives modules

// =============================================================================
// GEOSPATIAL TOOLS
// =============================================================================

/// Calculate distance between two GPS coordinates using Haversine formula
pub struct DistanceTool;

impl ToolHandler for DistanceTool {
    const NAME: &'static str = "distance";
    const DESCRIPTION: &'static str = "Calculate distance between two GPS coordinates using Haversine formula";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "lat1": { "type": "number", "description": "Latitude of first point" },
                "lon1": { "type": "number", "description": "Longitude of first point" },
                "lat2": { "type": "number", "description": "Latitude of second point" },
                "lon2": { "type": "number", "description": "Longitude of second point" }
            },
            "required": ["lat1", "lon1", "lat2", "lon2"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geospatial::distance::CoordinateInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        if let Err(e) = common::validate_coordinates(input.lat1, input.lon1) {
            return Err(e);
        }
        
        if let Err(e) = common::validate_coordinates(input.lat2, input.lon2) {
            return Err(e);
        }
        
        let result = geospatial::distance::calculate_distance(input);
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
}

/// Calculate bearing between two GPS coordinates
pub struct BearingTool;

impl ToolHandler for BearingTool {
    const NAME: &'static str = "bearing";
    const DESCRIPTION: &'static str = "Calculate bearing between two GPS coordinates";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "lat1": { "type": "number", "description": "Latitude of first point" },
                "lon1": { "type": "number", "description": "Longitude of first point" },
                "lat2": { "type": "number", "description": "Latitude of second point" },
                "lon2": { "type": "number", "description": "Longitude of second point" }
            },
            "required": ["lat1", "lon1", "lat2", "lon2"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geospatial::bearing::BearingInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        if let Err(e) = common::validate_coordinates(input.lat1, input.lon1) {
            return Err(e);
        }
        
        if let Err(e) = common::validate_coordinates(input.lat2, input.lon2) {
            return Err(e);
        }
        
        let result = geospatial::bearing::get_bearing(input);
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
}

/// Calculate area of a GPS polygon
pub struct PolygonAreaTool;

impl ToolHandler for PolygonAreaTool {
    const NAME: &'static str = "polygon_area";
    const DESCRIPTION: &'static str = "Calculate area of a GPS polygon";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "coordinates": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "lat": { "type": "number" },
                            "lon": { "type": "number" }
                        },
                        "required": ["lat", "lon"]
                    },
                    "description": "Array of GPS coordinates forming the polygon"
                }
            },
            "required": ["coordinates"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geospatial::polygon_area::PolygonInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match geospatial::polygon_area::get_polygon_area(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

// =============================================================================
// COORDINATE UTILITIES
// =============================================================================

/// Convert decimal degrees to DMS format
pub struct ConvertToDMSTool;

impl ToolHandler for ConvertToDMSTool {
    const NAME: &'static str = "convert_to_dms";
    const DESCRIPTION: &'static str = "Convert decimal degrees to DMS (Degrees, Minutes, Seconds) format";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "latitude": { "type": "number", "description": "Latitude in decimal degrees" },
                "longitude": { "type": "number", "description": "Longitude in decimal degrees" }
            },
            "required": ["latitude", "longitude"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: coordinate_utils::coordinate_conversion::DecimalDegreesInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match coordinate_utils::coordinate_conversion::convert_to_dms(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Convert DMS format to decimal degrees
pub struct ConvertToDecimalTool;

impl ToolHandler for ConvertToDecimalTool {
    const NAME: &'static str = "convert_to_decimal";
    const DESCRIPTION: &'static str = "Convert DMS (Degrees, Minutes, Seconds) format to decimal degrees";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "latitude": {
                    "type": "object",
                    "properties": {
                        "degrees": { "type": "number" },
                        "minutes": { "type": "number" },
                        "seconds": { "type": "number" },
                        "direction": { "type": "string", "enum": ["N", "S"] }
                    },
                    "required": ["degrees", "minutes", "seconds", "direction"]
                },
                "longitude": {
                    "type": "object",
                    "properties": {
                        "degrees": { "type": "number" },
                        "minutes": { "type": "number" },
                        "seconds": { "type": "number" },
                        "direction": { "type": "string", "enum": ["E", "W"] }
                    },
                    "required": ["degrees", "minutes", "seconds", "direction"]
                }
            },
            "required": ["latitude", "longitude"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: coordinate_utils::coordinate_conversion::DMSInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match coordinate_utils::coordinate_conversion::convert_to_decimal(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Validate GPS coordinates
pub struct ValidateCoordinatesTool;

impl ToolHandler for ValidateCoordinatesTool {
    const NAME: &'static str = "validate_coordinates";
    const DESCRIPTION: &'static str = "Validate GPS coordinates and provide information about them";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "latitude": { "type": "number", "description": "Latitude to validate" },
                "longitude": { "type": "number", "description": "Longitude to validate" }
            },
            "required": ["latitude", "longitude"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: coordinate_utils::validation::CoordinateValidationInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        let result = coordinate_utils::validation::validate_coordinate_input(input);
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
}

// =============================================================================
// GEOFENCING TOOLS
// =============================================================================

/// Check if a point is inside a polygon (geofencing)
pub struct PointInPolygonTool;

impl ToolHandler for PointInPolygonTool {
    const NAME: &'static str = "point_in_polygon";
    const DESCRIPTION: &'static str = "Check if a GPS point is inside a polygon using ray casting algorithm";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "point": {
                    "type": "object",
                    "properties": {
                        "lat": { "type": "number" },
                        "lon": { "type": "number" }
                    },
                    "required": ["lat", "lon"]
                },
                "polygon": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "lat": { "type": "number" },
                            "lon": { "type": "number" }
                        },
                        "required": ["lat", "lon"]
                    },
                    "description": "Array of GPS coordinates forming the polygon"
                }
            },
            "required": ["point", "polygon"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geofencing::point_in_polygon::PointInPolygonInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match geofencing::point_in_polygon::point_in_polygon_check(input, false) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Check multiple points against a polygon
pub struct MultiPointCheckTool;

impl ToolHandler for MultiPointCheckTool {
    const NAME: &'static str = "multi_point_check";
    const DESCRIPTION: &'static str = "Check multiple GPS points against a polygon for batch geofencing";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "points": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "lat": { "type": "number" },
                            "lon": { "type": "number" }
                        },
                        "required": ["lat", "lon"]
                    },
                    "description": "Array of GPS points to check"
                },
                "polygon": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "lat": { "type": "number" },
                            "lon": { "type": "number" }
                        },
                        "required": ["lat", "lon"]
                    },
                    "description": "Array of GPS coordinates forming the polygon"
                }
            },
            "required": ["points", "polygon"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geofencing::point_in_polygon::MultiPointInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match geofencing::point_in_polygon::multi_point_check(input, false) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Create circular buffer around a point
pub struct CircularBufferTool;

impl ToolHandler for CircularBufferTool {
    const NAME: &'static str = "circular_buffer";
    const DESCRIPTION: &'static str = "Create circular buffer zone around a GPS point";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "center": {
                    "type": "object",
                    "properties": {
                        "lat": { "type": "number" },
                        "lon": { "type": "number" }
                    },
                    "required": ["lat", "lon"]
                },
                "radius_km": { "type": "number", "description": "Radius in kilometers" },
                "num_points": { "type": "number", "description": "Number of points to generate (default: 36)" }
            },
            "required": ["center", "radius_km"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: geofencing::buffer_zones::CircularBufferInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match geofencing::buffer_zones::create_circular_buffer(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

// =============================================================================
// 3D MATHEMATICS TOOLS
// =============================================================================

/// Calculate 3D vector dot product
pub struct DotProductTool;

impl ToolHandler for DotProductTool {
    const NAME: &'static str = "dot_product_3d";
    const DESCRIPTION: &'static str = "Calculate dot product of two 3D vectors";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "vector1": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "vector2": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                }
            },
            "required": ["vector1", "vector2"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::vector_ops::TwoVectorInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match math_3d::vector_ops::compute_dot_product(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Calculate 3D vector cross product
pub struct CrossProductTool;

impl ToolHandler for CrossProductTool {
    const NAME: &'static str = "cross_product_3d";
    const DESCRIPTION: &'static str = "Calculate cross product of two 3D vectors";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "vector1": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "vector2": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                }
            },
            "required": ["vector1", "vector2"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::vector_ops::TwoVectorInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        let result = math_3d::vector_ops::compute_cross_product(input);
        serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e))
    }
}

/// Calculate 3D line intersection
pub struct LineIntersectionTool;

impl ToolHandler for LineIntersectionTool {
    const NAME: &'static str = "line_intersection_3d";
    const DESCRIPTION: &'static str = "Find intersection of two 3D lines";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "line1": {
                    "type": "object",
                    "properties": {
                        "point": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        },
                        "direction": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        }
                    },
                    "required": ["point", "direction"]
                },
                "line2": {
                    "type": "object",
                    "properties": {
                        "point": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        },
                        "direction": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        }
                    },
                    "required": ["point", "direction"]
                }
            },
            "required": ["line1", "line2"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::line_intersection::TwoLineInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match math_3d::line_intersection::detect_line_intersection(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Calculate 3D tetrahedron volume
pub struct TetrahedronVolumeTool;

impl ToolHandler for TetrahedronVolumeTool {
    const NAME: &'static str = "tetrahedron_volume_3d";
    const DESCRIPTION: &'static str = "Calculate volume of tetrahedron from 4 points in 3D space";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "point_a": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "point_b": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "point_c": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "point_d": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                }
            },
            "required": ["point_a", "point_b", "point_c", "point_d"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::volume_calculations::TetrahedronInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match math_3d::volume_calculations::handle_tetrahedron_volume(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e.error)
        }
    }
}

/// Calculate 3D point to line distance
pub struct PointLineDistanceTool;

impl ToolHandler for PointLineDistanceTool {
    const NAME: &'static str = "point_line_distance_3d";
    const DESCRIPTION: &'static str = "Calculate distance from a point to a line in 3D space";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "point": {
                    "type": "object",
                    "properties": {
                        "x": { "type": "number" },
                        "y": { "type": "number" },
                        "z": { "type": "number" }
                    },
                    "required": ["x", "y", "z"]
                },
                "line": {
                    "type": "object",
                    "properties": {
                        "point": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        },
                        "direction": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        }
                    },
                    "required": ["point", "direction"]
                }
            },
            "required": ["point", "line"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::distance_operations::PointLineInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match math_3d::distance_operations::compute_point_line_distance(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Calculate 3D sphere-ray intersection
pub struct SphereRayIntersectionTool;

impl ToolHandler for SphereRayIntersectionTool {
    const NAME: &'static str = "sphere_ray_intersection_3d";
    const DESCRIPTION: &'static str = "Test ray-sphere intersection in 3D space";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "sphere": {
                    "type": "object",
                    "properties": {
                        "center": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        },
                        "radius": { "type": "number" }
                    },
                    "required": ["center", "radius"]
                },
                "ray": {
                    "type": "object",
                    "properties": {
                        "origin": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        },
                        "direction": {
                            "type": "object",
                            "properties": {
                                "x": { "type": "number" },
                                "y": { "type": "number" },
                                "z": { "type": "number" }
                            },
                            "required": ["x", "y", "z"]
                        }
                    },
                    "required": ["origin", "direction"]
                }
            },
            "required": ["sphere", "ray"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: math_3d::primitives::SphereRayInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match math_3d::primitives::handle_sphere_ray_intersection(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e.error)
        }
    }
}

// =============================================================================
// STATISTICAL ANALYSIS TOOLS
// =============================================================================

/// Calculate descriptive statistics
pub struct DescriptiveStatsTool;

impl ToolHandler for DescriptiveStatsTool {
    const NAME: &'static str = "descriptive_stats";
    const DESCRIPTION: &'static str = "Calculate comprehensive descriptive statistics including mean, median, mode, standard deviation, variance, skewness, and kurtosis";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "data": {
                    "type": "array",
                    "items": { "type": "number" },
                    "description": "Array of numerical data"
                }
            },
            "required": ["data"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: statistics::descriptive::StatisticsInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match statistics::descriptive::calculate_descriptive_statistics(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Calculate Pearson correlation
pub struct PearsonCorrelationTool;

impl ToolHandler for PearsonCorrelationTool {
    const NAME: &'static str = "pearson_correlation";
    const DESCRIPTION: &'static str = "Calculate Pearson product-moment correlation coefficient between two variables";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "x": {
                    "type": "array",
                    "items": { "type": "number" },
                    "description": "First variable data"
                },
                "y": {
                    "type": "array",
                    "items": { "type": "number" },
                    "description": "Second variable data"
                }
            },
            "required": ["x", "y"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: statistics::correlation::TwoSeriesInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match statistics::correlation::calculate_pearson_correlation(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

/// Calculate linear regression
pub struct LinearRegressionTool;

impl ToolHandler for LinearRegressionTool {
    const NAME: &'static str = "linear_regression";
    const DESCRIPTION: &'static str = "Perform simple linear regression analysis with comprehensive statistics";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "x": {
                    "type": "array",
                    "items": { "type": "number" },
                    "description": "Independent variable data"
                },
                "y": {
                    "type": "array",
                    "items": { "type": "number" },
                    "description": "Dependent variable data"
                }
            },
            "required": ["x", "y"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let input: statistics::regression::RegressionInput = serde_json::from_value(args)
            .map_err(|e| format!("Invalid input: {}", e))?;
        
        match statistics::regression::calculate_linear_regression(input) {
            Ok(result) => serde_json::to_string(&result).map_err(|e| format!("Serialization error: {}", e)),
            Err(e) => Err(e)
        }
    }
}

// =============================================================================
// MAIN CORETOOLS HANDLER - DISPATCHES TO ALL TOOLS
// =============================================================================

/// Core Tools Dispatcher - Routes to appropriate tool based on description
pub struct CoreToolsDispatcher;

impl ToolHandler for CoreToolsDispatcher {
    const NAME: &'static str = "coretools";
    const DESCRIPTION: &'static str = "Comprehensive computational tools for geospatial analysis, 3D mathematics, and statistical operations. Specify the tool name and parameters to execute.";
    
    fn input_schema() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "tool": {
                    "type": "string",
                    "description": "Name of the tool to execute",
                    "enum": [
                        "distance", "bearing", "polygon_area",
                        "convert_to_dms", "convert_to_decimal", "validate_coordinates",
                        "point_in_polygon", "multi_point_check", "circular_buffer",
                        "dot_product_3d", "cross_product_3d", "line_intersection_3d",
                        "tetrahedron_volume_3d", "point_line_distance_3d", "sphere_ray_intersection_3d",
                        "descriptive_stats", "pearson_correlation", "linear_regression"
                    ]
                },
                "args": {
                    "type": "object",
                    "description": "Arguments for the specified tool"
                }
            },
            "required": ["tool", "args"]
        })
    }
    
    fn execute(args: serde_json::Value) -> Result<String, String> {
        let tool_name = args["tool"].as_str().ok_or("Missing tool name")?;
        let tool_args = args["args"].clone();
        
        match tool_name {
            "distance" => DistanceTool::execute(tool_args),
            "bearing" => BearingTool::execute(tool_args),
            "polygon_area" => PolygonAreaTool::execute(tool_args),
            "convert_to_dms" => ConvertToDMSTool::execute(tool_args),
            "convert_to_decimal" => ConvertToDecimalTool::execute(tool_args),
            "validate_coordinates" => ValidateCoordinatesTool::execute(tool_args),
            "point_in_polygon" => PointInPolygonTool::execute(tool_args),
            "multi_point_check" => MultiPointCheckTool::execute(tool_args),
            "circular_buffer" => CircularBufferTool::execute(tool_args),
            "dot_product_3d" => DotProductTool::execute(tool_args),
            "cross_product_3d" => CrossProductTool::execute(tool_args),
            "line_intersection_3d" => LineIntersectionTool::execute(tool_args),
            "tetrahedron_volume_3d" => TetrahedronVolumeTool::execute(tool_args),
            "point_line_distance_3d" => PointLineDistanceTool::execute(tool_args),
            "sphere_ray_intersection_3d" => SphereRayIntersectionTool::execute(tool_args),
            "descriptive_stats" => DescriptiveStatsTool::execute(tool_args),
            "pearson_correlation" => PearsonCorrelationTool::execute(tool_args),
            "linear_regression" => LinearRegressionTool::execute(tool_args),
            _ => Err(format!("Unknown tool: {}", tool_name))
        }
    }
}

// wasmcp::create_handler!(
//     tools: [CoreToolsDispatcher]
// )

// TODO: Generate the main MCP handler with all tools
// wasmcp::create_handler!(
//     tools: [
//         CoreToolsDispatcher,
//         DistanceTool,
//         BearingTool,
//         PolygonAreaTool,
//         ConvertToDMSTool,
//         ConvertToDecimalTool,
//         ValidateCoordinatesTool,
//         PointInPolygonTool,
//         MultiPointCheckTool,
//         CircularBufferTool,
//         DotProductTool,
//         CrossProductTool,
//         LineIntersectionTool,
//         TetrahedronVolumeTool,
//         PointLineDistanceTool,
//         SphereRayIntersectionTool,
//         DescriptiveStatsTool,
//         PearsonCorrelationTool,
//         LinearRegressionTool
//     ],
// );