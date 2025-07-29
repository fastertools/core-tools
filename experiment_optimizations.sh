#!/bin/bash

# Core Tools - Optimization Experimentation Script
# Tests different combinations of Cargo profiles and wasm-opt settings

set -e

# Configuration
TOOLS_DIR="tools"
OUTPUT_DIR="optimization_experiments"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
RESULTS_FILE="${OUTPUT_DIR}/experiment_results_${TIMESTAMP}.tsv"
SUMMARY_FILE="${OUTPUT_DIR}/experiment_summary_${TIMESTAMP}.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
MAGENTA='\033[0;35m'
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

log_experiment() {
    echo -e "${MAGENTA}[EXPERIMENT]${NC} $1"
}

# Function to display usage
usage() {
    cat << EOF
Core Tools - Optimization Experimentation Script

USAGE:
    $0 [OPTIONS]

OPTIONS:
    --tools PATTERN     Glob pattern for tools to test (default: all)
    --profiles LIST     Comma-separated list of Cargo profiles to test
                       (default: release,lean-wasm,size-opt,aggressive-opt,balanced-opt)
    --opt-levels LIST   Comma-separated list of wasm-opt levels to test
                       (default: none,Oz,O1,O2,O3,O4)
    --custom-passes     Include custom wasm-opt pass experiments
    --quick            Quick mode - test fewer combinations
    --output-dir DIR   Directory for results (default: $OUTPUT_DIR)
    --help             Show this help message

EXAMPLES:
    $0                                    # Test all combinations
    $0 --quick                           # Quick test with limited combinations
    $0 --tools "basic_math/*"            # Test only basic math tools
    $0 --profiles release,lean-wasm      # Test specific profiles

OUTPUT:
    - Detailed TSV results in: experiment_results_TIMESTAMP.tsv
    - Summary report in: experiment_summary_TIMESTAMP.txt
    - Best configurations for each tool category

EOF
}

# Parse command line arguments
TOOL_PATTERN=""
PROFILES="release,lean-wasm,size-opt,aggressive-opt,balanced-opt"
OPT_LEVELS="none,Oz,O1,O2,O3,O4"
INCLUDE_CUSTOM=false
QUICK_MODE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --tools)
            TOOL_PATTERN="$2"
            shift 2
            ;;
        --profiles)
            PROFILES="$2"
            shift 2
            ;;
        --opt-levels)
            OPT_LEVELS="$2"
            shift 2
            ;;
        --custom-passes)
            INCLUDE_CUSTOM=true
            shift
            ;;
        --quick)
            QUICK_MODE=true
            PROFILES="release,lean-wasm,aggressive-opt"
            OPT_LEVELS="none,Oz,O2"
            shift
            ;;
        --output-dir)
            OUTPUT_DIR="$2"
            RESULTS_FILE="${OUTPUT_DIR}/experiment_results_${TIMESTAMP}.tsv"
            SUMMARY_FILE="${OUTPUT_DIR}/experiment_summary_${TIMESTAMP}.txt"
            shift 2
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

# Custom wasm-opt pass combinations to test
CUSTOM_PASSES=(
    "--converge --gufa -tnh"
    "--inline-functions-with-loops --simplify-locals --vacuum"
    "--merge-blocks --remove-unused-names --strip"
    "--coalesce-locals --reorder-locals --merge-locals --vacuum"
    "--flatten --simplify-locals --vacuum --strip"
)

# Function to find tools
find_tools() {
    local pattern="${1:-*}"
    find "$TOOLS_DIR" -path "*/$pattern/Cargo.toml" -not -path "*/target/*" | while read -r cargo_file; do
        dirname "$cargo_file"
    done | sort
}

# Function to get tool category
get_category() {
    local tool_path="$1"
    basename "$(dirname "$tool_path")"
}

