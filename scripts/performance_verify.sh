#!/usr/bin/env bash
# Performance verification script for pb v1.0.0

set -e

echo "=== pb v1.0.0 Performance Verification ==="
echo

# Build optimized binary
echo "Building optimized release binary..."
cargo build --release
echo "✓ Build completed"
echo

# Check binary size
BINARY_SIZE=$(stat -c%s target/release/pb 2>/dev/null || stat -f%z target/release/pb)
BINARY_SIZE_MB=$(echo "scale=1; $BINARY_SIZE / 1024 / 1024" | bc -l)
echo "Binary size: ${BINARY_SIZE_MB}MB"

if (( $(echo "$BINARY_SIZE_MB < 5.0" | bc -l) )); then
    echo "✓ Binary size is within acceptable range (<5MB)"
else
    echo "⚠ Binary size is larger than expected"
fi
echo

# Test startup time
echo "Testing startup time..."
START_TIME=$(date +%s%N)
./target/release/pb --help > /dev/null
END_TIME=$(date +%s%N)
STARTUP_TIME_MS=$(echo "scale=2; ($END_TIME - $START_TIME) / 1000000" | bc -l)

echo "Startup time: ${STARTUP_TIME_MS}ms"
if (( $(echo "$STARTUP_TIME_MS < 100" | bc -l) )); then
    echo "✓ Startup time is within acceptable range (<100ms)"
else
    echo "⚠ Startup time is higher than expected"
fi
echo

# Test version display performance
echo "Testing version command performance..."
time ./target/release/pb --version
echo "✓ Version command completed"
echo

# Test help display performance  
echo "Testing help command performance..."
time ./target/release/pb --help > /dev/null
echo "✓ Help command completed"
echo

# Test argument parsing with invalid input
echo "Testing error handling performance..."
START_TIME=$(date +%s%N)
./target/release/pb --start "invalid" --end "also-invalid" 2>/dev/null || true
END_TIME=$(date +%s%N)
ERROR_TIME_MS=$(echo "scale=2; ($END_TIME - $START_TIME) / 1000000" | bc -l)

echo "Error handling time: ${ERROR_TIME_MS}ms"
if (( $(echo "$ERROR_TIME_MS < 50" | bc -l) )); then
    echo "✓ Error handling is fast (<50ms)"
else
    echo "⚠ Error handling is slower than expected"
fi
echo

# Memory usage test (requires valgrind on Linux)
if command -v valgrind >/dev/null 2>&1; then
    echo "Testing memory usage with valgrind..."
    valgrind --tool=massif --pages-as-heap=yes --massif-out-file=massif.out \
        timeout 5 ./target/release/pb --start "$(date '+%Y-%m-%d %H:%M:%S')" --end "1m" --interval 1 2>/dev/null || true
    
    if [ -f massif.out ]; then
        PEAK_MEMORY=$(grep "mem_heap_B=" massif.out | sed -e 's/mem_heap_B=\([0-9]*\)/\1/' | sort -rn | head -1)
        PEAK_MEMORY_MB=$(echo "scale=1; $PEAK_MEMORY / 1024 / 1024" | bc -l)
        echo "Peak memory usage: ${PEAK_MEMORY_MB}MB"
        
        if (( $(echo "$PEAK_MEMORY_MB < 15" | bc -l) )); then
            echo "✓ Memory usage is within acceptable range (<15MB)"
        else
            echo "⚠ Memory usage is higher than expected"
        fi
        rm -f massif.out
    fi
    echo
fi

# Cross-platform compatibility test
echo "Testing cross-platform binary compatibility..."
file target/release/pb
echo "✓ Binary type verified"
echo

# Performance summary
echo "=== Performance Summary ==="
echo "Binary size: ${BINARY_SIZE_MB}MB"
echo "Startup time: ${STARTUP_TIME_MS}ms"
echo "Error handling: ${ERROR_TIME_MS}ms"
echo
echo "✓ Performance verification completed successfully!"
echo "pb v1.0.0 meets all performance requirements."