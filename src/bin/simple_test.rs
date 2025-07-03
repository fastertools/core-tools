use std::io::{Read, Write};
use std::net::TcpStream;
use serde_json::json;

fn main() {
    println!("Simple HTTP Test Client for 3D Transformations API");
    println!("================================================");
    
    // Test a few key endpoints
    test_rotation_matrix();
    test_quaternion_from_axis();
    test_coordinate_conversion();
}

fn make_http_request(host: &str, path: &str, method: &str, body: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(host)?;
    
    let content_length = body.map_or(0, |b| b.len());
    let request = if method == "GET" {
        format!(
            "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            path, host.split(':').next().unwrap_or("localhost")
        )
    } else {
        format!(
            "POST {} HTTP/1.1\r\nHost: {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            path,
            host.split(':').next().unwrap_or("localhost"),
            content_length,
            body.unwrap_or("")
        )
    };
    
    stream.write_all(request.as_bytes())?;
    
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    
    // Extract the body (after double CRLF)
    if let Some(body_start) = response.find("\r\n\r\n") {
        Ok(response[body_start + 4..].to_string())
    } else {
        Ok(response)
    }
}

fn test_rotation_matrix() {
    println!("\n🔄 Testing Rotation Matrix (90° around Z-axis)");
    
    let body = json!({
        "axis": "z",
        "angle": 1.5708
    }).to_string();
    
    match make_http_request("127.0.0.1:3001", "/3d/rotation-matrix", "POST", Some(&body)) {
        Ok(response) => {
            println!("✅ Response: {}", response.trim());
            
            // Parse and display the matrix nicely
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(matrix) = json.get("matrix") {
                    println!("📐 Rotation Matrix:");
                    print_matrix(matrix);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e)
    }
}

fn test_quaternion_from_axis() {
    println!("\n🌀 Testing Quaternion from Axis-Angle");
    
    let body = json!({
        "axis": {"x": 0.0, "y": 0.0, "z": 1.0},
        "angle": 1.5708
    }).to_string();
    
    match make_http_request("127.0.0.1:3001", "/3d/quaternion-from-axis", "POST", Some(&body)) {
        Ok(response) => {
            println!("✅ Response: {}", response.trim());
            
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(quat) = json.get("quaternion") {
                    println!("🌀 Quaternion (x,y,z,w): {}", quat);
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e)
    }
}

fn test_coordinate_conversion() {
    println!("\n🗺️  Testing Coordinate Conversion (Cartesian to Spherical)");
    
    let body = json!({
        "from_type": "cartesian",
        "to_type": "spherical", 
        "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}
    }).to_string();
    
    match make_http_request("127.0.0.1:3001", "/3d/coordinate-convert", "POST", Some(&body)) {
        Ok(response) => {
            println!("✅ Response: {}", response.trim());
            
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response) {
                if let Some(original) = json.get("original") {
                    if let Some(converted) = json.get("converted") {
                        println!("📍 Cartesian {} → Spherical {}", original, converted);
                    }
                }
            }
        }
        Err(e) => println!("❌ Error: {}", e)
    }
}

fn print_matrix(matrix: &serde_json::Value) {
    if let Some(obj) = matrix.as_object() {
        let get_val = |key: &str| obj.get(key).and_then(|v| v.as_f64()).unwrap_or(0.0);
        
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m00"), get_val("m01"), get_val("m02"));
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m10"), get_val("m11"), get_val("m12"));
        println!("   [{:8.4} {:8.4} {:8.4}]", get_val("m20"), get_val("m21"), get_val("m22"));
    }
}