#!/bin/bash
set -e

# Directory paths
FIXTURES_DIR="$(dirname "$0")/fixtures/basic"
EXPECTED_DIR="$(dirname "$0")/expected/basic"
TMP_DIR="/tmp/highrust_runtime_tests"
WORKSPACE_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
CLI_BIN="$WORKSPACE_ROOT/target/debug/highrust-cli"

mkdir -p "$TMP_DIR"

# Build the CLI if not already built
echo "Building highrust CLI..."
(cd "$(dirname "$0")/../.." && cargo build --bin highrust-cli)

failures=0
total=0

for hrs_file in "$FIXTURES_DIR"/hello_world.hrs; do
    base=$(basename "$hrs_file" .hrs)
    gen_rs="$TMP_DIR/${base}_gen.rs"
    bin_out="$TMP_DIR/${base}_gen_bin"

    echo "=== Test: $base ==="
    echo "Transpiling $hrs_file -> $gen_rs"
    "$CLI_BIN" transpile --input "$hrs_file" --output "$gen_rs"

    # Optionally compare to expected output
    expected_rs="$EXPECTED_DIR/$base.rs"
    if [ -f "$expected_rs" ]; then
        if ! diff -q "$gen_rs" "$expected_rs" > /dev/null; then
            echo "WARNING: Generated Rust code differs from expected for $base"
        fi
    fi

    echo "Compiling $gen_rs -> $bin_out"
    rustc "$gen_rs" -o "$bin_out"

    echo "Running $bin_out"
    if "$bin_out"; then
        echo "PASS: $base"
    else
        echo "FAIL: $base (runtime error)"
        failures=$((failures+1))
    fi
    total=$((total+1))
    echo
done

echo "=== Runtime Test Summary ==="
echo "Total: $total, Failures: $failures"

if [ "$failures" -eq 0 ]; then
    echo "All runtime tests passed."
    exit 0
else
    echo "Some runtime tests failed."
    exit 1
fi