# 🧮 3D Mathematics Documentation

A comprehensive suite of 25 3D mathematics endpoints providing vector operations, transformations, volume calculations, distance operations, and geometric primitive intersections.

## 🌐 **Overview**

The 3D mathematics tools provide high-performance mathematical operations designed for 3D graphics, robotics, CAD applications, and scientific computing. All operations are validated against reference implementations for mathematical accuracy.

### **Performance Characteristics**
- **Precision**: IEEE standard floating-point arithmetic with microsecond accuracy
- **Speed**: Vectorized operations complete in microseconds
- **Validation**: All operations tested against reference mathematical implementations
- **Accuracy**: Exact mathematical precision for all test cases

## ➗ **Vector Operations (4 endpoints)**

### Dot Product
```bash
POST /3d/dot-product
```
Calculate the dot product of two 3D vectors.

**Input:**
```json
{
  "vector1": {"x": 1.0, "y": 2.0, "z": 3.0},
  "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}
}
```

**Output:**
```json
{
  "dot_product": 32.0,
  "vector1": {"x": 1.0, "y": 2.0, "z": 3.0},
  "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}
}
```

### Cross Product
```bash
POST /3d/cross-product
```
Calculate the cross product of two 3D vectors.

### Vector Magnitude
```bash
POST /3d/vector-magnitude
```
Calculate magnitude and unit vector of a 3D vector.

### Vector Angle
```bash
POST /3d/vector-angle
```
Calculate angle between two vectors in radians and degrees.

## 🔄 **3D Transformations (7 endpoints)**

### Rotation Matrix
```bash
POST /3d/rotation-matrix
```
Create rotation matrix around X, Y, or Z axis.

**Input:**
```json
{
  "axis": "z",
  "angle": 1.5707963267948966
}
```

**Output:**
```json
{
  "matrix": {
    "m00": 0.0, "m01": -1.0, "m02": 0.0,
    "m10": 1.0, "m11": 0.0, "m12": 0.0,
    "m20": 0.0, "m21": 0.0, "m22": 1.0
  }
}
```

### Arbitrary Rotation
```bash
POST /3d/rotation-arbitrary
```
Create rotation matrix around arbitrary axis.

### Quaternion from Axis-Angle
```bash
POST /3d/quaternion-from-axis
```
Create quaternion from axis and angle representation.

### Quaternion Multiplication
```bash
POST /3d/quaternion-multiply
```
Multiply two quaternions for rotation composition.

### Quaternion SLERP
```bash
POST /3d/quaternion-slerp
```
Spherical linear interpolation between two quaternions.

### Matrix-Vector Multiplication
```bash
POST /3d/matrix-vector
```
Apply 3x3 transformation matrix to a vector.

### Coordinate System Conversion
```bash
POST /3d/coordinate-convert
```
Convert between cartesian, spherical, and cylindrical coordinates.

**Input:**
```json
{
  "from_type": "cartesian",
  "to_type": "spherical",
  "coordinates": {"x": 1.0, "y": 1.0, "z": 1.0}
}
```

## 🔍 **Line Operations (6 endpoints)**

### 3D Line Intersection
```bash
POST /3d/line-intersection
```
Find intersection of two infinite 3D lines.

**Input:**
```json
{
  "line1": {
    "point": {"x": 0.0, "y": 0.0, "z": 0.0},
    "direction": {"x": 1.0, "y": 0.0, "z": 0.0}
  },
  "line2": {
    "point": {"x": 1.0, "y": 0.0, "z": 0.0},
    "direction": {"x": 0.0, "y": 1.0, "z": 0.0}
  }
}
```

**Output:**
```json
{
  "intersection_type": "Intersecting",
  "intersection_point": {"x": 1.0, "y": 0.0, "z": 0.0},
  "distance_between_lines": 0.0
}
```

### Line Segment Intersection
```bash
POST /3d/segment-intersection
```
Find intersection of two finite 3D line segments.

### Multi-Line Intersection
```bash
POST /3d/multi-line-intersection
```
Find best intersection point for multiple lines.

### Line-Plane Intersection
```bash
POST /3d/line-plane
```
Find intersection point of line and plane.

### Plane-Plane Intersection
```bash
POST /3d/plane-plane
```
Find intersection line of two planes.

### Point-Plane Distance
```bash
POST /3d/point-plane-distance
```
Calculate distance from point to plane.

## 📏 **Distance Operations (6 endpoints)**

### Point-to-Line Distance
```bash
POST /3d/distance/point-line
```
Calculate shortest distance from point to 3D line.

### Point-to-Plane Distance
```bash
POST /3d/distance/point-plane
```
Calculate perpendicular distance from point to plane.

### Line-to-Plane Distance
```bash
POST /3d/distance/line-plane
```
Calculate distance between parallel line and plane.

### Vector Projection
```bash
POST /3d/projection/vector
```
Project one vector onto another (scalar and vector projections).

### Point Projection on Line
```bash
POST /3d/projection/point-line
```
Project point onto closest point on line.

### Point Projection on Plane
```bash
POST /3d/projection/point-plane
```
Project point onto closest point on plane.

## 📦 **Volume Calculations (6 endpoints)**

### Tetrahedron Volume
```bash
POST /3d/volume/tetrahedron
```
Calculate volume of tetrahedron from 4 points using scalar triple product.

