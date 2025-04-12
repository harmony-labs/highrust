
OUTPUT ?=                                       # Output file
CAPTURE ?=                                      # Empty by default, set to enable contree
CONTREE_CMD = contree -D $(if $(OUTPUT),-o $(OUTPUT),)   # Command to pipe output to contree
PROJECT_ROOT ?= $(abspath $(dir $(lastword $(MAKEFILE_LIST)))/..)
DIR ?= $(PROJECT_ROOT)/.context
DIR_SIMPLE ?= $(PROJECT_ROOT)/.context/tasks/test
LLDB_TARGET ?=
OUTPUT_CMD ?= &> $(if $(OUTPUT),-o $(OUTPUT),) 2>&
RUST_BACKTRACE ?= full
RUST_LOG ?= harmony_vfs=trace
RUST_LOG_BENCH ?= harmony_vfs=warn
RUST_LOG_FLAMEGRAPH ?= harmony_vfs=error
TEST ?=

# Dynamically detect OS and architecture
OS = $(shell uname -s 2>/dev/null || echo Windows)
ARCH = $(shell uname -m 2>/dev/null || echo x86_64)

# Set target based on OS and architecture
ifeq ($(OS),Darwin)
	ifeq ($(ARCH),x86_64)
		TARGET = x86_64-apple-darwin
	else ifeq ($(ARCH),arm64)
		TARGET = aarch64-apple-darwin
	else
		$(error Unsupported macOS architecture: $(ARCH))
	endif
else ifeq ($(OS),Linux)
	ifeq ($(ARCH),x86_64)
		TARGET = x86_64-unknown-linux-gnu
	else ifeq ($(ARCH),aarch64)
		TARGET = aarch64-unknown-linux-gnu
	else ifeq ($(ARCH),arm)
		TARGET = armv7-unknown-linux-gnueabihf
	else
		$(error Unsupported Linux architecture: $(ARCH))
	endif
else ifeq ($(OS),Windows)
	ifeq ($(ARCH),x86_64)
		TARGET = x86_64-pc-windows-msvc
	else ifeq ($(ARCH),i686)
		TARGET = i686-pc-windows-msvc
	else ifeq ($(ARCH),aarch64)
		TARGET = aarch64-pc-windows-msvc
	else
		$(error Unsupported Windows architecture: $(ARCH))
	endif
else
	$(error Unsupported OS: $(OS))
endif

.PHONY: build clean install release test test-unit test-integration

add-target:
	rustup target add $(TARGET)

add-all-targets: add-target-aarch64-apple-darwin add-target-x86_64-apple-darwin add-target-x86_64-unknown-linux-gnu add-target-x86_64-pc-windows-msvc

add-target-aarch64-apple-darwin:
	rustup target add aarch64-apple-darwin

add-target-x86_64-apple-darwin:
	rustup target add x86_64-apple-darwin

add-target-x86_64-unknown-linux-gnu:
	rustup target add x86_64-unknown-linux-gnu

add-target-x86_64-pc-windows-msvc:
	rustup target add x86_64-pc-windows-msvc

bench:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG_BENCH) cargo bench --target $(TARGET) $(TEST) --features bench -- --nocapture $(if $(CAPTURE),| $(OUTPUT_CMD),)

build:
	RUST_LOG=build_script_build=debug cargo build --target $(TARGET) $(if $(CAPTURE),| $(CONTREE_CMD),)

build-all: build-aarch64-apple-darwin build-x86_64-apple-darwin build-x86_64-unknown-linux-gnu build-x86_64-pc-windows-msvc

build-aarch64-apple-darwin:
	cross build --target aarch64-apple-darwin $(if $(CAPTURE),| $(CONTREE_CMD),) # macOS ARM

build-x86_64-apple-darwin:
	cross build --target x86_64-apple-darwin $(if $(CAPTURE),| $(CONTREE_CMD),) # macOS Intel

build-x86_64-unknown-linux-gnu:
	cross build --target x86_64-unknown-linux-gnu $(if $(CAPTURE),| $(CONTREE_CMD),) # Linux x86_64

build-x86_64-pc-windows-msvc:
	cross build --target x86_64-pc-windows-msvc $(if $(CAPTURE),| $(CONTREE_CMD),) # Windows x86_64

clean:
	cargo clean $(if $(CAPTURE),| $(CONTREE_CMD),)
	rm vendor/**/*.dylib

flamegraph:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG_FLAMEGRAPH) cargo flamegraph --bench benchmarks --root --features bench -- --bench $(TEST)

install:
	cargo install --path .

install-rust-tools:
	cargo install cross
	cargo install flamegraph

lldb:
# LLDB_TARGET should be something like target/aarch64-apple-darwin/release/deps/benchmarks-960d62b7cb798ea8
# after executable starts, type 'run'
# hit ctrl+c when you want to break
# type 'bt' to get a backtrace
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) lldb -- $(LLDB_TARGET)

# Build release with optional capture
release:
	cargo build --release --target $(TARGET) $(if $(CAPTURE),| $(CONTREE_CMD),)

# run: db-dir
# 	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo run --target $(TARGET) --bin harmony-vfs -- run $(DIR) --db-path $(DB_PATH) $(if $(HARMONY_SOCKET_PATH),--socket-path $(HARMONY_SOCKET_PATH),) --verbose $(if $(CAPTURE),| $(CONTREE_CMD),)

# run-for-integration-tests: db-dir
# 	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo run --target $(TARGET) --bin harmony-vfs -- run $(DIR) --db-path $(DB_PATH) --socket-path $(HARMONY_INTEGRATION_TEST_SOCKET_PATH) --verbose $(if $(CAPTURE),| $(CONTREE_CMD),)

# run-in-memory:
# 	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo run --target $(TARGET) --bin harmony-vfs -- run $(DIR) --verbose $(if $(CAPTURE),| $(CONTREE_CMD),)

# run-simple:
# 	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo run --target $(TARGET) --bin harmony-vfs -- run $(DIR_SIMPLE) --verbose $(if $(CAPTURE),| $(CONTREE_CMD),)

# Rust test targets
# NOTE: In Rust, all tests in the tests/ directory (e.g., tests/api/graphql_test.rs) are always considered integration tests by cargo,
# even if they are "unit-style" tests. Only #[cfg(test)] modules in src/ are considered unit/lib tests.
# - test: runs all tests (unit/lib + integration)
# - test-unit: runs only unit/lib tests in src/
# - test-integration: runs only integration tests in tests/
# If you want tests in tests/ to be run as unit tests, they must be moved into src/ as #[cfg(test)] modules.

test:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=error cargo test --target $(TARGET) $(TEST) --workspace -- --nocapture --test-threads=1 $(if $(CAPTURE),| $(CONTREE_CMD),)

test-verbose:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo test --target $(TARGET) $(TEST) --workspace -- --nocapture --test-threads=1 $(if $(CAPTURE),| $(CONTREE_CMD),)

test-list:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo test --target $(TARGET) -- --list $(if $(CAPTURE),| $(CONTREE_CMD),)

test-integration:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=error cargo test --test '*' --target $(TARGET) --workspace $(TEST) -- --nocapture --test-threads=1 $(if $(CAPTURE),| $(CONTREE_CMD),)

test-integration-verbose:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo test --test '*' --target $(TARGET) --workspace $(TEST) -- --nocapture --test-threads=1 $(if $(CAPTURE),| $(CONTREE_CMD),)
