#!/bin/bash

# Core Tools - WASM Size Tracking Script
# Analyzes WASM file sizes before and after optimization

set -e

# Configuration
TWIGGY_TOP_COUNT=10
OUTPUT_FORMAT="tsv"  # Tab-separated values for easy parsing

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
}

# Function to display usage
usage() {
    cat << EOF >&2
WASM Size Tracking Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --wasm PATH         Path to the WASM file to analyze (required)
    --tool NAME         Tool name for reporting (required)
    --profile PROFILE   Build profile used (default: release)
    --opt-level LEVEL   Optimization level (none, Oz, O1-O4, custom)
    --opt-passes PASSES Custom wasm-opt passes (use with --opt-level custom)
    --output-dir DIR    Directory to save detailed analysis (optional)
    --no-optimize       Skip optimization, only analyze original
    --verbose           Show detailed output
    --help              Show this help message

EXAMPLES:
    $0 --wasm target/wasm32-wasip1/release/add_tool.wasm --tool add --profile release --opt-level Oz
    $0 --wasm file.wasm --tool test --opt-level custom --opt-passes "--inline-functions --vacuum"

OUTPUT FORMAT:
    TSV with columns: tool_name, profile, opt_level, opt_passes, original_size, optimized_size, reduction_percent

EOF
}

# Parse command line arguments
WASM_PATH=""
TOOL_NAME=""
PROFILE="release"
OPT_LEVEL="none"
OPT_PASSES=""
OUTPUT_DIR=""
NO_OPTIMIZE=false
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --wasm)
            WASM_PATH="$2"
            shift 2
            ;;
        --tool)
            TOOL_NAME="$2"
            shift 2
            ;;
        --profile)
            PROFILE="$2"
            shift 2
            ;;
        --opt-level)
            OPT_LEVEL="$2"
            shift 2
            ;;
        --opt-passes)
            OPT_PASSES="$2"
            shift 2
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        --no-optimize)
            NO_OPTIMIZE=true
            shift
            ;;
        --verbose)
            VERBOSE=true
            shift
            ;;
        --help)
            usage
            exit 0
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Validate required arguments
if [[ -z "$WASM_PATH" ]]; then
    log_error "WASM file path is required"
    usage
    exit 1
fi

if [[ -z "$TOOL_NAME" ]]; then
    log_error "Tool name is required"
    usage
    exit 1
fi

if [[ ! -f "$WASM_PATH" ]]; then
    log_error "WASM file not found: $WASM_PATH"
    exit 1
fi

# Check for required tools
if ! command -v twiggy &> /dev/null; then
    log_error "twiggy is not installed. Install with: cargo install twiggy"
    exit 1
fi

if [[ "$NO_OPTIMIZE" = false ]] && ! command -v wasm-opt &> /dev/null; then
    log_error "wasm-opt is not installed. Install from: https://github.com/WebAssembly/binaryen"
    exit 1
fi

# Function to get file size in bytes
get_file_size() {
    local file="$1"
    if [[ "$OSTYPE" == "darwin"* ]]; then
        stat -f%z "$file"
    else
        stat -c%s "$file"
    fi
}

# Function to format bytes for display
format_bytes() {
    local bytes=$1
    if (( bytes < 1024 )); then
        echo "${bytes}B"
    elif (( bytes < 1048576 )); then
        echo "$(( bytes / 1024 ))KB"
    else
        echo "$(( bytes / 1048576 ))MB"
    fi
}

