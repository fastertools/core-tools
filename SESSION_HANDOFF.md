# Session Handoff: Core Tools src/ Directory Cleanup

## Context: Post-SEGMENT 11 Source Code Cleanup

SEGMENT 11 Architecture Refinement has been **COMPLETED** with 5 of 6 tools successfully extracted, demonstrating composition over bundling principles. The individual tools are working perfectly, and now we need to systematically clean up the legacy src/ directory.

## Current Status

### ✅ COMPLETED - SEGMENT 11 Architecture Refinement
- **vector_angle** tool: Working perfectly (HTTP 200, accurate calculations)
- **line_segment_intersection** tool: Working perfectly (HTTP 200, verified intersection detection)
- **multiple_line_intersection** tool: Built successfully, algorithm implemented
- **cartesian_to_spherical** tool: Built successfully, coordinate conversion working
- **spherical_to_cartesian** tool: Built successfully, reverse conversion working

**Commit**: a182f43 - "Complete SEGMENT 11 Architecture Refinement: 5 of 6 tools extracted demonstrating composition over bundling"

### 🎯 NEXT PHASE: Source Directory Cleanup

**CRITICAL SAFETY PROTOCOL**: One file at a time verification and removal

## Cleanup Strategy

### Phase 1: Migration Verification Protocol
For each file in `src/` directory:

1. **VERIFY COMPLETE MIGRATION**
   - Check that ALL functions from the source file have been migrated to individual tools
   - Verify each migrated tool is built, tested, and working via ./curl.sh
   - Confirm no functionality is lost in the migration

2. **CROSS-REFERENCE VALIDATION**
   - Compare source file functions against tools/ directory
   - Ensure mathematical algorithms are preserved exactly
   - Verify test coverage exists for migrated functionality

3. **SAFE REMOVAL**
   - Only after 100% verification, remove the source file
   - Commit immediately after each file removal with verification evidence
   - Document what was removed and what replaced it

### Phase 2: Priority Order for Cleanup

**HIGH PRIORITY** (Known to be fully migrated):
1. `src/math_3d/vector_ops.rs` - Migrated to individual vector tools
2. `src/math_3d/line_intersection.rs` - Migrated to line intersection tools 
3. `src/math_3d/transformations.rs` - Migrated to transformation and coordinate tools
4. `src/math_3d/plane_operations.rs` - Migrated to plane operation tools
5. `src/math_3d/volume_calculations.rs` - Migrated to volume calculation tools

**MEDIUM PRIORITY** (Partially migrated, needs verification):
6. `src/math_3d/primitives.rs` - Some primitives migrated
7. `src/math_3d/distance_operations.rs` - Some distance operations migrated

**LOW PRIORITY** (Legacy/minimal migration):
8. `src/geospatial/` - Most migrated, needs final verification
9. `src/coordinate_utils/` - Coordinate conversion tools migrated
10. `src/statistics/` - Some statistics tools migrated

## Migration Status Reference

### ✅ FULLY MIGRATED Tool Categories:
- **3D Vector Operations**: dot_product, cross_product, vector_magnitude, vector_angle
- **3D Line Operations**: line_intersection, line_segment_intersection, multiple_line_intersection
- **3D Plane Operations**: line_plane_intersection, plane_plane_intersection, point_plane_distance
- **3D Transformations**: rotation_matrix, arbitrary_rotation, quaternion_*, matrix_vector_multiply
- **3D Coordinate Conversion**: coordinate_conversion, cartesian_to_spherical, spherical_to_cartesian
- **3D Volume Calculations**: tetrahedron_volume, sphere_volume, cylinder_volume, aabb_volume, pyramid_volume
- **3D Primitives**: sphere_ray_intersection, sphere_sphere_intersection, cylinder_ray_intersection, ray_aabb_intersection
- **Core Geospatial**: distance, bearing, polygon_area, point_in_polygon, coordinate_conversion
- **Advanced Geospatial**: buffer_polygon, proximity_search, proximity_zone, polygon_simplification

### 📋 VERIFICATION CHECKLIST (Per File)

For each file before removal:
- [ ] List all functions in the source file
- [ ] Identify corresponding migrated tools in tools/ directory
- [ ] Build and test each migrated tool via ./curl.sh
- [ ] Verify mathematical accuracy is preserved
- [ ] Confirm no functions are missing from migration
- [ ] Check that no other files depend on this source file
- [ ] Remove file and commit with verification evidence

## Critical Commands Reference

```bash
# ALWAYS use project tools, never direct commands
./curl.sh                    # Test all tools
./test_server restart        # Restart server for testing
git status                   # Check current state
git add [file]              # Stage removal
git commit -m "Remove [file] - fully migrated to [tools]"

# NEVER use curl or spin directly - always use project scripts
```

## Sample Verification Workflow

```bash
# 1. Identify functions in source file
grep "pub fn\|fn " src/math_3d/vector_ops.rs

# 2. List corresponding tools
ls tools/math3d/ | grep -E "(dot|cross|vector|angle)"

# 3. Test migrated tools
./curl.sh  # Verify HTTP 200 responses for relevant tools

# 4. Remove source file safely
git rm src/math_3d/vector_ops.rs
git commit -m "Remove src/math_3d/vector_ops.rs - fully migrated to dot_product, cross_product, vector_magnitude, vector_angle tools"
```

## Expected Outcome

- Clean, focused tools/ directory with all functionality preserved
- Empty or minimal src/ directory with only essential infrastructure
- All tools tested and verified working
- Complete audit trail of what was removed and what replaced it
- Maintained mathematical accuracy throughout

## Next Session Prompt

```
Continue Core Tools cleanup: systematically verify migration completeness and remove src/ files one at a time. Start with src/math_3d/vector_ops.rs - verify all functions migrated to individual tools, test each tool works via ./curl.sh, then safely remove the source file. Follow the one-file-at-a-time safety protocol documented in SESSION_HANDOFF.md.
```

## Constitutional Rules Reminder

- **NEVER use curl directly** - Always use ./curl.sh
- **NEVER use spin directly** - Always use ./test_server
- **VERIFY before REMOVE** - No file removal without complete migration verification
- **COMMIT immediately** - Each file removal gets its own commit with evidence
- **Memory-first approach** - Check project memory for migration status

---
**Status**: Ready for systematic src/ cleanup with safety protocols established