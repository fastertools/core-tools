use coretools::{
    DistanceTool, DotProductTool, DescriptiveStatsTool, CoreToolsDispatcher
};
use serde_json::json;
use wasmcp::ToolHandler;

#[test]
fn test_distance_tool() {
    let input = json!({
        "lat1": 40.7128,
        "lon1": -74.0060,
        "lat2": 34.0522,
        "lon2": -118.2437
    });
    
    let result = DistanceTool::execute(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // NYC to LA distance should be around 3944 km
    assert!(parsed["distance_km"].as_f64().unwrap() > 3900.0);
    assert!(parsed["distance_km"].as_f64().unwrap() < 4000.0);
}

#[test]
fn test_dot_product_tool() {
    let input = json!({
        "vector1": {"x": 1.0, "y": 2.0, "z": 3.0},
        "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}
    });
    
    let result = DotProductTool::execute(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // 1*4 + 2*5 + 3*6 = 4 + 10 + 18 = 32
    assert_eq!(parsed["dot_product"].as_f64().unwrap(), 32.0);
}

#[test]
fn test_descriptive_stats_tool() {
    let input = json!({
        "data": [1.0, 2.0, 3.0, 4.0, 5.0]
    });
    
    let result = DescriptiveStatsTool::execute(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Mean of 1,2,3,4,5 should be 3.0
    assert_eq!(parsed["mean"].as_f64().unwrap(), 3.0);
    assert_eq!(parsed["count"].as_u64().unwrap(), 5);
}

#[test]
fn test_core_tools_dispatcher() {
    let input = json!({
        "tool": "distance",
        "args": {
            "lat1": 40.7128,
            "lon1": -74.0060,
            "lat2": 34.0522,
            "lon2": -118.2437
        }
    });
    
    let result = CoreToolsDispatcher::execute(input).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&result).unwrap();
    
    // Should dispatch to DistanceTool and return the same result
    assert!(parsed["distance_km"].as_f64().unwrap() > 3900.0);
    assert!(parsed["distance_km"].as_f64().unwrap() < 4000.0);
}

#[test]
fn test_tool_input_schemas() {
    // Test that all tools have valid input schemas
    let distance_schema = DistanceTool::input_schema();
    assert_eq!(distance_schema["type"], "object");
    assert!(distance_schema["properties"]["lat1"].is_object());
    
    let dot_product_schema = DotProductTool::input_schema();
    assert_eq!(dot_product_schema["type"], "object");
    assert!(dot_product_schema["properties"]["vector1"].is_object());
    
    let stats_schema = DescriptiveStatsTool::input_schema();
    assert_eq!(stats_schema["type"], "object");
    assert!(stats_schema["properties"]["data"].is_object());
    
    let dispatcher_schema = CoreToolsDispatcher::input_schema();
    assert_eq!(dispatcher_schema["type"], "object");
    assert!(dispatcher_schema["properties"]["tool"]["enum"].is_array());
}

#[test]
fn test_tool_names_and_descriptions() {
    // Verify all tools have proper names and descriptions
    assert_eq!(DistanceTool::NAME, "distance");
    assert!(DistanceTool::DESCRIPTION.contains("GPS coordinates"));
    
    assert_eq!(DotProductTool::NAME, "dot_product_3d");
    assert!(DotProductTool::DESCRIPTION.contains("3D vectors"));
    
    assert_eq!(DescriptiveStatsTool::NAME, "descriptive_stats");
    assert!(DescriptiveStatsTool::DESCRIPTION.contains("statistics"));
    
    assert_eq!(CoreToolsDispatcher::NAME, "coretools");
    assert!(CoreToolsDispatcher::DESCRIPTION.contains("computational tools"));
}