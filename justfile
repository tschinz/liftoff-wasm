# liftoff — Justfile
#
# This Justfile provides convenient targets for building, running, and testing
# the liftoff application. It's designed to simplify the development workflow
# for this Rust WebAssembly project that uses the Trunk build tool.
#
# Usage:
#   just <target>         # Run a specific target
#   just                  # Show help (default target)
#   just --list           # List all available targets
#
# Prerequisites:
#   - Rust toolchain (https://rustup.rs/)
#   - Trunk build tool: just install-trunk
#
# Quick start:
#   just install-deps     # Install Trunk and fetch dependencies
#   just run              # Start the development server
#
# For more information about Just, see: https://just.systems/

_default: help

# ──────────────────────────────────────────────────────
# Help & Information
# ──────────────────────────────────────────────────────

# Show available targets
help:
	@just --list

# ──────────────────────────────────────────────────────
# Installation
# ──────────────────────────────────────────────────────

# Install all dependencies: Trunk (WebAssembly bundler) and Rust crates
# This is the first command to run when setting up the project.
# It installs the trunk CLI tool globally and fetches all Cargo dependencies.
install-deps:
	@echo "=== Installing Trunk and fetching dependencies ==="
	cargo install trunk --locked
	cargo build --release

# Install only the Trunk build tool (useful if you already have Rust deps)
install-trunk:
	cargo install trunk --locked

# ──────────────────────────────────────────────────────
# Development
# ──────────────────────────────────────────────────────

# Start the development server on http://localhost:8080
# Automatically opens your browser to the app.
# Use this during active development for hot-reloading.
run:
	@echo "=== Starting dev server on http://localhost:8080 ==="
	trunk serve --port 8080 --open

# Start the development server without auto-opening the browser
# Useful when you want to manually choose which browser to use.
run-no-open:
	@echo "=== Starting dev server on http://localhost:8080 ==="
	trunk serve --port 8080

# Build the WebAssembly bundle for production
# Output is placed in the dist/ directory with optimized WASM and minified assets.
build:
	@echo "=== Building release bundle ==="
	trunk build --release

# ──────────────────────────────────────────────────────
# Testing & Linting
# ──────────────────────────────────────────────────────

# Run all Rust tests (unit and integration tests)
test:
	@echo "== Running tests =="
	cargo test --all

# Run tests with coverage report (requires cargo-tarpaulin)
test-coverage:
	@echo "== Running tests with coverage =="
	cargo tarpaulin --out Html --all

# Run tests in release mode for performance-sensitive test suites
test-optimized:
	@echo "=== Running tests in release mode ==="
	cargo test --all --release

# Check code compilation without producing output artifacts
# Much faster than build; useful for quick verification during development.
check:
	@echo "=== Checking code ==="
	cargo check --all

# Format all Rust code according to idiomatic Rust style
# Automatically fixes formatting issues. Run before committing.
fmt:
	@echo "=== Formatting code ==="
	cargo fmt --all

# Check code formatting without making changes
# Useful in CI/CD pipelines to ensure code is properly formatted.
fmt-check:
	@echo "=== Checking formatting ==="
	cargo fmt --all --check

# Run Clippy lints with strict warnings (treats warnings as errors)
# Catches common mistakes and suggests idiomatic code improvements.
clippy:
	@echo "=== Running Clippy ==="
	cargo clippy --all-targets -- -D warnings

# Auto-fix Clippy warnings where possible
# The --allow-dirty flag lets you keep uncommitted changes.
clippy-fix:
	@echo "=== Fixing Clippy warnings ==="
	cargo clippy --all-targets --fix --allow-staged --allow-dirty --allow-no-vcs -- -D warnings

# ──────────────────────────────────────────────────────
# Maintenance
# ──────────────────────────────────────────────────────

# Remove all build artifacts and generated files
# Cleans the Cargo target directory, dist output, and any cached files.
# Useful when you encounter build issues or want to free up disk space.
clean:
	@echo "=== Cleaning build artifacts ==="
	cargo clean
	rm -rf target
	rm -rf dist

# Update all Cargo dependencies to their latest compatible versions
# Run this to get the newest versions of all crates.
update-deps:
	@echo "=== Updating dependencies ==="
	cargo update
