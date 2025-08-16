#!/bin/bash

# VectraEdge Test Runner Script
# This script runs all tests for the VectraEdge project

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
RUST_TESTS=true
PYTHON_TESTS=true
INTEGRATION_TESTS=true
BENCHMARKS=true
PERFORMANCE_TESTS=true
COVERAGE=false

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --rust-only)
            PYTHON_TESTS=false
            INTEGRATION_TESTS=false
            BENCHMARKS=false
            PERFORMANCE_TESTS=false
            shift
            ;;
        --python-only)
            RUST_TESTS=false
            INTEGRATION_TESTS=false
            BENCHMARKS=false
            PERFORMANCE_TESTS=false
            shift
            ;;
        --no-integration)
            INTEGRATION_TESTS=false
            shift
            ;;
        --no-benchmarks)
            BENCHMARKS=false
            shift
            ;;
        --no-performance)
            PERFORMANCE_TESTS=false
            shift
            ;;
        --coverage)
            COVERAGE=true
            shift
            ;;
        --help)
            echo "Usage: $0 [OPTIONS]"
            echo "Options:"
            echo "  --rust-only        Run only Rust tests"
            echo "  --python-only      Run only Python tests"
            echo "  --no-integration   Skip integration tests"
            echo "  --no-benchmarks    Skip benchmark tests"
            echo "  --no-performance   Skip performance tests"
            echo "  --coverage         Generate coverage reports"
            echo "  --help             Show this help message"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            echo "Use --help for usage information"
            exit 1
            ;;
    esac
done

echo -e "${BLUE}🚀 VectraEdge Test Runner${NC}"
echo "================================"

# Function to print status
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
print_status "Checking prerequisites..."

if ! command_exists cargo; then
    print_error "Rust/Cargo not found. Please install Rust first."
    exit 1
fi

if ! command_exists python3; then
    print_warning "Python 3 not found. Python tests will be skipped."
    PYTHON_TESTS=false
fi

if ! command_exists pip; then
    print_warning "pip not found. Python tests will be skipped."
    PYTHON_TESTS=false
fi

print_success "Prerequisites check completed"

# Create test results directory
TEST_RESULTS_DIR="test_results_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$TEST_RESULTS_DIR"

print_status "Test results will be saved to: $TEST_RESULTS_DIR"

# Run Rust tests
if [ "$RUST_TESTS" = true ]; then
    echo ""
    print_status "Running Rust tests..."
    
    if [ "$COVERAGE" = true ]; then
        if command_exists cargo-tarpaulin; then
            print_status "Running Rust tests with coverage..."
            cargo tarpaulin --out Html --output-dir "$TEST_RESULTS_DIR/rust_coverage" || {
                print_warning "Coverage generation failed, running tests without coverage"
                cargo test --verbose 2>&1 | tee "$TEST_RESULTS_DIR/rust_tests.log"
            }
        else
            print_warning "cargo-tarpaulin not found, installing..."
            cargo install cargo-tarpaulin
            cargo tarpaulin --out Html --output-dir "$TEST_RESULTS_DIR/rust_coverage" || {
                print_warning "Coverage generation failed, running tests without coverage"
                cargo test --verbose 2>&1 | tee "$TEST_RESULTS_DIR/rust_tests.log"
            }
        fi
    else
        cargo test --verbose 2>&1 | tee "$TEST_RESULTS_DIR/rust_tests.log"
    fi
    
    if [ ${PIPESTATUS[0]} -eq 0 ]; then
        print_success "Rust tests completed successfully"
    else
        print_error "Rust tests failed"
        RUST_TESTS_FAILED=true
    fi
fi

# Run Python tests
if [ "$PYTHON_TESTS" = true ]; then
    echo ""
    print_status "Running Python tests..."
    
    # Check if we're in the right directory
    if [ ! -d "python" ]; then
        print_warning "Python directory not found, skipping Python tests"
        PYTHON_TESTS=false
    else
        cd python
        
        # Install test dependencies
        print_status "Installing Python test dependencies..."
        pip install -e ".[dev]" || {
            print_warning "Failed to install dev dependencies, trying basic install"
            pip install pytest pytest-asyncio
        }
        
        # Run tests
        if [ "$COVERAGE" = true ] && command_exists pytest-cov; then
            print_status "Running Python tests with coverage..."
            python -m pytest tests/ --cov=vectra --cov-report=html --cov-report=term-missing \
                --cov-report=xml 2>&1 | tee "../$TEST_RESULTS_DIR/python_tests.log"
        else
            python -m pytest tests/ -v 2>&1 | tee "../$TEST_RESULTS_DIR/python_tests.log"
        fi
        
        if [ ${PIPESTATUS[0]} -eq 0 ]; then
            print_success "Python tests completed successfully"
        else
            print_error "Python tests failed"
            PYTHON_TESTS_FAILED=true
        fi
        
        cd ..
    fi
fi

