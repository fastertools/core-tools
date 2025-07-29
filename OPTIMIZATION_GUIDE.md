# WASM Optimization and Size Tracking Guide

This guide explains how to use the new WASM optimization and size tracking system in Core Tools.

## Overview

The system provides:
- Multiple Cargo optimization profiles
- Automated size tracking during builds
- wasm-opt integration with customizable passes
- Experimentation tools for finding optimal settings
- Analysis tools for understanding results

## Quick Start

### Basic Size Tracking

Track sizes while building all tools with default settings:
```bash
./build_all.sh --track-sizes --optimize Oz
```

### Using Custom Profiles

Build with a specific optimization profile:
```bash
./build_all.sh --profile aggressive-opt --track-sizes
```

### Custom wasm-opt Passes

Use custom optimization passes:
```bash
./build_all.sh --track-sizes --optimize custom --opt-passes "--inline-functions --vacuum"
```

## Available Profiles

| Profile | Description | Best For |
|---------|-------------|----------|
| `release` | Standard release build | General use |
| `lean-wasm` | Basic size optimization | Baseline optimization |
| `size-opt` | Optimize for size (opt-level 's') | Size-critical applications |
| `aggressive-opt` | Maximum size reduction | When size is paramount |
| `balanced-opt` | Balance size and speed | Performance-sensitive tools |

## Tracking Individual Tools

Track a single WASM file:
```bash
./track_wasm_sizes.sh \
    --wasm target/wasm32-wasip1/release/tool.wasm \
    --tool category/name \
    --profile release \
    --opt-level Oz \
    --verbose
```

## Running Experiments

Test all optimization combinations:
```bash
./experiment_optimizations.sh
```

Quick test with limited combinations:
```bash
./experiment_optimizations.sh --quick
```

Test specific tools:
```bash
./experiment_optimizations.sh --tools "basic_math/*"
```

## Analyzing Results

Basic analysis:
```bash
./analyze_metrics.sh build_metrics_20241201_120000.tsv
```

Group by profile:
```bash
./analyze_metrics.sh --by-profile metrics.tsv
```

Group by optimization level:
```bash
./analyze_metrics.sh --by-opt metrics.tsv
```

## Output Format

The tracking system outputs TSV files with these columns:
- `tool_name`: Category and name of the tool
- `profile`: Cargo profile used
- `opt_level`: wasm-opt optimization level
- `opt_passes`: Custom wasm-opt passes (if any)
- `original_size`: Size before optimization (bytes)
- `optimized_size`: Size after optimization (bytes)
- `reduction_percent`: Percentage reduction

## Best Practices

1. **Start with Quick Tests**: Use `--quick` mode to identify promising configurations
2. **Focus on Categories**: Different tool categories may benefit from different optimizations
3. **Custom Passes**: Experiment with wasm-opt passes for specific tools that need extra optimization
4. **Track Progress**: Save metrics files to track optimization improvements over time

## Common wasm-opt Passes

- `-Oz`: Optimize for size (recommended starting point)
- `-O1` to `-O4`: Different optimization levels
- `--converge`: Run passes to convergence
- `--inline-functions`: Inline function calls
- `--vacuum`: Remove unnecessary code
- `--strip`: Remove debug info
- `--gufa`: Grand Unified Flow Analysis

## Example Workflow

1. Build with tracking to establish baseline:
   ```bash
   ./build_all.sh --track-sizes --output-metrics baseline.tsv
   ```

2. Run quick experiments:
   ```bash
   ./experiment_optimizations.sh --quick --output-dir experiments/
   ```

3. Analyze results:
   ```bash
   ./analyze_metrics.sh experiments/experiment_results_*.tsv
   ```

4. Apply best settings to specific tools:
   ```bash
   ./build_all.sh --profile aggressive-opt --track-sizes --optimize Oz
   ```

## Tips

- The `aggressive-opt` profile with `-Oz` typically gives the best size reduction
- Custom passes can sometimes outperform standard optimization levels
- Some tools may actually increase in size with certain optimizations
- Always verify that optimized tools still function correctly