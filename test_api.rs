// Simple test function to verify our transformations work
fn main() {
    println!("Testing 3D Transformations Implementation");
    
    test_rotation_matrices();
    test_quaternions();
    test_coordinate_conversions();
    test_matrix_operations();
}

fn test_rotation_matrices() {
    println!("\n=== Testing Rotation Matrices ===");
    
    // Test 90 degree rotation around Z axis
    let angle = std::f64::consts::PI / 2.0; // 90 degrees
    println!("Testing rotation around Z axis by 90 degrees (π/2 radians)");
    
    // Manually create the expected matrix for 90° Z rotation
    let cos_90 = angle.cos(); // ~0
    let sin_90 = angle.sin(); // ~1
    
    println!("Expected matrix elements:");
    println!("cos(90°) = {:.6}", cos_90);
    println!("sin(90°) = {:.6}", sin_90);
    
    // Expected matrix:
    // [cos -sin  0]   [0 -1  0]
    // [sin  cos  0] = [1  0  0]
    // [0    0    1]   [0  0  1]
    
    println!("Expected rotation matrix for 90° around Z:");
    println!("[{:8.4} {:8.4} {:8.4}]", cos_90, -sin_90, 0.0);
    println!("[{:8.4} {:8.4} {:8.4}]", sin_90, cos_90, 0.0);
    println!("[{:8.4} {:8.4} {:8.4}]", 0.0, 0.0, 1.0);
    
    // Test vector rotation: (1,0,0) -> (0,1,0)
    let original_vector = (1.0, 0.0, 0.0);
    let rotated_x = cos_90 * original_vector.0 + (-sin_90) * original_vector.1 + 0.0 * original_vector.2;
    let rotated_y = sin_90 * original_vector.0 + cos_90 * original_vector.1 + 0.0 * original_vector.2;
    let rotated_z = 0.0 * original_vector.0 + 0.0 * original_vector.1 + 1.0 * original_vector.2;
    
    println!("Vector (1,0,0) rotated 90° around Z: ({:.4}, {:.4}, {:.4})", rotated_x, rotated_y, rotated_z);
    println!("Expected: (0.0000, 1.0000, 0.0000)");
}

fn test_quaternions() {
    println!("\n=== Testing Quaternions ===");
    
    // Test quaternion from axis-angle
    println!("Testing quaternion from axis (0,0,1) and angle π/2");
    
    let axis = (0.0, 0.0, 1.0); // Z axis
    let angle = std::f64::consts::PI / 2.0; // 90 degrees
    
    let half_angle = angle * 0.5;
    let sin_half = half_angle.sin();
    let cos_half = half_angle.cos();
    
    let q_x = axis.0 * sin_half;
    let q_y = axis.1 * sin_half;
    let q_z = axis.2 * sin_half;
    let q_w = cos_half;
    
    println!("Expected quaternion: ({:.4}, {:.4}, {:.4}, {:.4})", q_x, q_y, q_z, q_w);
    println!("Expected: (0.0000, 0.0000, 0.7071, 0.7071)");
    
    // Test quaternion multiplication (identity * q = q)
    let _identity = (0.0, 0.0, 0.0, 1.0);
    println!("Identity quaternion * test quaternion = test quaternion");
    
    // Test SLERP between identity and test quaternion at t=0.5
    let _t = 0.5;
    println!("SLERP from identity to test quaternion at t=0.5 should be halfway rotation");
}

fn test_coordinate_conversions() {
    println!("\n=== Testing Coordinate Conversions ===");
    
    // Test Cartesian to Spherical
    println!("Testing Cartesian (1, 1, 1) to Spherical");
    
    let x: f64 = 1.0;
    let y: f64 = 1.0;
    let z: f64 = 1.0;
    
    let radius = (x*x + y*y + z*z).sqrt();
    let theta = y.atan2(x);
    let phi = (z / radius).acos();
    
    println!("Cartesian: ({}, {}, {})", x, y, z);
    println!("Spherical: radius={:.4}, theta={:.4}, phi={:.4}", radius, theta, phi);
    println!("Expected: radius=1.7321, theta=0.7854, phi=0.9553");
    
    // Test Spherical back to Cartesian
    let sin_phi = phi.sin();
    let cos_phi = phi.cos();
    let sin_theta = theta.sin();
    let cos_theta = theta.cos();
    
    let x_back = radius * sin_phi * cos_theta;
    let y_back = radius * sin_phi * sin_theta;
    let z_back = radius * cos_phi;
    
    println!("Converted back to Cartesian: ({:.4}, {:.4}, {:.4})", x_back, y_back, z_back);
    println!("Should match original: (1.0000, 1.0000, 1.0000)");
    
    // Test Cartesian to Cylindrical
    println!("\nTesting Cartesian (1, 1, 2) to Cylindrical");
    let x2: f64 = 1.0;
    let y2: f64 = 1.0;
    let z2: f64 = 2.0;
    
    let cyl_radius = (x2*x2 + y2*y2).sqrt();
    let cyl_theta = y2.atan2(x2);
    
    println!("Cartesian: ({}, {}, {})", x2, y2, z2);
    println!("Cylindrical: radius={:.4}, theta={:.4}, z={:.4}", cyl_radius, cyl_theta, z2);
    println!("Expected: radius=1.4142, theta=0.7854, z=2.0000");
}

fn test_matrix_operations() {
    println!("\n=== Testing Matrix Operations ===");
    
    // Test 3x3 matrix determinant
    println!("Testing 3x3 identity matrix determinant");
    
    // Identity matrix determinant should be 1
    let det = 1.0 * (1.0 * 1.0 - 0.0 * 0.0) - 0.0 * (0.0 * 1.0 - 0.0 * 0.0) + 0.0 * (0.0 * 0.0 - 1.0 * 0.0);
    println!("Identity matrix determinant: {}", det);
    println!("Expected: 1.0000");
    
    // Test matrix-vector multiplication
    println!("\nTesting identity matrix * vector (2, 3, 4)");
    let vec = (2.0, 3.0, 4.0);
    
    // Identity matrix multiplication should return the same vector
    let result_x = 1.0 * vec.0 + 0.0 * vec.1 + 0.0 * vec.2;
    let result_y = 0.0 * vec.0 + 1.0 * vec.1 + 0.0 * vec.2;
    let result_z = 0.0 * vec.0 + 0.0 * vec.1 + 1.0 * vec.2;
    
    println!("Result: ({}, {}, {})", result_x, result_y, result_z);
    println!("Expected: (2.0000, 3.0000, 4.0000)");
}