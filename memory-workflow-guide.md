# Memory and Segment-Based Development Workflow Guide - Core Tools Project

## Overview

This project uses a dual memory system for maintaining context and organizing development work:

1. **MCP Server Memory** - Persistent knowledge graph for cross-session continuity
2. **Segment Files** - Structured development planning and status tracking

## MCP Server Memory System

### Purpose
The MCP server memory maintains a persistent knowledge graph that survives across Claude sessions, storing project entities, their relationships, and observations.

### Key Memory Tools Available
```
mcp__server-memory__create_entities     - Add new entities to knowledge graph
mcp__server-memory__create_relations    - Connect entities with relationships  
mcp__server-memory__add_observations    - Add information to existing entities
mcp__server-memory__read_graph          - View entire knowledge graph
mcp__server-memory__search_nodes        - Find entities by query
mcp__server-memory__open_nodes          - Get specific entities and relations
mcp__server-memory__delete_entities     - Remove entities (use sparingly)
```

### When to Use Memory Tools

#### CREATE ENTITIES for:
- **New architectural patterns** (e.g., MCP_Dispatch_Architecture, FTL_SDK_Pattern)
- **Major tool categories** (e.g., Geospatial_Tools_Suite, 3D_Mathematics_Suite)
- **Implementation milestones** (e.g., Component_Discovery_System, Tool_Router)
- **Framework changes** (e.g., Spin_to_FTL_Migration)
- **Performance benchmarks** (e.g., specific accuracy or throughput metrics)

#### CREATE RELATIONS to show:
- **Dependencies** (Dispatcher → depends_on → Tool_Components)
- **Implementation hierarchy** (Suite → implements → Individual_Tools)
- **Architectural evolution** (FTL_Pattern → replaces → Spin_Architecture)
- **Composition relationships** (Dispatch_System → routes_to → Tool_Suites)

#### ADD OBSERVATIONS for:
- **Implementation details** that might be forgotten
- **Performance metrics** and benchmark results (99.8% accuracy, 500K ops/sec)
- **Architectural decisions** and their rationale
- **Integration patterns** and best practices
- **Migration progress** and lessons learned

### Memory Usage Patterns

#### Session Start Pattern
```
1. mcp__server-memory__read_graph (get full context)
2. mcp__server-memory__search_nodes "current project" (find relevant entities)
3. Review memory context before making changes
```

#### Development Progress Pattern
```
1. mcp__server-memory__add_observations (document progress on existing entities)
2. mcp__server-memory__create_entities (for major new components)
3. mcp__server-memory__create_relations (connect new work to existing structure)
```

#### Session End Pattern
```
1. mcp__server-memory__add_observations (capture session accomplishments)
2. Document any new architectural insights or patterns discovered
3. Update relations if project structure evolved
```

## Segment Files System

### Core Segment Files

#### 1. `optimized-segments.md` - Primary Development Queue
**Purpose**: Prioritized, actionable development segments with clear status tracking

**Structure**:
- **Priority levels**: 1 (highest) to 4 (future roadmap)
- **Status tracking**: NOT_STARTED, IN_PROGRESS, COMPLETED, BLOCKED
- **Clear directives**: What needs to be implemented
- **Success criteria**: How to know when complete
- **Dependencies**: What must be done first

#### 2. `Master Checklist.md` - Comprehensive Project Status
**Purpose**: High-level project milestone tracking across all categories

**Structure**:
- **Foundation & Infrastructure** (MCP integration, build system)
- **Core Tool Categories** (geospatial ✅, 3D math ✅, statistics)
- **Dispatch Architecture** (component discovery, routing, error handling)
- **FTL SDK Migration** (tool conversion, pattern implementation)

#### 3. `Master Codebase.md` - Implementation Status Documentation
**Purpose**: Detailed tracking of file-level implementation status and architectural decisions

**Structure**:
- **File Structure Status** (implemented ✅, planned 🔄, future 📋)
- **API Endpoints** (grouped by category with implementation status)
- **Data Structures** (core types and their usage)
- **Implementation Patterns** (established conventions)

## Integrated Workflow: Memory + Segments

### Starting a Development Session

1. **Review Memory Context**:
   ```
   mcp__server-memory__read_graph
   mcp__server-memory__search_nodes "Core_Tools_Project"
   ```

2. **Check Segment Status**:
   - Read `optimized-segments.md` to find highest priority NOT_STARTED item
   - Review dependencies in `Master Checklist.md`
   - Check implementation details in `Master Codebase.md`

3. **Update Working Status**:
   - Mark chosen segment as IN_PROGRESS in `optimized-segments.md`
   - Add memory observation about starting work on this segment

### During Development

1. **Track Progress in Segments**:
   - Update `optimized-segments.md` with implementation progress
   - Update file status in `Master Codebase.md` as files are created
   - Check off items in `Master Checklist.md` as milestones are reached

2. **Capture Knowledge in Memory**:
   - Add observations about implementation challenges/solutions
   - Create entities for significant new components
   - Document architectural insights as they emerge

### Completing Work

1. **Verify Completion**:
   - Check all success criteria in `optimized-segments.md`
   - Confirm relevant items checked in `Master Checklist.md`
   - Verify file status updated in `Master Codebase.md`

2. **Update Memory**:
   - Add completion observations to relevant entities
   - Create relations to connect completed work with overall project
   - Document any new patterns or insights discovered

## Current Project Memory Context

### Key Entities Established:
- **Core_Tools_Project** - Root project entity
- **Geospatial_Tools_Suite** - 13+ endpoints, 99.8% accuracy, 500K ops/sec
- **3D_Mathematics_Suite** - 13+ endpoints, 5-module architecture
- **Statistical_Analysis_Suite** - Descriptive stats, correlation, regression
- **MCP_Dispatch_Architecture** - Dynamic component discovery and routing
- **FTL_SDK_Pattern** - Target migration framework

### Priority Development Areas:
1. **FTL SDK Migration** - Convert from wasmcp to FTL SDK pattern
2. **Component Discovery** - Implement dynamic tool discovery mechanism
3. **Dispatch Router** - Create unified MCP endpoint with internal routing
4. **Tool Composition** - Enable tools to call each other internally

## Development Commands Reference

### Build and Test
```bash
cargo build --target wasm32-wasip1  # Build WASM components
cargo test                         # Run integration tests
spin build                        # Build Spin application
spin up                          # Start local server
```

### Tool Testing Examples
```bash
# Test distance calculation
curl -X POST http://127.0.0.1:3000/distance \
  -H "Content-Type: application/json" \
  -d '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}'

# Test 3D dot product
curl -X POST http://127.0.0.1:3000/3d/dot-product \
  -H "Content-Type: application/json" \
  -d '{"vector1": {"x": 1.0, "y": 2.0, "z": 3.0}, "vector2": {"x": 4.0, "y": 5.0, "z": 6.0}}'
```

This integrated system ensures **persistent knowledge retention** (memory) and **structured development planning** (segments) work together for consistent cross-session productivity in the core-tools project.