**Input:**
```json
{
  "point_a": {"x": 0.0, "y": 0.0, "z": 0.0},
  "point_b": {"x": 1.0, "y": 0.0, "z": 0.0},
  "point_c": {"x": 0.0, "y": 1.0, "z": 0.0},
  "point_d": {"x": 0.0, "y": 0.0, "z": 1.0}
}
```

**Output:**
```json
{
  "volume": 0.16666666666666666,
  "calculation_method": "Scalar triple product",
  "points": [
    {"x": 0.0, "y": 0.0, "z": 0.0},
    {"x": 1.0, "y": 0.0, "z": 0.0},
    {"x": 0.0, "y": 1.0, "z": 0.0},
    {"x": 0.0, "y": 0.0, "z": 1.0}
  ]
}
```

### Sphere Volume
```bash
POST /3d/volume/sphere
```
Calculate volume of sphere using formula (4/3)πr³.

### Cylinder Volume
```bash
POST /3d/volume/cylinder
```
Calculate volume of cylinder using formula πr²h.

### AABB Volume
```bash
POST /3d/volume/aabb
```
Calculate volume of axis-aligned bounding box.

### Pyramid Volume
```bash
POST /3d/volume/pyramid
```
Calculate volume of pyramid: (1/3) × base_area × height.

### Convex Hull Volume
```bash
POST /3d/volume/convex-hull
```
Calculate volume of convex hull using triangulation.

## 🎯 **Geometric Primitives (5 endpoints)**

### Sphere-Ray Intersection
```bash
POST /3d/primitives/sphere-ray
```
Test intersection between ray and sphere.

### Sphere-Sphere Intersection
```bash
POST /3d/primitives/sphere-sphere
```
Test intersection between two spheres.

### Cylinder-Ray Intersection
```bash
POST /3d/primitives/cylinder-ray
```
Test intersection between ray and cylinder.

### AABB-Ray Intersection
```bash
POST /3d/primitives/aabb-ray
```
Test intersection between ray and axis-aligned bounding box.

### AABB-AABB Intersection
```bash
POST /3d/primitives/aabb-aabb
```
Test intersection between two axis-aligned bounding boxes.

## 🧪 **Mathematical Algorithms**

### Vector Mathematics
- **Dot Product**: Standard Euclidean dot product calculation
- **Cross Product**: Right-hand rule vector cross product
- **Normalization**: Unit vector calculation with magnitude preservation
- **Angle Calculation**: Arc-cosine of normalized dot product

### Transformation Mathematics
- **Rotation Matrices**: Rodrigues' rotation formula for arbitrary axes
- **Quaternions**: Hamilton product and SLERP interpolation
- **Coordinate Systems**: Spherical and cylindrical coordinate transformations
- **Matrix Operations**: Standard linear algebra operations

### Geometric Algorithms
- **Line Intersection**: Parametric line equations with tolerance handling
- **Volume Calculations**: Scalar triple product and geometric formulas
- **Distance Calculations**: Point-to-primitive distance algorithms
- **Intersection Tests**: Geometric intersection algorithms with proper edge case handling

## 🎯 **Use Cases**

### 3D Graphics & Game Development
```bash
# Rotate object around arbitrary axis
POST /3d/rotation-arbitrary

# Interpolate between orientations
POST /3d/quaternion-slerp

# Test ray-object intersection for picking
POST /3d/primitives/sphere-ray
```

### CAD & Manufacturing
```bash
# Calculate volume of manufactured part
POST /3d/volume/convex-hull

# Find intersection of construction lines
POST /3d/line-intersection

# Calculate distances for clearance checks
POST /3d/distance/point-line
```

### Robotics & Automation
```bash
# Transform coordinates between reference frames
POST /3d/coordinate-convert

# Calculate joint rotations
POST /3d/quaternion-from-axis

# Collision detection between robot parts
POST /3d/primitives/aabb-aabb
```

### Scientific Computing
```bash
# Vector analysis
POST /3d/dot-product
POST /3d/cross-product

# Geometric modeling
POST /3d/volume/tetrahedron

# Spatial analysis
POST /3d/projection/point-plane
```

## ⚡ **Performance Benchmarks**

### Accuracy Validation
- **Vector Operations**: Exact mathematical precision for all test cases
- **Rotations**: Validated against reference quaternion implementations
- **Volume Calculations**: Cross-validated with analytical solutions
- **Intersections**: Tested with known geometric configurations

### Speed Benchmarks
- **Vector Operations**: Complete in microseconds
- **Matrix Operations**: Optimized linear algebra routines
- **Complex Calculations**: Volume and intersection tests <1ms
- **Batch Operations**: Efficient processing of multiple geometric queries

## 🚀 **Getting Started**

```bash
# Basic vector operations
curl -X POST http://localhost:3000/3d/dot-product \
  -H "Content-Type: application/json" \
  -d '{"vector1": {"x": 1.0, "y": 2.0, "z": 3.0}, "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}}'

# 3D transformations
curl -X POST http://localhost:3000/3d/rotation-matrix \
  -H "Content-Type: application/json" \
  -d '{"axis": "z", "angle": 1.5707963267948966}'

# Volume calculation
curl -X POST http://localhost:3000/3d/volume/tetrahedron \
  -H "Content-Type: application/json" \
  -d '{
    "point_a": {"x": 0.0, "y": 0.0, "z": 0.0},
    "point_b": {"x": 1.0, "y": 0.0, "z": 0.0},
    "point_c": {"x": 0.0, "y": 1.0, "z": 0.0},
    "point_d": {"x": 0.0, "y": 0.0, "z": 1.0}
  }'
```