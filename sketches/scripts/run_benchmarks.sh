#!/bin/bash
#
# run_all_benchmarks.sh - Run all Criterion benchmarks
#
# This script runs all benchmarks in the benches/ directory and
# generates reports in the target/criterion directory.

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}================================${NC}"
echo -e "${BLUE}Criterion Benchmark Suite${NC}"
echo -e "${BLUE}================================${NC}"
echo ""

# Check if cargo is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: cargo is not installed${NC}"
    exit 1
fi

echo -e "${BLUE}Cargo version:${NC} $(cargo --version)"
echo -e "${BLUE}Rust version:${NC} $(rustc --version)"
echo ""

# Parse command line arguments
CRITERION_ARGS=""
SAVE_BASELINE=""
COMPARE_BASELINE=""
VERBOSE=""
BENCHMARKS=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --baseline)
            CRITERION_ARGS="$CRITERION_ARGS --save-baseline $2"
            SAVE_BASELINE="$2"
            shift 2
            ;;
        --compare)
            CRITERION_ARGS="$CRITERION_ARGS --baseline $2"
            COMPARE_BASELINE="$2"
            shift 2
            ;;
        --verbose)
            VERBOSE="--verbose"
            shift
            ;;
        --bench)
            BENCHMARKS="$BENCHMARKS --bench $2"
            shift 2
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  --baseline NAME     Save results as Criterion baseline NAME"
            echo "  --compare NAME      Compare against Criterion baseline NAME"
            echo "  --verbose           Show verbose output"
            echo "  --bench NAME        Run specific benchmark NAME"
            echo "  --help              Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                              # Run all benchmarks"
            echo "  $0 --baseline main              # Save as 'main' baseline"
            echo "  $0 --compare main               # Compare against 'main'"
            echo "  $0 --bench bloom_throughput     # Run specific benchmark"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

# Build benchmarks first
echo -e "${BLUE}Building benchmarks...${NC}"
if cargo build --benches --release $VERBOSE; then
    echo -e "${GREEN}✓ Build successful${NC}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi

echo ""

# Run benchmarks
echo -e "${BLUE}Running benchmarks...${NC}"
echo ""

if [ -n "$SAVE_BASELINE" ]; then
    echo -e "${YELLOW}Saving results as baseline: $SAVE_BASELINE${NC}"
    echo ""
fi

if [ -n "$COMPARE_BASELINE" ]; then
    echo -e "${YELLOW}Comparing against baseline: ${COMPARE_BASELINE#--baseline }${NC}"
    echo ""
fi

# Check if benches directory exists
if [ ! -d "benches" ]; then
    echo -e "${YELLOW}No benches directory found${NC}"
    echo "Create benchmark files in benches/ to get started"
    exit 0
fi

# Count benchmark files
BENCH_COUNT=$(find benches -name '*.rs' -type f | wc -l | xargs)

if [ "$BENCH_COUNT" -eq 0 ]; then
    echo -e "${YELLOW}No benchmark files found in benches/${NC}"
    echo "Create .rs files in benches/ to define benchmarks"
    exit 0
fi

echo -e "${BLUE}Found $BENCH_COUNT benchmark file(s)${NC}"
echo ""

# Run cargo bench with Criterion arguments passed after --
if [ -n "$CRITERION_ARGS" ]; then
    BENCH_CMD="cargo bench --all-features $BENCHMARKS $VERBOSE -- $CRITERION_ARGS"
else
    BENCH_CMD="cargo bench --all-features $BENCHMARKS $VERBOSE"
fi

if eval "$BENCH_CMD"; then
    echo ""
    echo -e "${GREEN}✓ Benchmarks completed successfully${NC}"
    
    # Show where results are stored
    if [ -d "target/criterion" ]; then
        echo ""
        echo -e "${BLUE}Results saved to: target/criterion/${NC}"
        echo ""
        echo "View HTML reports:"
        
        # List all HTML reports
        find target/criterion -name 'index.html' -type f | while read report; do
            # Get relative path
            rel_path=${report#target/criterion/}
            bench_name=$(dirname "$rel_path")
            echo -e "  ${GREEN}•${NC} $bench_name"
            echo -e "    file://$(pwd)/$report"
        done
        
        # Summary report
        if [ -f "target/criterion/report/index.html" ]; then
            echo ""
            echo -e "${BLUE}Summary report:${NC}"
            echo -e "  file://$(pwd)/target/criterion/report/index.html"
        fi
    fi
    
    exit 0
else
    echo ""
    echo -e "${RED}✗ Benchmarks failed${NC}"
    exit 1
fi
