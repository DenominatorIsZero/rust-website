# Rust Personal Website - Development Commands

# Default recipe shows available commands
default:
    @just --list

# Setup and Maintenance Commands

# Install required development tools
setup:
    cargo install cargo-watch
    cd tailwind && npm install

# Clean all build artifacts
clean:
    cargo clean
    rm -f static/css/index.css

# Development Commands

# Start development server with hot reload
dev:
    cargo watch -x run

# Run the website server normally
run:
    cargo run

# Watch and rebuild CSS during development
css-watch:
    cd tailwind && npm run watch-css

# Build CSS for production
css-build:
    cd tailwind && npm run build-css-prod

# Build Commands

# Build the website (debug)
build:
    cargo build

# Build the website (release)
build-release:
    cargo build --release

# Build Docker image locally
docker-build:
    docker build -t rust-website .

# Build for deployment and save as tar
docker-deploy:
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building for linux/amd64 deployment..."
    docker buildx build --platform linux/amd64 -t rust-website --output type=docker .
    echo "Saving Docker image to tar file..."
    docker save -o rust-website.tar rust-website
    echo "Deployment package ready: rust-website.tar"
    ls -lh rust-website.tar

# Quality Commands

# Format all code
fmt:
    cargo fmt --all

# Run clippy linting
lint:
    cargo clippy -- -D warnings

# Run tests (none yet, but ready for future)
test:
    cargo test

# Run security audit
audit:
    cargo audit

# Run all quality checks including security
check: fmt lint test audit

# Prepare for deployment (build CSS and release binary)
publish: css-build build-release