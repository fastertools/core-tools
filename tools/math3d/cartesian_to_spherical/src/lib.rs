use ftl_sdk::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize, JsonSchema)]
pub struct CartesianInput {
    pub coordinates: Vector3D,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SphericalCoord {
    pub radius: f64,
    pub theta: f64,
    pub phi: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct CartesianToSphericalResult {
    pub original_cartesian: Vector3D,
    pub spherical_coordinates: SphericalCoord,
    pub conversion_notes: String,
}

fn cartesian_to_spherical(v: &Vector3D) -> SphericalCoord {
    let radius = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    let theta = v.y.atan2(v.x);
    let phi = if radius > 0.0 { (v.z / radius).acos() } else { 0.0 };
    
    SphericalCoord { radius, theta, phi }
}

#[tool]
pub fn cartesian_to_spherical_conversion(input: CartesianInput) -> Result<CartesianToSphericalResult, String> {
    let spherical = cartesian_to_spherical(&input.coordinates);
    
    let conversion_notes = format!(
        "Converted from Cartesian ({:.3}, {:.3}, {:.3}) to Spherical (r={:.3}, θ={:.3} rad, φ={:.3} rad)",
        input.coordinates.x,
        input.coordinates.y, 
        input.coordinates.z,
        spherical.radius,
        spherical.theta,
        spherical.phi
    );
    
    Ok(CartesianToSphericalResult {
        original_cartesian: input.coordinates,
        spherical_coordinates: spherical,
        conversion_notes,
    })
}