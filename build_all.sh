#!/bin/bash

# Core Tools - Unified Build Script
# Builds all tools to WebAssembly targets for Spin deployment

set -e

# Configuration
TARGET="wasm32-wasip1"
BUILD_TYPE="release"
MAX_PARALLEL_JOBS=4
TOOLS_DIR="tools"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to build a single tool
build_tool() {
    local tool_path="$1"
    local tool_name=$(basename "$tool_path")
    local category=$(basename "$(dirname "$tool_path")")
    
    log_info "Building $category/$tool_name..."
    
    # Get package name from Cargo.toml
    local package_name=$(grep '^name = ' "$tool_path/Cargo.toml" | cut -d'"' -f2)
    
    if cargo build -p "$package_name" --target "$TARGET" --"$BUILD_TYPE" 2>/dev/null; then
        log_success "Built $category/$tool_name (package: $package_name)"
        return 0
    else
        log_error "Failed to build $category/$tool_name (package: $package_name)"
        return 1
    fi
}

# Function to find all tool directories
find_tools() {
    find "$TOOLS_DIR" -name "Cargo.toml" -not -path "*/target/*" | while read -r cargo_file; do
        dirname "$cargo_file"
    done | sort
}

# Function to build tools in parallel
build_tools_parallel() {
    local tools=("$@")
    local pids=()
    local results=()
    local job_count=0
    
    for tool in "${tools[@]}"; do
        # Wait for a slot if we've reached max parallel jobs
        if [ $job_count -ge $MAX_PARALLEL_JOBS ]; then
            wait_for_job
            job_count=$((job_count - 1))
        fi
        
        # Start build in background
        (
            build_tool "$tool"
            echo $? > "/tmp/build_result_$(basename "$tool")"
        ) &
        
        pids+=($!)
        job_count=$((job_count + 1))
    done
    
    # Wait for all remaining jobs
    for pid in "${pids[@]}"; do
        wait "$pid"
    done
}

# Function to wait for the first job to complete
wait_for_job() {
    wait -n
}

