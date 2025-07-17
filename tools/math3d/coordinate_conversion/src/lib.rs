use ftl_sdk::tool;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

mod logic;
use logic::{CoordinateConversionInput as LogicInput, coordinate_conversion_logic};

#[derive(Deserialize, JsonSchema)]
pub struct CoordinateConversionInput {
    /// Source coordinate system: "cartesian", "spherical", "cylindrical"
    pub from_type: String,
    /// Target coordinate system: "cartesian", "spherical", "cylindrical"  
    pub to_type: String,
    /// Input coordinates as Vector3D
    pub coordinates: Vector3D,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Vector3D {
    /// X coordinate (or radius for spherical/cylindrical)
    pub x: f64,
    /// Y coordinate (or theta for spherical/cylindrical)
    pub y: f64,
    /// Z coordinate (or phi for spherical)
    pub z: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CoordinateConversionResult {
    /// Original coordinates
    pub original: Vector3D,
    /// Converted coordinates
    pub converted: Vector3D,
    /// Source coordinate system
    pub from_type: String,
    /// Target coordinate system
    pub to_type: String,
}

// Helper structs for calling individual tools
#[derive(Serialize, Deserialize)]
struct CartesianCoordinates {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize)]
struct SphericalCoordinates {
    radius: f64,
    theta: f64,
    phi: f64,
}

#[derive(Deserialize)]
struct CartesianToSphericalResult {
    spherical_coordinates: SphericalCoordinates,
}

#[derive(Deserialize)]
struct SphericalToCartesianResult {
    cartesian_coordinates: CartesianCoordinates,
}

#[derive(Serialize, Deserialize)]
struct CylindricalCoordinates {
    radius: f64,
    theta: f64,
    z: f64,
}

#[derive(Deserialize)]
struct CartesianToCylindricalResult {
    cylindrical_coordinates: CylindricalCoordinates,
}

#[derive(Deserialize)]
struct CylindricalToCartesianResult {
    cartesian_coordinates: CartesianCoordinates,
}

#[derive(Deserialize)]
struct OkResponse<T> {
    #[serde(rename = "Ok")]
    ok: T,
}

/// Convert between different 3D coordinate systems (cartesian, spherical, cylindrical)
/// For cartesian↔spherical conversions, delegates to individual tools via HTTP
#[cfg_attr(not(test), tool)]
pub async fn coordinate_conversion(input: CoordinateConversionInput) -> Result<CoordinateConversionResult, String> {
    use spin_sdk::http::{Method, Request};
    
    // Normalize coordinate system names
    let from_type = input.from_type.to_lowercase();
    let to_type = input.to_type.to_lowercase();
    
    let converted = match (from_type.as_str(), to_type.as_str()) {
        ("cartesian", "spherical") => {
            // Call cartesian-to-spherical tool via HTTP
            let cartesian_input = CartesianCoordinates {
                x: input.coordinates.x,
                y: input.coordinates.y,
                z: input.coordinates.z,
            };
            let request_body = serde_json::to_string(&cartesian_input)
                .map_err(|e| format!("Failed to serialize cartesian input: {}", e))?;
            
            let request = Request::builder()
                .method(Method::Post)
                .uri("http://cartesian-to-spherical.spin.internal")
                .header("Content-Type", "application/json")
                .body(request_body.into_bytes())
                .build();
            
            let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
                .map_err(|e| format!("Error calling cartesian-to-spherical tool: {:?}", e))?;
            
            let body_bytes = response.into_body();
            let body = String::from_utf8(body_bytes)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let result: OkResponse<CartesianToSphericalResult> = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse cartesian-to-spherical result: {}. Response body: {}", e, body))?;
            
            Vector3D {
                x: result.ok.spherical_coordinates.radius,
                y: result.ok.spherical_coordinates.theta,
                z: result.ok.spherical_coordinates.phi,
            }
        },
        ("spherical", "cartesian") => {
            // Call spherical-to-cartesian tool via HTTP
            let spherical_input = SphericalCoordinates {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                phi: input.coordinates.z,
            };
            let request_body = serde_json::to_string(&spherical_input)
                .map_err(|e| format!("Failed to serialize spherical input: {}", e))?;
            
            let request = Request::builder()
                .method(Method::Post)
                .uri("http://spherical-to-cartesian.spin.internal")
                .header("Content-Type", "application/json")
                .body(request_body.into_bytes())
                .build();
            
            let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
                .map_err(|e| format!("Error calling spherical-to-cartesian tool: {:?}", e))?;
            
            let body_bytes = response.into_body();
            let body = String::from_utf8(body_bytes)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let result: OkResponse<SphericalToCartesianResult> = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse spherical-to-cartesian result: {}. Response body: {}", e, body))?;
            
            Vector3D {
                x: result.ok.cartesian_coordinates.x,
                y: result.ok.cartesian_coordinates.y,
                z: result.ok.cartesian_coordinates.z,
            }
        },
        ("cartesian", "cylindrical") => {
            // Call cartesian-to-cylindrical tool via HTTP
            let cartesian_input = CartesianCoordinates {
                x: input.coordinates.x,
                y: input.coordinates.y,
                z: input.coordinates.z,
            };
            let request_body = serde_json::to_string(&cartesian_input)
                .map_err(|e| format!("Failed to serialize cartesian input: {}", e))?;
            
            let request = Request::builder()
                .method(Method::Post)
                .uri("http://cartesian-to-cylindrical.spin.internal")
                .header("Content-Type", "application/json")
                .body(request_body.into_bytes())
                .build();
            
            let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
                .map_err(|e| format!("Error calling cartesian-to-cylindrical tool: {:?}", e))?;
            
            let body_bytes = response.into_body();
            let body = String::from_utf8(body_bytes)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let result: OkResponse<CartesianToCylindricalResult> = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse cartesian-to-cylindrical result: {}. Response body: {}", e, body))?;
            
            Vector3D {
                x: result.ok.cylindrical_coordinates.radius,
                y: result.ok.cylindrical_coordinates.theta,
                z: result.ok.cylindrical_coordinates.z,
            }
        },
        ("cylindrical", "cartesian") => {
            // Call cylindrical-to-cartesian tool via HTTP
            let cylindrical_input = CylindricalCoordinates {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                z: input.coordinates.z,
            };
            let request_body = serde_json::to_string(&cylindrical_input)
                .map_err(|e| format!("Failed to serialize cylindrical input: {}", e))?;
            
            let request = Request::builder()
                .method(Method::Post)
                .uri("http://cylindrical-to-cartesian.spin.internal")
                .header("Content-Type", "application/json")
                .body(request_body.into_bytes())
                .build();
            
            let response: spin_sdk::http::Response = spin_sdk::http::send(request).await
                .map_err(|e| format!("Error calling cylindrical-to-cartesian tool: {:?}", e))?;
            
            let body_bytes = response.into_body();
            let body = String::from_utf8(body_bytes)
                .map_err(|e| format!("Failed to parse response body: {}", e))?;
            
            let result: OkResponse<CylindricalToCartesianResult> = serde_json::from_str(&body)
                .map_err(|e| format!("Failed to parse cylindrical-to-cartesian result: {}. Response body: {}", e, body))?;
            
            Vector3D {
                x: result.ok.cartesian_coordinates.x,
                y: result.ok.cartesian_coordinates.y,
                z: result.ok.cartesian_coordinates.z,
            }
        },
        _ => {
            return Err("Invalid coordinate conversion. Supported: cartesian↔spherical, cartesian↔cylindrical".to_string());
        }
    };
    
    Ok(CoordinateConversionResult {
        original: input.coordinates,
        converted,
        from_type: input.from_type,
        to_type: input.to_type,
    })
}