# Main execution
main() {
    local original_size=$(get_file_size "$WASM_PATH")
    local optimized_size=$original_size
    local reduction_percent=0
    
    if [[ "$VERBOSE" = true ]]; then
        log_info "Analyzing WASM file: $WASM_PATH"
        log_info "Original size: $(format_bytes $original_size) ($original_size bytes)"
    fi
    
    # Create output directory if specified
    if [[ -n "$OUTPUT_DIR" ]]; then
        mkdir -p "$OUTPUT_DIR"
        local timestamp=$(date +%Y%m%d_%H%M%S)
        local analysis_prefix="${OUTPUT_DIR}/${TOOL_NAME}_${timestamp}"
    fi
    
    # Run twiggy analysis on original
    if [[ "$VERBOSE" = true ]] || [[ -n "$OUTPUT_DIR" ]]; then
        if [[ "$VERBOSE" = true ]]; then
            log_info "Running twiggy analysis on original..."
        fi
        
        local twiggy_output=$(twiggy top -n $TWIGGY_TOP_COUNT "$WASM_PATH" 2>&1)
        
        if [[ -n "$OUTPUT_DIR" ]]; then
            echo "$twiggy_output" > "${analysis_prefix}_original_twiggy.txt"
        fi
        
        if [[ "$VERBOSE" = true ]]; then
            echo "$twiggy_output" >&2
        fi
    fi
    
    # Optimize if requested
    if [[ "$NO_OPTIMIZE" = false ]] && [[ "$OPT_LEVEL" != "none" ]]; then
        local opt_output="${WASM_PATH}.opt"
        local opt_command="wasm-opt"
        
        # Build optimization command
        case "$OPT_LEVEL" in
            Oz)
                opt_command="$opt_command -Oz"
                ;;
            O1)
                opt_command="$opt_command -O1"
                ;;
            O2)
                opt_command="$opt_command -O2"
                ;;
            O3)
                opt_command="$opt_command -O3"
                ;;
            O4)
                opt_command="$opt_command -O4"
                ;;
            custom)
                if [[ -z "$OPT_PASSES" ]]; then
                    log_error "Custom optimization requires --opt-passes"
                    exit 1
                fi
                opt_command="$opt_command $OPT_PASSES"
                ;;
            *)
                log_error "Unknown optimization level: $OPT_LEVEL"
                exit 1
                ;;
        esac
        
        opt_command="$opt_command \"$WASM_PATH\" -o \"$opt_output\""
        
        if [[ "$VERBOSE" = true ]]; then
            log_info "Running optimization: $opt_command"
        fi
        
        # Execute optimization
        if eval "$opt_command" 2>&1; then
            optimized_size=$(get_file_size "$opt_output")
            
            # Calculate reduction
            if (( original_size > 0 )); then
                reduction_percent=$(awk "BEGIN {printf \"%.2f\", (($original_size - $optimized_size) / $original_size) * 100}")
            fi
            
            if [[ "$VERBOSE" = true ]]; then
                log_success "Optimized size: $(format_bytes $optimized_size) ($optimized_size bytes)"
                log_success "Size reduction: ${reduction_percent}%"
            fi
            
            # Run twiggy analysis on optimized
            if [[ "$VERBOSE" = true ]] || [[ -n "$OUTPUT_DIR" ]]; then
                if [[ "$VERBOSE" = true ]]; then
                    log_info "Running twiggy analysis on optimized..."
                fi
                
                local twiggy_opt_output=$(twiggy top -n $TWIGGY_TOP_COUNT "$opt_output" 2>&1)
                
                if [[ -n "$OUTPUT_DIR" ]]; then
                    echo "$twiggy_opt_output" > "${analysis_prefix}_optimized_twiggy.txt"
                    # Also save the optimized WASM
                    cp "$opt_output" "${analysis_prefix}_optimized.wasm"
                fi
                
                if [[ "$VERBOSE" = true ]]; then
                    echo "$twiggy_opt_output" >&2
                fi
            fi
            
            # Clean up temporary optimized file
            rm -f "$opt_output"
        else
            log_error "Optimization failed"
            exit 1
        fi
    fi
    
    # Output TSV data
    # Format: tool_name profile opt_level opt_passes original_size optimized_size reduction_percent
    local opt_passes_display="${OPT_PASSES:-none}"
    printf "%s\t%s\t%s\t%s\t%d\t%d\t%.2f\n" \
        "$TOOL_NAME" \
        "$PROFILE" \
        "$OPT_LEVEL" \
        "$opt_passes_display" \
        "$original_size" \
        "$optimized_size" \
        "$reduction_percent"
}

# Run main function
main "$@"