# Function to check if tools have changed (for CI)
get_changed_tools() {
    local base_ref="${1:-main}"
    
    if ! git rev-parse --verify "$base_ref" >/dev/null 2>&1; then
        log_warning "Base ref '$base_ref' not found, building all tools"
        find_tools
        return
    fi
    
    # Get changed files since base ref
    local changed_files
    changed_files=$(git diff --name-only "$base_ref"...HEAD)
    
    # Find tools that have changed
    local changed_tools=()
    while IFS= read -r file; do
        if [[ "$file" == tools/* ]]; then
            # Extract tool directory from changed file path
            local tool_dir=$(echo "$file" | cut -d'/' -f1-3)
            if [[ -f "$tool_dir/Cargo.toml" ]] && [[ ! " ${changed_tools[@]} " =~ " $tool_dir " ]]; then
                changed_tools+=("$tool_dir")
            fi
        fi
    done <<< "$changed_files"
    
    if [ ${#changed_tools[@]} -eq 0 ]; then
        log_info "No tools changed since $base_ref"
        return
    fi
    
    printf '%s\n' "${changed_tools[@]}"
}

# Function to clean build artifacts
clean_builds() {
    log_info "Cleaning build artifacts..."
    find "$TOOLS_DIR" -name "target" -type d -exec rm -rf {} + 2>/dev/null || true
    log_success "Cleaned build artifacts"
}

# Function to check build requirements
check_requirements() {
    log_info "Checking build requirements..."
    
    # Check if cargo is installed
    if ! command -v cargo &> /dev/null; then
        log_error "Cargo is not installed"
        exit 1
    fi
    
    # Check if wasm32-wasip1 target is installed
    if ! rustup target list --installed | grep -q "$TARGET"; then
        log_warning "Installing $TARGET target..."
        rustup target add "$TARGET"
    fi
    
    log_success "Build requirements satisfied"
}

# Function to display usage
usage() {
    cat << EOF
Core Tools - Unified Build Script

USAGE:
    $0 [OPTIONS] [COMMAND]

COMMANDS:
    build       Build all tools (default)
    clean       Clean build artifacts
    changed     Build only changed tools since main branch
    list        List all available tools
    help        Show this help message

OPTIONS:
    --target TARGET     Set build target (default: $TARGET)
    --debug             Build in debug mode (default: release)
    --jobs N            Set maximum parallel jobs (default: $MAX_PARALLEL_JOBS)
    --base-ref REF      Base reference for changed detection (default: main)

EXAMPLES:
    $0                  # Build all tools in release mode
    $0 --debug          # Build all tools in debug mode
    $0 changed          # Build only changed tools
    $0 clean            # Clean all build artifacts
    $0 --jobs 8 build   # Build with 8 parallel jobs

EOF
}

# Parse command line arguments
COMMAND="build"
BASE_REF="main"

while [[ $# -gt 0 ]]; do
    case $1 in
        --target)
            TARGET="$2"
            shift 2
            ;;
        --debug)
            BUILD_TYPE="debug"
            shift
            ;;
        --jobs)
            MAX_PARALLEL_JOBS="$2"
            shift 2
            ;;
        --base-ref)
            BASE_REF="$2"
            shift 2
            ;;
        build|clean|changed|list|help)
            COMMAND="$1"
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            usage
            exit 1
            ;;
    esac
done

# Main execution
main() {
    local start_time=$(date +%s)
    
    case "$COMMAND" in
        help)
            usage
            exit 0
            ;;
        clean)
            clean_builds
            exit 0
            ;;
        list)
            log_info "Available tools:"
            find_tools | while read -r tool; do
                local category=$(basename "$(dirname "$tool")")
                local name=$(basename "$tool")
                echo "  $category/$name"
            done
            exit 0
            ;;
        changed)
            check_requirements
            log_info "Building changed tools since $BASE_REF..."
            
            mapfile -t tools < <(get_changed_tools "$BASE_REF")
            
            if [ ${#tools[@]} -eq 0 ]; then
                log_info "No tools to build"
                exit 0
            fi
            
            log_info "Changed tools: ${tools[*]}"
            build_tools_parallel "${tools[@]}"
            ;;
        build)
            check_requirements
            log_info "Building all tools..."
            log_info "Target: $TARGET, Mode: $BUILD_TYPE, Parallel jobs: $MAX_PARALLEL_JOBS"
            
            mapfile -t tools < <(find_tools)
            log_info "Found ${#tools[@]} tools to build"
            
            build_tools_parallel "${tools[@]}"
            ;;
        *)
            log_error "Unknown command: $COMMAND"
            usage
            exit 1
            ;;
    esac
    
    local end_time=$(date +%s)
    local duration=$((end_time - start_time))
    
    # Check results
    local failed_tools=()
    for tool in "${tools[@]}"; do
        local tool_name=$(basename "$tool")
        local result_file="/tmp/build_result_$tool_name"
        if [[ -f "$result_file" ]] && [[ $(cat "$result_file") != "0" ]]; then
            failed_tools+=("$tool")
        fi
        rm -f "$result_file" 2>/dev/null || true
    done
    
    # Report results
    local total_tools=${#tools[@]}
    local failed_count=${#failed_tools[@]}
    local success_count=$((total_tools - failed_count))
    
    echo
    log_info "Build Summary:"
    log_success "Successfully built: $success_count/$total_tools tools"
    
    if [ $failed_count -gt 0 ]; then
        log_error "Failed builds: $failed_count"
        for tool in "${failed_tools[@]}"; do
            log_error "  - $tool"
        done
        exit 1
    fi
    
    log_success "All tools built successfully in ${duration}s"
}

# Change to script directory
cd "$(dirname "$0")"

# Run main function
main "$@"