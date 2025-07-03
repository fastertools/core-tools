use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize)]
pub struct TwoVectorInput {
    pub vector1: Vector3D,
    pub vector2: Vector3D,
}

#[derive(Deserialize)]
pub struct SingleVectorInput {
    pub vector: Vector3D,
}

#[derive(Serialize)]
pub struct DotProductResult {
    pub dot_product: f64,
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub are_perpendicular: bool,
    pub are_parallel: bool,
}

#[derive(Serialize)]
pub struct CrossProductResult {
    pub cross_product: Vector3D,
    pub magnitude: f64,
    pub area_parallelogram: f64,
    pub are_parallel: bool,
}

#[derive(Serialize)]
pub struct VectorMagnitudeResult {
    pub magnitude: f64,
    pub unit_vector: Vector3D,
    pub is_zero_vector: bool,
}

#[derive(Serialize)]
pub struct VectorAngleResult {
    pub angle_radians: f64,
    pub angle_degrees: f64,
    pub cos_angle: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3D { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Result<Vector3D, String> {
        let mag = self.magnitude();
        if mag == 0.0 {
            return Err("Cannot normalize zero vector".to_string());
        }
        Ok(Vector3D {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        })
    }

    pub fn dot(&self, other: &Vector3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn add(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn subtract(&self, other: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    pub fn scale(&self, scalar: f64) -> Vector3D {
        Vector3D {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn distance_to(&self, other: &Vector3D) -> f64 {
        self.subtract(other).magnitude()
    }

    pub fn is_zero(&self) -> bool {
        const EPSILON: f64 = 1e-10;
        self.magnitude() < EPSILON
    }

    pub fn are_parallel(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        let cross = self.cross(other);
        cross.magnitude() < EPSILON
    }

    pub fn are_perpendicular(&self, other: &Vector3D) -> bool {
        const EPSILON: f64 = 1e-10;
        self.dot(other).abs() < EPSILON
    }

    pub fn angle_with(&self, other: &Vector3D) -> Result<f64, String> {
        let mag1 = self.magnitude();
        let mag2 = other.magnitude();
        
        if mag1 == 0.0 || mag2 == 0.0 {
            return Err("Cannot compute angle with zero vector".to_string());
        }
        
        let cos_angle = self.dot(other) / (mag1 * mag2);
        // Clamp to [-1, 1] to handle floating point errors
        let cos_angle = cos_angle.max(-1.0).min(1.0);
        Ok(cos_angle.acos())
    }
}

pub fn compute_dot_product(input: TwoVectorInput) -> Result<DotProductResult, String> {
    let v1 = &input.vector1;
    let v2 = &input.vector2;
    
    let dot_product = v1.dot(v2);
    let are_perpendicular = v1.are_perpendicular(v2);
    let are_parallel = v1.are_parallel(v2);
    
    let (angle_radians, angle_degrees) = if v1.is_zero() || v2.is_zero() {
        (0.0, 0.0)
    } else {
        match v1.angle_with(v2) {
            Ok(angle_rad) => (angle_rad, angle_rad.to_degrees()),
            Err(_) => (0.0, 0.0),
        }
    };
    
    Ok(DotProductResult {
        dot_product,
        angle_radians,
        angle_degrees,
        are_perpendicular,
        are_parallel,
    })
}

pub fn compute_cross_product(input: TwoVectorInput) -> CrossProductResult {
    let v1 = &input.vector1;
    let v2 = &input.vector2;
    
    let cross_product = v1.cross(v2);
    let magnitude = cross_product.magnitude();
    let area_parallelogram = magnitude;
    let are_parallel = v1.are_parallel(v2);
    
    CrossProductResult {
        cross_product,
        magnitude,
        area_parallelogram,
        are_parallel,
    }
}

pub fn compute_vector_magnitude(input: SingleVectorInput) -> Result<VectorMagnitudeResult, String> {
    let vector = &input.vector;
    let magnitude = vector.magnitude();
    let is_zero_vector = vector.is_zero();
    
    let unit_vector = if is_zero_vector {
        Vector3D::new(0.0, 0.0, 0.0)
    } else {
        vector.normalize()?
    };
    
    Ok(VectorMagnitudeResult {
        magnitude,
        unit_vector,
        is_zero_vector,
    })
}

pub fn compute_vector_angle(input: TwoVectorInput) -> Result<VectorAngleResult, String> {
    let v1 = &input.vector1;
    let v2 = &input.vector2;
    
    if v1.is_zero() || v2.is_zero() {
        return Err("Cannot compute angle with zero vector".to_string());
    }
    
    let angle_radians = v1.angle_with(v2)?;
    let angle_degrees = angle_radians.to_degrees();
    let cos_angle = v1.dot(v2) / (v1.magnitude() * v2.magnitude());
    
    Ok(VectorAngleResult {
        angle_radians,
        angle_degrees,
        cos_angle,
    })
}