# Function to analyze results
analyze_results() {
    log_info "Analyzing experiment results..."
    
    {
        echo "==================== OPTIMIZATION EXPERIMENT SUMMARY ===================="
        echo "Timestamp: $(date)"
        echo "Total experiments: $(tail -n +2 "$RESULTS_FILE" | wc -l)"
        echo ""
        
        # Find best configuration for each tool
        echo "=== BEST CONFIGURATIONS BY TOOL ==="
        echo ""
        
        # Get unique tools
        local tools=$(tail -n +2 "$RESULTS_FILE" | cut -f1 | sort -u)
        
        while IFS= read -r tool; do
            echo "Tool: $tool"
            # Find configuration with smallest optimized size for this tool
            local best=$(grep "^$tool" "$RESULTS_FILE" | sort -t$'\t' -k6 -n | head -1)
            if [[ -n "$best" ]]; then
                local profile=$(echo "$best" | cut -f2)
                local opt_level=$(echo "$best" | cut -f3)
                local opt_passes=$(echo "$best" | cut -f4)
                local original=$(echo "$best" | cut -f5)
                local optimized=$(echo "$best" | cut -f6)
                local reduction=$(echo "$best" | cut -f7)
                
                echo "  Best config: Profile=$profile, Opt=$opt_level"
                if [[ "$opt_passes" != "none" ]]; then
                    echo "  Custom passes: $opt_passes"
                fi
                echo "  Size: $original -> $optimized bytes (${reduction}% reduction)"
                echo ""
            fi
        done <<< "$tools"
        
        # Category analysis
        echo "=== BEST CONFIGURATIONS BY CATEGORY ==="
        echo ""
        
        # Get categories and their best average reduction
        local categories=$(tail -n +2 "$RESULTS_FILE" | while IFS=$'\t' read -r tool rest; do
            echo "${tool%/*}"
        done | sort -u)
        
        while IFS= read -r category; do
            echo "Category: $category"
            
            # Calculate average reduction for each profile/opt combination in this category
            local best_config=""
            local best_avg_reduction=0
            
            # Get all unique profile/opt combinations
            grep "^$category/" "$RESULTS_FILE" | while IFS=$'\t' read -r tool profile opt_level opt_passes original optimized reduction; do
                echo "$profile|$opt_level"
            done | sort -u | while IFS='|' read -r profile opt_level; do
                # Calculate average reduction for this combination
                local sum=0
                local count=0
                while IFS=$'\t' read -r tool p o op orig opt red; do
                    if [[ "$p" == "$profile" ]] && [[ "$o" == "$opt_level" ]]; then
                        sum=$(awk "BEGIN {print $sum + $red}")
                        count=$((count + 1))
                    fi
                done < <(grep "^$category/" "$RESULTS_FILE")
                
                if [[ $count -gt 0 ]]; then
                    local avg=$(awk "BEGIN {printf \"%.2f\", $sum / $count}")
                    echo "  $profile + $opt_level: avg ${avg}% reduction"
                fi
            done
            echo ""
        done <<< "$categories"
        
        # Overall statistics
        echo "=== OVERALL STATISTICS ==="
        echo ""
        
        # Best profile overall
        echo "Average reduction by profile:"
        for profile in ${PROFILES//,/ }; do
            local sum=0
            local count=0
            while IFS=$'\t' read -r tool p o op orig opt red; do
                if [[ "$p" == "$profile" ]]; then
                    sum=$(awk "BEGIN {print $sum + $red}")
                    count=$((count + 1))
                fi
            done < <(tail -n +2 "$RESULTS_FILE")
            
            if [[ $count -gt 0 ]]; then
                local avg=$(awk "BEGIN {printf \"%.2f\", $sum / $count}")
                echo "  $profile: ${avg}%"
            fi
        done
        echo ""
        
        # Best optimization level overall
        echo "Average reduction by optimization level:"
        for opt_level in ${OPT_LEVELS//,/ }; do
            local sum=0
            local count=0
            while IFS=$'\t' read -r tool p o op orig opt red; do
                if [[ "$o" == "$opt_level" ]]; then
                    sum=$(awk "BEGIN {print $sum + $red}")
                    count=$((count + 1))
                fi
            done < <(tail -n +2 "$RESULTS_FILE")
            
            if [[ $count -gt 0 ]]; then
                local avg=$(awk "BEGIN {printf \"%.2f\", $sum / $count}")
                echo "  $opt_level: ${avg}%"
            fi
        done
        
    } > "$SUMMARY_FILE"
    
    log_success "Analysis complete. Summary saved to: $SUMMARY_FILE"
}

# Main execution
main() {
    log_info "Starting optimization experiments..."
    
    # Create output directory
    mkdir -p "$OUTPUT_DIR"
    
    # Initialize results file with header
    echo -e "tool_name\tprofile\topt_level\topt_passes\toriginal_size\toptimized_size\treduction_percent" > "$RESULTS_FILE"
    
    # Find tools to test
    local tools
    if [[ -n "$TOOL_PATTERN" ]]; then
        mapfile -t tools < <(find_tools "$TOOL_PATTERN")
    else
        mapfile -t tools < <(find_tools)
    fi
    
    if [[ ${#tools[@]} -eq 0 ]]; then
        log_error "No tools found matching pattern: ${TOOL_PATTERN:-*}"
        exit 1
    fi
    
    log_info "Found ${#tools[@]} tools to test"
    
    # Count total experiments
    local profile_count=$(echo "$PROFILES" | tr ',' ' ' | wc -w)
    local opt_count=$(echo "$OPT_LEVELS" | tr ',' ' ' | wc -w)
    local custom_count=0
    if [[ "$INCLUDE_CUSTOM" = true ]]; then
        custom_count=${#CUSTOM_PASSES[@]}
    fi
    local total_experiments=$((${#tools[@]} * profile_count * (opt_count + custom_count)))
    
    log_info "Total experiments to run: $total_experiments"
    
    # Run experiments
    local experiment_num=0
    
    for tool in "${tools[@]}"; do
        local tool_name=$(basename "$tool")
        local category=$(get_category "$tool")
        
        log_experiment "Testing $category/$tool_name"
        
        # Test each profile
        for profile in ${PROFILES//,/ }; do
            # First, build with this profile
            log_info "Building with profile: $profile"
            
            if ! ./build_all.sh --profile "$profile" --target wasm32-wasip1 build >/dev/null 2>&1; then
                log_warning "Failed to build $tool_name with profile $profile, skipping..."
                continue
            fi
            
            # Test each optimization level
            for opt_level in ${OPT_LEVELS//,/ }; do
                experiment_num=$((experiment_num + 1))
                echo -ne "\r${CYAN}Progress: $experiment_num/$total_experiments${NC} "
                
                # Track this combination
                ./track_wasm_sizes.sh \
                    --wasm "target/wasm32-wasip1/release/${tool_name}.wasm" \
                    --tool "$category/$tool_name" \
                    --profile "$profile" \
                    --opt-level "$opt_level" \
                    >> "$RESULTS_FILE" 2>/dev/null || {
                        log_warning "Failed to track: $tool_name, $profile, $opt_level"
                    }
            done
            
            # Test custom passes if enabled
            if [[ "$INCLUDE_CUSTOM" = true ]]; then
                for passes in "${CUSTOM_PASSES[@]}"; do
                    experiment_num=$((experiment_num + 1))
                    echo -ne "\r${CYAN}Progress: $experiment_num/$total_experiments${NC} "
                    
                    ./track_wasm_sizes.sh \
                        --wasm "target/wasm32-wasip1/release/${tool_name}.wasm" \
                        --tool "$category/$tool_name" \
                        --profile "$profile" \
                        --opt-level "custom" \
                        --opt-passes "$passes" \
                        >> "$RESULTS_FILE" 2>/dev/null || {
                            log_warning "Failed to track custom: $tool_name, $profile, $passes"
                        }
                done
            fi
        done
    done
    
    echo "" # Clear progress line
    log_success "All experiments complete!"
    
    # Analyze results
    analyze_results
    
    # Display summary
    echo ""
    log_info "Results saved to:"
    log_info "  - Detailed data: $RESULTS_FILE"
    log_info "  - Summary report: $SUMMARY_FILE"
    echo ""
    
    # Show a preview of the summary
    echo "=== TOP RECOMMENDATIONS ==="
    grep -A 10 "BEST CONFIGURATIONS BY CATEGORY" "$SUMMARY_FILE" | head -20
}

# Change to script directory
cd "$(dirname "$0")"

# Run main function
main "$@"