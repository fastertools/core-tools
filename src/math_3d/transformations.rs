use serde::{Deserialize, Serialize};
use crate::math_3d::vector_ops::Vector3D;
use crate::common::ErrorResponse;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Matrix3x3 {
    pub m00: f64, pub m01: f64, pub m02: f64,
    pub m10: f64, pub m11: f64, pub m12: f64,
    pub m20: f64, pub m21: f64, pub m22: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Matrix4x4 {
    pub m00: f64, pub m01: f64, pub m02: f64, pub m03: f64,
    pub m10: f64, pub m11: f64, pub m12: f64, pub m13: f64,
    pub m20: f64, pub m21: f64, pub m22: f64, pub m23: f64,
    pub m30: f64, pub m31: f64, pub m32: f64, pub m33: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SphericalCoord {
    pub radius: f64,
    pub theta: f64,
    pub phi: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct CylindricalCoord {
    pub radius: f64,
    pub theta: f64,
    pub z: f64,
}

#[derive(Deserialize)]
pub struct RotationMatrixInput {
    pub axis: String,
    pub angle: f64,
}

#[derive(Deserialize)]
pub struct ArbitraryRotationInput {
    pub axis: Vector3D,
    pub angle: f64,
}

#[derive(Deserialize)]
pub struct QuaternionFromAxisAngleInput {
    pub axis: Vector3D,
    pub angle: f64,
}

#[derive(Deserialize)]
pub struct QuaternionMultiplyInput {
    pub q1: Quaternion,
    pub q2: Quaternion,
}

#[derive(Deserialize)]
pub struct QuaternionSlerpInput {
    pub q1: Quaternion,
    pub q2: Quaternion,
    pub t: f64,
}

#[derive(Deserialize)]
pub struct MatrixVectorInput {
    pub matrix: Matrix3x3,
    pub vector: Vector3D,
}

#[derive(Deserialize)]
pub struct Matrix4VectorInput {
    pub matrix: Matrix4x4,
    pub vector: Vector3D,
}

#[derive(Deserialize)]
pub struct CoordinateConversionInput {
    pub from_type: String,
    pub to_type: String,
    pub coordinates: Vector3D,
}

#[derive(Serialize)]
pub struct Matrix3x3Response {
    pub matrix: Matrix3x3,
}

#[derive(Serialize)]
pub struct Matrix4x4Response {
    pub matrix: Matrix4x4,
}

#[derive(Serialize)]
pub struct QuaternionResponse {
    pub quaternion: Quaternion,
}

#[derive(Serialize)]
pub struct Vector3DResponse {
    pub vector: Vector3D,
}

#[derive(Serialize)]
pub struct CoordinateConversionResponse {
    pub original: Vector3D,
    pub converted: Vector3D,
    pub from_type: String,
    pub to_type: String,
}

impl Matrix3x3 {
    pub fn identity() -> Self {
        Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    }

    pub fn rotation_x(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: 1.0, m01: 0.0, m02: 0.0,
            m10: 0.0, m11: cos_a, m12: -sin_a,
            m20: 0.0, m21: sin_a, m22: cos_a,
        }
    }

    pub fn rotation_y(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: 0.0, m02: sin_a,
            m10: 0.0, m11: 1.0, m12: 0.0,
            m20: -sin_a, m21: 0.0, m22: cos_a,
        }
    }

    pub fn rotation_z(angle: f64) -> Self {
        let cos_a = angle.cos();
        let sin_a = angle.sin();
        Matrix3x3 {
            m00: cos_a, m01: -sin_a, m02: 0.0,
            m10: sin_a, m11: cos_a, m12: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0,
        }
    }

    pub fn rotation_axis(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        let magnitude = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        let ux = axis.x / magnitude;
        let uy = axis.y / magnitude;
        let uz = axis.z / magnitude;

        let cos_a = angle.cos();
        let sin_a = angle.sin();
        let one_minus_cos = 1.0 - cos_a;

        Ok(Matrix3x3 {
            m00: cos_a + ux * ux * one_minus_cos,
            m01: ux * uy * one_minus_cos - uz * sin_a,
            m02: ux * uz * one_minus_cos + uy * sin_a,
            m10: uy * ux * one_minus_cos + uz * sin_a,
            m11: cos_a + uy * uy * one_minus_cos,
            m12: uy * uz * one_minus_cos - ux * sin_a,
            m20: uz * ux * one_minus_cos - uy * sin_a,
            m21: uz * uy * one_minus_cos + ux * sin_a,
            m22: cos_a + uz * uz * one_minus_cos,
        })
    }

    pub fn multiply_vector(&self, v: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.m00 * v.x + self.m01 * v.y + self.m02 * v.z,
            y: self.m10 * v.x + self.m11 * v.y + self.m12 * v.z,
            z: self.m20 * v.x + self.m21 * v.y + self.m22 * v.z,
        }
    }

    pub fn multiply(&self, other: &Matrix3x3) -> Matrix3x3 {
        Matrix3x3 {
            m00: self.m00 * other.m00 + self.m01 * other.m10 + self.m02 * other.m20,
            m01: self.m00 * other.m01 + self.m01 * other.m11 + self.m02 * other.m21,
            m02: self.m00 * other.m02 + self.m01 * other.m12 + self.m02 * other.m22,
            m10: self.m10 * other.m00 + self.m11 * other.m10 + self.m12 * other.m20,
            m11: self.m10 * other.m01 + self.m11 * other.m11 + self.m12 * other.m21,
            m12: self.m10 * other.m02 + self.m11 * other.m12 + self.m12 * other.m22,
            m20: self.m20 * other.m00 + self.m21 * other.m10 + self.m22 * other.m20,
            m21: self.m20 * other.m01 + self.m21 * other.m11 + self.m22 * other.m21,
            m22: self.m20 * other.m02 + self.m21 * other.m12 + self.m22 * other.m22,
        }
    }

    pub fn determinant(&self) -> f64 {
        self.m00 * (self.m11 * self.m22 - self.m12 * self.m21) -
        self.m01 * (self.m10 * self.m22 - self.m12 * self.m20) +
        self.m02 * (self.m10 * self.m21 - self.m11 * self.m20)
    }

    pub fn inverse(&self) -> Result<Matrix3x3, String> {
        let det = self.determinant();
        if det.abs() < 1e-10 {
            return Err("Matrix is not invertible (determinant is zero)".to_string());
        }

        let inv_det = 1.0 / det;
        Ok(Matrix3x3 {
            m00: (self.m11 * self.m22 - self.m12 * self.m21) * inv_det,
            m01: (self.m02 * self.m21 - self.m01 * self.m22) * inv_det,
            m02: (self.m01 * self.m12 - self.m02 * self.m11) * inv_det,
            m10: (self.m12 * self.m20 - self.m10 * self.m22) * inv_det,
            m11: (self.m00 * self.m22 - self.m02 * self.m20) * inv_det,
            m12: (self.m02 * self.m10 - self.m00 * self.m12) * inv_det,
            m20: (self.m10 * self.m21 - self.m11 * self.m20) * inv_det,
            m21: (self.m01 * self.m20 - self.m00 * self.m21) * inv_det,
            m22: (self.m00 * self.m11 - self.m01 * self.m10) * inv_det,
        })
    }
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Matrix4x4 {
            m00: 1.0, m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: 1.0, m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: 1.0, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn from_matrix3x3(m: &Matrix3x3) -> Self {
        Matrix4x4 {
            m00: m.m00, m01: m.m01, m02: m.m02, m03: 0.0,
            m10: m.m10, m11: m.m11, m12: m.m12, m13: 0.0,
            m20: m.m20, m21: m.m21, m22: m.m22, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        Matrix4x4 {
            m00: 1.0, m01: 0.0, m02: 0.0, m03: x,
            m10: 0.0, m11: 1.0, m12: 0.0, m13: y,
            m20: 0.0, m21: 0.0, m22: 1.0, m23: z,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn scale(x: f64, y: f64, z: f64) -> Self {
        Matrix4x4 {
            m00: x, m01: 0.0, m02: 0.0, m03: 0.0,
            m10: 0.0, m11: y, m12: 0.0, m13: 0.0,
            m20: 0.0, m21: 0.0, m22: z, m23: 0.0,
            m30: 0.0, m31: 0.0, m32: 0.0, m33: 1.0,
        }
    }

    pub fn transform_point(&self, v: &Vector3D) -> Vector3D {
        Vector3D {
            x: self.m00 * v.x + self.m01 * v.y + self.m02 * v.z + self.m03,
            y: self.m10 * v.x + self.m11 * v.y + self.m12 * v.z + self.m13,
            z: self.m20 * v.x + self.m21 * v.y + self.m22 * v.z + self.m23,
        }
    }

    pub fn multiply(&self, other: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            m00: self.m00 * other.m00 + self.m01 * other.m10 + self.m02 * other.m20 + self.m03 * other.m30,
            m01: self.m00 * other.m01 + self.m01 * other.m11 + self.m02 * other.m21 + self.m03 * other.m31,
            m02: self.m00 * other.m02 + self.m01 * other.m12 + self.m02 * other.m22 + self.m03 * other.m32,
            m03: self.m00 * other.m03 + self.m01 * other.m13 + self.m02 * other.m23 + self.m03 * other.m33,
            m10: self.m10 * other.m00 + self.m11 * other.m10 + self.m12 * other.m20 + self.m13 * other.m30,
            m11: self.m10 * other.m01 + self.m11 * other.m11 + self.m12 * other.m21 + self.m13 * other.m31,
            m12: self.m10 * other.m02 + self.m11 * other.m12 + self.m12 * other.m22 + self.m13 * other.m32,
            m13: self.m10 * other.m03 + self.m11 * other.m13 + self.m12 * other.m23 + self.m13 * other.m33,
            m20: self.m20 * other.m00 + self.m21 * other.m10 + self.m22 * other.m20 + self.m23 * other.m30,
            m21: self.m20 * other.m01 + self.m21 * other.m11 + self.m22 * other.m21 + self.m23 * other.m31,
            m22: self.m20 * other.m02 + self.m21 * other.m12 + self.m22 * other.m22 + self.m23 * other.m32,
            m23: self.m20 * other.m03 + self.m21 * other.m13 + self.m22 * other.m23 + self.m23 * other.m33,
            m30: self.m30 * other.m00 + self.m31 * other.m10 + self.m32 * other.m20 + self.m33 * other.m30,
            m31: self.m30 * other.m01 + self.m31 * other.m11 + self.m32 * other.m21 + self.m33 * other.m31,
            m32: self.m30 * other.m02 + self.m31 * other.m12 + self.m32 * other.m22 + self.m33 * other.m32,
            m33: self.m30 * other.m03 + self.m31 * other.m13 + self.m32 * other.m23 + self.m33 * other.m33,
        }
    }
}

impl Quaternion {
    pub fn identity() -> Self {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn from_axis_angle(axis: &Vector3D, angle: f64) -> Result<Self, String> {
        let magnitude = (axis.x * axis.x + axis.y * axis.y + axis.z * axis.z).sqrt();
        if magnitude < 1e-10 {
            return Err("Axis vector cannot be zero".to_string());
        }

        let half_angle = angle * 0.5;
        let sin_half = half_angle.sin();
        let cos_half = half_angle.cos();

        Ok(Quaternion {
            x: (axis.x / magnitude) * sin_half,
            y: (axis.y / magnitude) * sin_half,
            z: (axis.z / magnitude) * sin_half,
            w: cos_half,
        })
    }

    pub fn normalize(&self) -> Result<Self, String> {
        let magnitude = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if magnitude < 1e-10 {
            return Err("Quaternion cannot be zero".to_string());
        }

        Ok(Quaternion {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        })
    }

    pub fn multiply(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }

    pub fn to_rotation_matrix(&self) -> Matrix3x3 {
        let x2 = self.x * self.x;
        let y2 = self.y * self.y;
        let z2 = self.z * self.z;
        let xy = self.x * self.y;
        let xz = self.x * self.z;
        let yz = self.y * self.z;
        let wx = self.w * self.x;
        let wy = self.w * self.y;
        let wz = self.w * self.z;

        Matrix3x3 {
            m00: 1.0 - 2.0 * (y2 + z2), m01: 2.0 * (xy - wz), m02: 2.0 * (xz + wy),
            m10: 2.0 * (xy + wz), m11: 1.0 - 2.0 * (x2 + z2), m12: 2.0 * (yz - wx),
            m20: 2.0 * (xz - wy), m21: 2.0 * (yz + wx), m22: 1.0 - 2.0 * (x2 + y2),
        }
    }

    pub fn rotate_vector(&self, v: &Vector3D) -> Vector3D {
        let matrix = self.to_rotation_matrix();
        matrix.multiply_vector(v)
    }

    pub fn slerp(&self, other: &Quaternion, t: f64) -> Result<Quaternion, String> {
        if t < 0.0 || t > 1.0 {
            return Err("Interpolation parameter t must be between 0 and 1".to_string());
        }

        let dot = self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w;
        
        let (q1, q2) = if dot < 0.0 {
            (self.clone(), Quaternion { x: -other.x, y: -other.y, z: -other.z, w: -other.w })
        } else {
            (self.clone(), other.clone())
        };

        let dot_abs = dot.abs();
        if dot_abs > 0.9995 {
            let result = Quaternion {
                x: q1.x + t * (q2.x - q1.x),
                y: q1.y + t * (q2.y - q1.y),
                z: q1.z + t * (q2.z - q1.z),
                w: q1.w + t * (q2.w - q1.w),
            };
            return result.normalize();
        }

        let theta_0 = dot_abs.acos();
        let sin_theta_0 = theta_0.sin();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        let s0 = cos_theta - dot_abs * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Ok(Quaternion {
            x: s0 * q1.x + s1 * q2.x,
            y: s0 * q1.y + s1 * q2.y,
            z: s0 * q1.z + s1 * q2.z,
            w: s0 * q1.w + s1 * q2.w,
        })
    }
}

pub fn cartesian_to_spherical(v: &Vector3D) -> SphericalCoord {
    let radius = (v.x * v.x + v.y * v.y + v.z * v.z).sqrt();
    let theta = v.y.atan2(v.x);
    let phi = if radius > 0.0 { (v.z / radius).acos() } else { 0.0 };
    
    SphericalCoord { radius, theta, phi }
}

pub fn spherical_to_cartesian(coord: &SphericalCoord) -> Vector3D {
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

pub fn cartesian_to_cylindrical(v: &Vector3D) -> CylindricalCoord {
    let radius = (v.x * v.x + v.y * v.y).sqrt();
    let theta = v.y.atan2(v.x);
    
    CylindricalCoord { radius, theta, z: v.z }
}

pub fn cylindrical_to_cartesian(coord: &CylindricalCoord) -> Vector3D {
    let cos_theta = coord.theta.cos();
    let sin_theta = coord.theta.sin();
    
    Vector3D {
        x: coord.radius * cos_theta,
        y: coord.radius * sin_theta,
        z: coord.z,
    }
}

pub fn handle_rotation_matrix(input: RotationMatrixInput) -> Result<Matrix3x3Response, ErrorResponse> {
    let matrix = match input.axis.to_lowercase().as_str() {
        "x" => Matrix3x3::rotation_x(input.angle),
        "y" => Matrix3x3::rotation_y(input.angle),
        "z" => Matrix3x3::rotation_z(input.angle),
        _ => return Err(ErrorResponse {
            error: "Invalid axis. Use 'x', 'y', or 'z'".to_string(),
        }),
    };

    Ok(Matrix3x3Response { matrix })
}

pub fn handle_arbitrary_rotation(input: ArbitraryRotationInput) -> Result<Matrix3x3Response, ErrorResponse> {
    match Matrix3x3::rotation_axis(&input.axis, input.angle) {
        Ok(matrix) => Ok(Matrix3x3Response { matrix }),
        Err(e) => Err(ErrorResponse { error: e }),
    }
}

pub fn handle_quaternion_from_axis_angle(input: QuaternionFromAxisAngleInput) -> Result<QuaternionResponse, ErrorResponse> {
    match Quaternion::from_axis_angle(&input.axis, input.angle) {
        Ok(quaternion) => Ok(QuaternionResponse { quaternion }),
        Err(e) => Err(ErrorResponse { error: e }),
    }
}

pub fn handle_quaternion_multiply(input: QuaternionMultiplyInput) -> Result<QuaternionResponse, ErrorResponse> {
    let result = input.q1.multiply(&input.q2);
    Ok(QuaternionResponse { quaternion: result })
}

pub fn handle_quaternion_slerp(input: QuaternionSlerpInput) -> Result<QuaternionResponse, ErrorResponse> {
    match input.q1.slerp(&input.q2, input.t) {
        Ok(quaternion) => Ok(QuaternionResponse { quaternion }),
        Err(e) => Err(ErrorResponse { error: e }),
    }
}

pub fn handle_matrix_vector_multiply(input: MatrixVectorInput) -> Result<Vector3DResponse, ErrorResponse> {
    let result = input.matrix.multiply_vector(&input.vector);
    Ok(Vector3DResponse { vector: result })
}

pub fn handle_coordinate_conversion(input: CoordinateConversionInput) -> Result<CoordinateConversionResponse, ErrorResponse> {
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
        _ => return Err(ErrorResponse {
            error: "Invalid coordinate conversion. Supported: cartesian↔spherical, cartesian↔cylindrical".to_string(),
        }),
    };

    Ok(CoordinateConversionResponse {
        original: input.coordinates,
        converted,
        from_type: input.from_type,
        to_type: input.to_type,
    })
}