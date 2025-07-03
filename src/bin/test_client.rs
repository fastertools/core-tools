use std::collections::HashMap;
use serde_json::{json, Value};

// Simple HTTP client for testing our API endpoints
fn main() {
    println!("3D Transformations API Test Client");
    println!("==================================");
    
    let base_url = "http://127.0.0.1:3001";
    
    // Test API info endpoint
    test_api_info(&base_url);
    
    // Test transformation endpoints
    test_rotation_matrix(&base_url);
    test_arbitrary_rotation(&base_url);
    test_quaternion_operations(&base_url);
    test_coordinate_conversions(&base_url);
    test_matrix_vector_operations(&base_url);
}

fn make_http_request(url: &str, method: &str, body: Option<&str>) -> Result<String, String> {
    let client = std::process::Command::new("curl")
        .arg("-s") // silent
        .arg("-X")
        .arg(method)
        .arg("-H")
        .arg("Content-Type: application/json")
        .args(if body.is_some() { vec!["-d", body.unwrap()] } else { vec![] })
        .arg(url)
        .output();
    
    match client {
        Ok(output) => {
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).to_string())
            } else {
                Err(format!("HTTP request failed: {}", String::from_utf8_lossy(&output.stderr)))
            }
        }
        Err(e) => Err(format!("Failed to execute curl: {}", e))
    }
}

fn test_api_info(base_url: &str) {
    println!("\n🔍 Testing API Info Endpoint");
    println!("GET {}/", base_url);
    
    match make_http_request(base_url, "GET", None) {
        Ok(response) => {
            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                println!("✅ API Info Response:");
                if let Some(service) = json_response.get("service") {
                    println!("   Service: {}", service);
                }
                if let Some(endpoints) = json_response.get("endpoints") {
                    if let Some(obj) = endpoints.as_object() {
                        let transform_endpoints: Vec<_> = obj.keys()
                            .filter(|k| k.contains("rotation") || k.contains("quaternion") || k.contains("matrix") || k.contains("coordinate"))
                            .collect();
                        println!("   New Transform Endpoints: {}", transform_endpoints.len());
                        for endpoint in transform_endpoints {
                            println!("     - {}", endpoint);
                        }
                    }
                }
            } else {
                println!("❌ Failed to parse JSON response");
            }
        }
        Err(e) => println!("❌ Request failed: {}", e)
    }
}

fn test_rotation_matrix(base_url: &str) {
    println!("\n🔄 Testing Rotation Matrix Generation");
    
    let test_cases = vec![
        ("X", 1.5708, "90° around X-axis"),
        ("Y", 1.5708, "90° around Y-axis"), 
        ("Z", 1.5708, "90° around Z-axis"),
        ("Z", 3.14159, "180° around Z-axis"),
    ];
    
    for (axis, angle, description) in test_cases {
        println!("\n📐 Test: {}", description);
        let url = format!("{}/3d/rotation-matrix", base_url);
        let body = json!({
            "axis": axis,
            "angle": angle
        }).to_string();
        
        match make_http_request(&url, "POST", Some(&body)) {
            Ok(response) => {
                if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                    if let Some(matrix) = json_response.get("matrix") {
                        println!("✅ Rotation Matrix:");
                        print_3x3_matrix(matrix);
                    } else {
                        println!("❌ No matrix in response: {}", response);
                    }
                } else {
                    println!("❌ Failed to parse response: {}", response);
                }
            }
            Err(e) => println!("❌ Request failed: {}", e)
        }
    }
}

fn test_arbitrary_rotation(base_url: &str) {
    println!("\n🎯 Testing Arbitrary Axis Rotation");
    
    let url = format!("{}/3d/rotation-arbitrary", base_url);
    let body = json!({
        "axis": {"x": 1.0, "y": 1.0, "z": 0.0},
        "angle": 1.5708
    }).to_string();
    
    println!("📐 Test: 90° rotation around axis (1,1,0)");
    
    match make_http_request(&url, "POST", Some(&body)) {
        Ok(response) => {
            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                if let Some(matrix) = json_response.get("matrix") {
                    println!("✅ Arbitrary Rotation Matrix:");
                    print_3x3_matrix(matrix);
                } else {
                    println!("❌ No matrix in response: {}", response);
                }
            } else {
                println!("❌ Failed to parse response: {}", response);
            }
        }
        Err(e) => println!("❌ Request failed: {}", e)
    }
}

