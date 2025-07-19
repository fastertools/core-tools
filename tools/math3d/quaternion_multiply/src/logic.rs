use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionMultiplyInput {
    pub q1: Quaternion,
    pub q2: Quaternion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuaternionMultiplyResponse {
    pub result: Quaternion,
}

impl Quaternion {
    pub fn multiply(&self, other: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * other.x + self.x * other.w + self.y * other.z - self.z * other.y,
            y: self.w * other.y - self.x * other.z + self.y * other.w + self.z * other.x,
            z: self.w * other.z + self.x * other.y - self.y * other.x + self.z * other.w,
            w: self.w * other.w - self.x * other.x - self.y * other.y - self.z * other.z,
        }
    }
}

pub fn compute_quaternion_multiply(
    input: QuaternionMultiplyInput,
) -> Result<QuaternionMultiplyResponse, String> {
    // Validate q1 for NaN and infinite values
    if input.q1.x.is_nan() || input.q1.y.is_nan() || input.q1.z.is_nan() || input.q1.w.is_nan() {
        return Err("Quaternion q1 contains NaN values".to_string());
    }
    if input.q1.x.is_infinite()
        || input.q1.y.is_infinite()
        || input.q1.z.is_infinite()
        || input.q1.w.is_infinite()
    {
        return Err("Quaternion q1 contains infinite values".to_string());
    }

    // Validate q2 for NaN and infinite values
    if input.q2.x.is_nan() || input.q2.y.is_nan() || input.q2.z.is_nan() || input.q2.w.is_nan() {
        return Err("Quaternion q2 contains NaN values".to_string());
    }
    if input.q2.x.is_infinite()
        || input.q2.y.is_infinite()
        || input.q2.z.is_infinite()
        || input.q2.w.is_infinite()
    {
        return Err("Quaternion q2 contains infinite values".to_string());
    }

    let result = input.q1.multiply(&input.q2);

    Ok(QuaternionMultiplyResponse { result })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }, // Identity
            q2: Quaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_multiplication_by_identity() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
            q2: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }, // Identity
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            w: 4.0,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_zero_quaternion_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            q2: Quaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_i_j_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }, // i
            q2: Quaternion {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            }, // j
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }; // k
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_j_k_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            }, // j
            q2: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            }, // k
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }; // i
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_k_i_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            }, // k
            q2: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }, // i
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }; // j
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_i_squared() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }, // i
            q2: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            }, // i
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: -1.0,
        }; // -1
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_j_squared() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            }, // j
            q2: Quaternion {
                x: 0.0,
                y: 1.0,
                z: 0.0,
                w: 0.0,
            }, // j
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: -1.0,
        }; // -1
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_k_squared() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            }, // k
            q2: Quaternion {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            }, // k
        };
        let result = compute_quaternion_multiply(input).unwrap();
        let expected = Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: -1.0,
        }; // -1
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_general_multiplication() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
            q2: Quaternion {
                x: 5.0,
                y: 6.0,
                z: 7.0,
                w: 8.0,
            },
        };
        let result = compute_quaternion_multiply(input).unwrap();
        // Calculated manually:
        // x: w1*x2 + x1*w2 + y1*z2 - z1*y2 = 4*5 + 1*8 + 2*7 - 3*6 = 20 + 8 + 14 - 18 = 24
        // y: w1*y2 - x1*z2 + y1*w2 + z1*x2 = 4*6 - 1*7 + 2*8 + 3*5 = 24 - 7 + 16 + 15 = 48
        // z: w1*z2 + x1*y2 - y1*x2 + z1*w2 = 4*7 + 1*6 - 2*5 + 3*8 = 28 + 6 - 10 + 24 = 48
        // w: w1*w2 - x1*x2 - y1*y2 - z1*z2 = 4*8 - 1*5 - 2*6 - 3*7 = 32 - 5 - 12 - 21 = -6
        let expected = Quaternion {
            x: 24.0,
            y: 48.0,
            z: 48.0,
            w: -6.0,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_multiplication_non_commutative() {
        let q1 = Quaternion {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }; // i
        let q2 = Quaternion {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }; // j

        let input1 = QuaternionMultiplyInput {
            q1: q1.clone(),
            q2: q2.clone(),
        };
        let result1 = compute_quaternion_multiply(input1).unwrap();

        let input2 = QuaternionMultiplyInput { q1: q2, q2: q1 };
        let result2 = compute_quaternion_multiply(input2).unwrap();

        // i * j = k, but j * i = -k
        assert_quaternion_eq(
            &result1.result,
            &Quaternion {
                x: 0.0,
                y: 0.0,
                z: 1.0,
                w: 0.0,
            },
            1e-15,
        );
        assert_quaternion_eq(
            &result2.result,
            &Quaternion {
                x: 0.0,
                y: 0.0,
                z: -1.0,
                w: 0.0,
            },
            1e-15,
        );
    }

    #[test]
    fn test_negative_values() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: -1.0,
                y: -2.0,
                z: -3.0,
                w: -4.0,
            },
            q2: Quaternion {
                x: 1.0,
                y: 2.0,
                z: 3.0,
                w: 4.0,
            },
        };
        let result = compute_quaternion_multiply(input).unwrap();
        // Calculated manually:
        // x: w1*x2 + x1*w2 + y1*z2 - z1*y2 = -4*1 + -1*4 + -2*3 - -3*2 = -4 - 4 - 6 + 6 = -8
        // y: w1*y2 - x1*z2 + y1*w2 + z1*x2 = -4*2 - -1*3 + -2*4 + -3*1 = -8 + 3 - 8 - 3 = -16
        // z: w1*z2 + x1*y2 - y1*x2 + z1*w2 = -4*3 + -1*2 - -2*1 + -3*4 = -12 - 2 + 2 - 12 = -24
        // w: w1*w2 - x1*x2 - y1*y2 - z1*z2 = -4*4 - -1*1 - -2*2 - -3*3 = -16 + 1 + 4 + 9 = -2
        let expected = Quaternion {
            x: -8.0,
            y: -16.0,
            z: -24.0,
            w: -2.0,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_unit_quaternion_multiplication() {

        // Two 90-degree rotations around different axes
        let sqrt2_inv = 1.0 / 2.0_f64.sqrt();
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: sqrt2_inv,
                y: 0.0,
                z: 0.0,
                w: sqrt2_inv,
            }, // 90° around X
            q2: Quaternion {
                x: 0.0,
                y: sqrt2_inv,
                z: 0.0,
                w: sqrt2_inv,
            }, // 90° around Y
        };
        let result = compute_quaternion_multiply(input).unwrap();

        // Result should still be a unit quaternion
        let magnitude_squared = result.result.x.powi(2)
            + result.result.y.powi(2)
            + result.result.z.powi(2)
            + result.result.w.powi(2);
        assert!((magnitude_squared - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_fractional_values() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 0.5,
                y: 0.5,
                z: 0.5,
                w: 0.5,
            },
            q2: Quaternion {
                x: 0.5,
                y: -0.5,
                z: 0.5,
                w: -0.5,
            },
        };
        let result = compute_quaternion_multiply(input).unwrap();
        // Calculated manually:
        // x: w1*x2 + x1*w2 + y1*z2 - z1*y2 = 0.5*0.5 + 0.5*-0.5 + 0.5*0.5 - 0.5*-0.5 = 0.25 - 0.25 + 0.25 + 0.25 = 0.5
        // y: w1*y2 - x1*z2 + y1*w2 + z1*x2 = 0.5*-0.5 - 0.5*0.5 + 0.5*-0.5 + 0.5*0.5 = -0.25 - 0.25 - 0.25 + 0.25 = -0.5
        // z: w1*z2 + x1*y2 - y1*x2 + z1*w2 = 0.5*0.5 + 0.5*-0.5 - 0.5*0.5 + 0.5*-0.5 = 0.25 - 0.25 - 0.25 - 0.25 = -0.5
        // w: w1*w2 - x1*x2 - y1*y2 - z1*z2 = 0.5*-0.5 - 0.5*0.5 - 0.5*-0.5 - 0.5*0.5 = -0.25 - 0.25 + 0.25 - 0.25 = -0.5
        let expected = Quaternion {
            x: 0.5,
            y: -0.5,
            z: -0.5,
            w: -0.5,
        };
        assert_quaternion_eq(&result.result, &expected, 1e-15);
    }

    #[test]
    fn test_nan_q1_error() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: f64::NAN,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            },
            q2: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
        };
        let result = compute_quaternion_multiply(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("q1"));
    }

    #[test]
    fn test_infinite_q2_error() {
        let input = QuaternionMultiplyInput {
            q1: Quaternion {
                x: 1.0,
                y: 0.0,
                z: 0.0,
                w: 0.0,
            },
            q2: Quaternion {
                x: 0.0,
                y: f64::INFINITY,
                z: 0.0,
                w: 1.0,
            },
        };
        let result = compute_quaternion_multiply(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("q2"));
    }

    // Helper function to compare quaternions with tolerance
    fn assert_quaternion_eq(actual: &Quaternion, expected: &Quaternion, tolerance: f64) {
        assert!(
            (actual.x - expected.x).abs() < tolerance,
            "x: {} != {}",
            actual.x,
            expected.x
        );
        assert!(
            (actual.y - expected.y).abs() < tolerance,
            "y: {} != {}",
            actual.y,
            expected.y
        );
        assert!(
            (actual.z - expected.z).abs() < tolerance,
            "z: {} != {}",
            actual.z,
            expected.z
        );
        assert!(
            (actual.w - expected.w).abs() < tolerance,
            "w: {} != {}",
            actual.w,
            expected.w
        );
    }
}
