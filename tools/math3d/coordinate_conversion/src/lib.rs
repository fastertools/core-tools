use ftl_sdk::{tool, ToolResponse};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SphericalCoord {
    pub radius: f64,
    pub theta: f64,
    pub phi: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CylindricalCoord {
    pub radius: f64,
    pub theta: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
struct CoordinateConversionInput {
    from_type: String,
    to_type: String,
    coordinates: Vector3D,
}

#[derive(Serialize)]
struct CoordinateConversionResponse {
    original: Vector3D,
    converted: Vector3D,
    from_type: String,
    to_type: String,
}

fn cartesian_to_spherical(v: &Vector3D) -> SphericalCoord {
    let radius = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    let theta = v.y.atan2(v.x);
    let phi = if radius > 0.0 { (v.z / radius).acos() } else { 0.0 };
    
    SphericalCoord { radius, theta, phi }
}

fn spherical_to_cartesian(coord: &SphericalCoord) -> Vector3D {
    let sin_phi = coord.phi.sin();
    let cos_phi = coord.phi.cos();
    let sin_theta = coord.theta.sin();
    let cos_theta = coord.theta.cos();
    
    Vector3D {
        x: coord.radius * sin_phi * cos_theta,
        y: coord.radius * sin_phi * sin_theta,
        z: coord.radius * cos_phi,
    }
}

fn cartesian_to_cylindrical(v: &Vector3D) -> CylindricalCoord {
    let radius = (v.x * v.x + v.y * v.y).sqrt();
    let theta = v.y.atan2(v.x);
    
    CylindricalCoord { radius, theta, z: v.z }
}

fn cylindrical_to_cartesian(coord: &CylindricalCoord) -> Vector3D {
    let cos_theta = coord.theta.cos();
    let sin_theta = coord.theta.sin();
    
    Vector3D {
        x: coord.radius * cos_theta,
        y: coord.radius * sin_theta,
        z: coord.z,
    }
}

#[tool]
fn coordinate_conversion(input: CoordinateConversionInput) -> ToolResponse {
    let converted = match (input.from_type.to_lowercase().as_str(), input.to_type.to_lowercase().as_str()) {
        ("cartesian", "spherical") => {
            let spherical = cartesian_to_spherical(&input.coordinates);
            Vector3D {
                x: spherical.radius,
                y: spherical.theta,
                z: spherical.phi,
            }
        },
        ("spherical", "cartesian") => {
            let spherical = SphericalCoord {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                phi: input.coordinates.z,
            };
            spherical_to_cartesian(&spherical)
        },
        ("cartesian", "cylindrical") => {
            let cylindrical = cartesian_to_cylindrical(&input.coordinates);
            Vector3D {
                x: cylindrical.radius,
                y: cylindrical.theta,
                z: cylindrical.z,
            }
        },
        ("cylindrical", "cartesian") => {
            let cylindrical = CylindricalCoord {
                radius: input.coordinates.x,
                theta: input.coordinates.y,
                z: input.coordinates.z,
            };
            cylindrical_to_cartesian(&cylindrical)
        },
        _ => {
            return ToolResponse::error("Invalid coordinate conversion. Supported: cartesian↔spherical, cartesian↔cylindrical");
        }
    };

    let response = CoordinateConversionResponse {
        original: input.coordinates,
        converted,
        from_type: input.from_type,
        to_type: input.to_type,
    };
    
    match serde_json::to_string(&response) {
        Ok(json) => ToolResponse::text(json),
        Err(e) => ToolResponse::error(&format!("Serialization error: {}", e)),
    }
}