fn test_quaternion_operations(base_url: &str) {
    println!("\n🌀 Testing Quaternion Operations");
    
    // Test quaternion from axis-angle
    println!("\n📐 Test: Quaternion from axis (0,0,1) angle π/2");
    let url = format!("{}/3d/quaternion-from-axis", base_url);
    let body = json!({
        "axis": {"x": 0.0, "y": 0.0, "z": 1.0},
        "angle": 1.5708
    }).to_string();
    
    let q1 = match make_http_request(&url, "POST", Some(&body)) {
        Ok(response) => {
            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                if let Some(quaternion) = json_response.get("quaternion") {
                    println!("✅ Quaternion: {}", quaternion);
                    Some(quaternion.clone())
                } else {
                    println!("❌ No quaternion in response: {}", response);
                    None
                }
            } else {
                println!("❌ Failed to parse response: {}", response);
                None
            }
        }
        Err(e) => {
            println!("❌ Request failed: {}", e);
            None
        }
    };
    
    // Test quaternion multiplication if we got a quaternion
    if let Some(q1_val) = q1 {
        println!("\n🔄 Test: Quaternion multiplication (q * identity)");
        let url = format!("{}/3d/quaternion-multiply", base_url);
        let body = json!({
            "q1": q1_val,
            "q2": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}
        }).to_string();
        
        match make_http_request(&url, "POST", Some(&body)) {
            Ok(response) => {
                if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                    if let Some(quaternion) = json_response.get("quaternion") {
                        println!("✅ Result: {}", quaternion);
                    } else {
                        println!("❌ No quaternion in response: {}", response);
                    }
                } else {
                    println!("❌ Failed to parse response: {}", response);
                }
            }
            Err(e) => println!("❌ Request failed: {}", e)
        }
        
        // Test SLERP
        println!("\n🌈 Test: SLERP from identity to quaternion (t=0.5)");
        let url = format!("{}/3d/quaternion-slerp", base_url);
        let body = json!({
            "q1": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0},
            "q2": q1_val,
            "t": 0.5
        }).to_string();
        
        match make_http_request(&url, "POST", Some(&body)) {
            Ok(response) => {
                if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                    if let Some(quaternion) = json_response.get("quaternion") {
                        println!("✅ SLERP Result: {}", quaternion);
                    } else {
                        println!("❌ No quaternion in response: {}", response);
                    }
                } else {
                    println!("❌ Failed to parse response: {}", response);
                }
            }
            Err(e) => println!("❌ Request failed: {}", e)
        }
    }
}

fn test_coordinate_conversions(base_url: &str) {
    println!("\n🗺️  Testing Coordinate Conversions");
    
    let test_cases = vec![
        ("cartesian", "spherical", json!({"x": 1.0, "y": 1.0, "z": 1.0}), "Cartesian to Spherical"),
        ("cartesian", "cylindrical", json!({"x": 1.0, "y": 1.0, "z": 2.0}), "Cartesian to Cylindrical"),
        ("spherical", "cartesian", json!({"x": 1.732, "y": 0.785, "z": 0.955}), "Spherical to Cartesian"),
        ("cylindrical", "cartesian", json!({"x": 1.414, "y": 0.785, "z": 2.0}), "Cylindrical to Cartesian"),
    ];
    
    for (from_type, to_type, coordinates, description) in test_cases {
        println!("\n📐 Test: {}", description);
        let url = format!("{}/3d/coordinate-convert", base_url);
        let body = json!({
            "from_type": from_type,
            "to_type": to_type,
            "coordinates": coordinates
        }).to_string();
        
        match make_http_request(&url, "POST", Some(&body)) {
            Ok(response) => {
                if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                    if let Some(original) = json_response.get("original") {
                        if let Some(converted) = json_response.get("converted") {
                            println!("✅ {} → {}", original, converted);
                        } else {
                            println!("❌ No converted coordinates: {}", response);
                        }
                    } else {
                        println!("❌ No original coordinates: {}", response);
                    }
                } else {
                    println!("❌ Failed to parse response: {}", response);
                }
            }
            Err(e) => println!("❌ Request failed: {}", e)
        }
    }
}

fn test_matrix_vector_operations(base_url: &str) {
    println!("\n🧮 Testing Matrix-Vector Operations");
    
    println!("\n📐 Test: Identity matrix * vector (2,3,4)");
    let url = format!("{}/3d/matrix-vector", base_url);
    let body = json!({
        "matrix": {
            "m00": 1.0, "m01": 0.0, "m02": 0.0,
            "m10": 0.0, "m11": 1.0, "m12": 0.0,
            "m20": 0.0, "m21": 0.0, "m22": 1.0
        },
        "vector": {"x": 2.0, "y": 3.0, "z": 4.0}
    }).to_string();
    
    match make_http_request(&url, "POST", Some(&body)) {
        Ok(response) => {
            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                if let Some(vector) = json_response.get("vector") {
                    println!("✅ Result: {}", vector);
                } else {
                    println!("❌ No vector in response: {}", response);
                }
            } else {
                println!("❌ Failed to parse response: {}", response);
            }
        }
        Err(e) => println!("❌ Request failed: {}", e)
    }
    
    // Test with a 90-degree Z rotation matrix
    println!("\n📐 Test: 90° Z rotation matrix * vector (1,0,0)");
    let body = json!({
        "matrix": {
            "m00": 0.0, "m01": -1.0, "m02": 0.0,
            "m10": 1.0, "m11": 0.0, "m12": 0.0,
            "m20": 0.0, "m21": 0.0, "m22": 1.0
        },
        "vector": {"x": 1.0, "y": 0.0, "z": 0.0}
    }).to_string();
    
    match make_http_request(&url, "POST", Some(&body)) {
        Ok(response) => {
            if let Ok(json_response) = serde_json::from_str::<Value>(&response) {
                if let Some(vector) = json_response.get("vector") {
                    println!("✅ Result: {} (should be ~(0,1,0))", vector);
                } else {
                    println!("❌ No vector in response: {}", response);
                }
            } else {
                println!("❌ Failed to parse response: {}", response);
            }
        }
        Err(e) => println!("❌ Request failed: {}", e)
    }
}

fn print_3x3_matrix(matrix: &Value) {
    if let Some(obj) = matrix.as_object() {
        let get_val = |key: &str| obj.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0);
        
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m00"), get_val("m01"), get_val("m02"));
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m10"), get_val("m11"), get_val("m12"));
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m20"), get_val("m21"), get_val("m22"));
    }
}