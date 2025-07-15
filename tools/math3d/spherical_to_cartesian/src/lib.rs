use ftl_sdk::tool;
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

#[derive(Deserialize, JsonSchema)]
pub struct SphericalInput {
    pub coordinates: SphericalCoord,
}

#[derive(Serialize, JsonSchema)]
pub struct SphericalToCartesianResult {
    pub original_spherical: SphericalCoord,
    pub cartesian_coordinates: Vector3D,
    pub conversion_notes: String,
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

#[tool]
pub fn spherical_to_cartesian_conversion(input: SphericalInput) -> Result<SphericalToCartesianResult, String> {
    if input.coordinates.radius < 0.0 {
        return Err("Radius must be non-negative".to_string());
    }
    
    let cartesian = spherical_to_cartesian(&input.coordinates);
    
    let conversion_notes = format!(
        "Converted from Spherical (r={:.3}, θ={:.3} rad, φ={:.3} rad) to Cartesian ({:.3}, {:.3}, {:.3})",
        input.coordinates.radius,
        input.coordinates.theta,
        input.coordinates.phi,
        cartesian.x,
        cartesian.y,
        cartesian.z
    );
    
    Ok(SphericalToCartesianResult {
        original_spherical: input.coordinates,
        cartesian_coordinates: cartesian,
        conversion_notes,
    })
}