# Run integration tests
if [ "$INTEGRATION_TESTS" = true ]; then
    echo ""
    print_status "Running integration tests..."
    
    # Check if Docker is available for integration tests
    if command_exists docker && command_exists docker-compose; then
        print_status "Starting services for integration tests..."
        
        # Start services in background
        docker-compose up -d vectra redpanda ollama || {
            print_warning "Failed to start services, skipping integration tests"
            INTEGRATION_TESTS=false
        }
        
        if [ "$INTEGRATION_TESTS" = true ]; then
            # Wait for services to be ready
            print_status "Waiting for services to be ready..."
            sleep 30
            
            # Run integration tests
            cargo test --test integration_tests --features integration 2>&1 | tee "$TEST_RESULTS_DIR/integration_tests.log"
            
            if [ ${PIPESTATUS[0]} -eq 0 ]; then
                print_success "Integration tests completed successfully"
            else
                print_error "Integration tests failed"
                INTEGRATION_TESTS_FAILED=true
            fi
            
            # Stop services
            print_status "Stopping services..."
            docker-compose down
        fi
    else
        print_warning "Docker not available, skipping integration tests"
        INTEGRATION_TESTS=false
    fi
fi

# Run benchmarks
if [ "$BENCHMARKS" = true ]; then
    echo ""
    print_status "Running benchmarks..."
    
    # Check if criterion is available
    if grep -q "criterion" Cargo.toml; then
        cargo bench 2>&1 | tee "$TEST_RESULTS_DIR/benchmarks.log"
        
        if [ ${PIPESTATUS[0]} -eq 0 ]; then
            print_success "Benchmarks completed successfully"
        else
            print_warning "Benchmarks failed or not implemented"
        fi
    else
        print_warning "Criterion not configured, skipping benchmarks"
    fi
fi

# Run performance tests
if [ "$PERFORMANCE_TESTS" = true ]; then
    echo ""
    print_status "Running performance tests..."
    
    if [ -f "scripts/performance_test.py" ]; then
        cd scripts
        python3 performance_test.py --output "../$TEST_RESULTS_DIR/performance_results.json" 2>&1 | tee "../$TEST_RESULTS_DIR/performance_tests.log"
        
        if [ ${PIPESTATUS[0]} -eq 0 ]; then
            print_success "Performance tests completed successfully"
        else
            print_warning "Performance tests failed"
        fi
        
        cd ..
    else
        print_warning "Performance test script not found"
    fi
fi

# Generate test summary
echo ""
print_status "Generating test summary..."

SUMMARY_FILE="$TEST_RESULTS_DIR/test_summary.txt"
{
    echo "VectraEdge Test Summary"
    echo "======================"
    echo "Date: $(date)"
    echo "Test Results Directory: $TEST_RESULTS_DIR"
    echo ""
    
    echo "Test Results:"
    echo "-------------"
    
    if [ "$RUST_TESTS" = true ]; then
        if [ "$RUST_TESTS_FAILED" = true ]; then
            echo "❌ Rust Tests: FAILED"
        else
            echo "✅ Rust Tests: PASSED"
        fi
    fi
    
    if [ "$PYTHON_TESTS" = true ]; then
        if [ "$PYTHON_TESTS_FAILED" = true ]; then
            echo "❌ Python Tests: FAILED"
        else
            echo "✅ Python Tests: PASSED"
        fi
    fi
    
    if [ "$INTEGRATION_TESTS" = true ]; then
        if [ "$INTEGRATION_TESTS_FAILED" = true ]; then
            echo "❌ Integration Tests: FAILED"
        else
            echo "✅ Integration Tests: PASSED"
        fi
    fi
    
    if [ "$BENCHMARKS" = true ]; then
        echo "📊 Benchmarks: COMPLETED"
    fi
    
    if [ "$PERFORMANCE_TESTS" = true ]; then
        echo "⚡ Performance Tests: COMPLETED"
    fi
    
    echo ""
    echo "Coverage:"
    echo "---------"
    if [ "$COVERAGE" = true ]; then
        if [ "$RUST_TESTS" = true ]; then
            echo "📈 Rust Coverage: Generated in $TEST_RESULTS_DIR/rust_coverage/"
        fi
        if [ "$PYTHON_TESTS" = true ]; then
            echo "📈 Python Coverage: Generated in $TEST_RESULTS_DIR/htmlcov/"
        fi
    else
        echo "📈 Coverage: Not generated (use --coverage flag)"
    fi
    
    echo ""
    echo "Log Files:"
    echo "----------"
    ls -la "$TEST_RESULTS_DIR"/*.log 2>/dev/null || echo "No log files found"
    
} > "$SUMMARY_FILE"

print_success "Test summary saved to: $SUMMARY_FILE"

# Display summary
cat "$SUMMARY_FILE"

echo ""
echo "================================"

# Check if any tests failed
if [ "$RUST_TESTS_FAILED" = true ] || [ "$PYTHON_TESTS_FAILED" = true ] || [ "$INTEGRATION_TESTS_FAILED" = true ]; then
    print_error "Some tests failed. Check the logs in $TEST_RESULTS_DIR for details."
    exit 1
else
    print_success "All tests completed successfully!"
    print_status "Test results and logs saved in: $TEST_RESULTS_DIR"
    exit 0
fi
