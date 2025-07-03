// Test the volume calculation functions manually

fn main() {
    println!("Testing 3D Volume Calculations");
    println!("==============================");
    
    test_tetrahedron_volume();
    test_sphere_volume();
    test_cylinder_volume();
    test_aabb_volume();
    test_pyramid_volume();
}

fn test_tetrahedron_volume() {
    println!("\n📐 Testing Tetrahedron Volume");
    
    // Unit cube vertices: (0,0,0), (1,0,0), (0,1,0), (0,0,1)
    let a = (0.0, 0.0, 0.0);
    let b = (1.0, 0.0, 0.0);
    let c = (0.0, 1.0, 0.0);
    let d = (0.0, 0.0, 1.0);
    
    // Calculate vectors from point A to the other three points
    let ab = (b.0 - a.0, b.1 - a.1, b.2 - a.2); // (1,0,0)
    let ac = (c.0 - a.0, c.1 - a.1, c.2 - a.2); // (0,1,0)
    let ad = (d.0 - a.0, d.1 - a.1, d.2 - a.2); // (0,0,1)
    
    // Cross product AC × AD
    let cross_ac_ad = (
        ac.1 * ad.2 - ac.2 * ad.1, // 1*1 - 0*0 = 1
        ac.2 * ad.0 - ac.0 * ad.2, // 0*0 - 0*1 = 0
        ac.0 * ad.1 - ac.1 * ad.0, // 0*0 - 1*0 = 0
    );
    
    // Scalar triple product: AB · (AC × AD)
    let scalar_triple: f64 = ab.0 * cross_ac_ad.0 + ab.1 * cross_ac_ad.1 + ab.2 * cross_ac_ad.2;
    // = 1*1 + 0*0 + 0*0 = 1
    
    let volume = scalar_triple.abs() / 6.0;
    
    println!("Tetrahedron vertices:");
    println!("  A: {:?}", a);
    println!("  B: {:?}", b);
    println!("  C: {:?}", c);
    println!("  D: {:?}", d);
    println!("Vectors from A:");
    println!("  AB: {:?}", ab);
    println!("  AC: {:?}", ac);
    println!("  AD: {:?}", ad);
    println!("Cross product AC × AD: {:?}", cross_ac_ad);
    println!("Scalar triple product: {}", scalar_triple);
    println!("Volume: {} cubic units", volume);
    println!("Expected: 1/6 = 0.1667 cubic units");
}

fn test_sphere_volume() {
    println!("\n🌍 Testing Sphere Volume");
    
    let radius: f64 = 2.0;
    let volume = (4.0 / 3.0) * std::f64::consts::PI * radius.powi(3);
    
    println!("Sphere radius: {}", radius);
    println!("Volume: {} cubic units", volume);
    println!("Expected: (4/3)π×8 = {} cubic units", (4.0 / 3.0) * std::f64::consts::PI * 8.0);
}

fn test_cylinder_volume() {
    println!("\n🛢️  Testing Cylinder Volume");
    
    let radius: f64 = 2.0;
    let height: f64 = 5.0;
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    
    println!("Cylinder radius: {}, height: {}", radius, height);
    println!("Volume: {} cubic units", volume);
    println!("Expected: π×4×5 = {} cubic units", std::f64::consts::PI * 20.0);
}

fn test_aabb_volume() {
    println!("\n📦 Testing AABB Volume");
    
    // Points forming a cube from (1,1,1) to (4,3,5)
    let points = vec![
        (1.0, 1.0, 1.0),
        (4.0, 3.0, 5.0),
        (2.0, 2.0, 3.0), // random interior point
    ];
    
    let (mut min_x, mut max_x): (f64, f64) = (points[0].0, points[0].0);
    let (mut min_y, mut max_y): (f64, f64) = (points[0].1, points[0].1);
    let (mut min_z, mut max_z): (f64, f64) = (points[0].2, points[0].2);
    
    for point in &points {
        min_x = min_x.min(point.0);
        max_x = max_x.max(point.0);
        min_y = min_y.min(point.1);
        max_y = max_y.max(point.1);
        min_z = min_z.min(point.2);
        max_z = max_z.max(point.2);
    }
    
    let dimensions = (max_x - min_x, max_y - min_y, max_z - min_z);
    let volume = dimensions.0 * dimensions.1 * dimensions.2;
    
    println!("AABB points: {:?}", points);
    println!("Min: ({}, {}, {})", min_x, min_y, min_z);
    println!("Max: ({}, {}, {})", max_x, max_y, max_z);
    println!("Dimensions: {:?}", dimensions);
    println!("Volume: {} cubic units", volume);
    println!("Expected: 3×2×4 = 24 cubic units");
}

fn test_pyramid_volume() {
    println!("\n🔺 Testing Pyramid Volume");
    
    // Square base in XY plane, apex above
    let base_points = vec![
        (0.0, 0.0, 0.0),
        (2.0, 0.0, 0.0),
        (2.0, 2.0, 0.0),
        (0.0, 2.0, 0.0),
    ];
    let apex = (1.0, 1.0, 3.0);
    
    // Base area (square): 2×2 = 4
    let base_area = 4.0;
    
    // Height: distance from apex to base plane (Z=0)
    let height = 3.0;
    
    let volume = (1.0 / 3.0) * base_area * height;
    
    println!("Pyramid base points: {:?}", base_points);
    println!("Apex: {:?}", apex);
    println!("Base area: {} square units", base_area);
    println!("Height: {} units", height);
    println!("Volume: {} cubic units", volume);
    println!("Expected: (1/3)×4×3 = 4 cubic units");
}