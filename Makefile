.PHONY: help build test clean run dev docker-build docker-run playground-install playground-dev

# Default target
help:
	@echo "VectraEdge Development Commands"
	@echo "================================"
	@echo ""
	@echo "Rust Commands:"
	@echo "  build          - Build the Rust application"
	@echo "  test           - Run Rust tests"
	@echo "  test-all       - Run all tests (Rust, Python, Integration)"
	@echo "  test-rust      - Run only Rust tests"
	@echo "  test-python    - Run only Python tests"
	@echo "  test-integration - Run integration tests"
	@echo "  test-benchmarks - Run benchmarks"
	@echo "  test-performance - Run performance tests"
	@echo "  test-coverage  - Run tests with coverage"
	@echo "  clean          - Clean build artifacts"
	@echo "  run            - Run the application"
	@echo "  dev            - Run in development mode with hot reload"
	@echo ""
	@echo "Docker Commands:"
	@echo "  docker-build   - Build Docker image"
	@echo "  docker-run     - Run with Docker Compose"
	@echo "  docker-stop    - Stop Docker services"
	@echo "  docker-clean   - Clean Docker resources"
	@echo ""
	@echo "Playground Commands:"
	@echo "  playground-install - Install playground dependencies"
	@echo "  playground-dev     - Run playground in development mode"
	@echo ""
	@echo "Utility Commands:"
	@echo "  install        - Install Rust dependencies"
	@echo "  format         - Format Rust code"
	@echo "  lint           - Run Rust linter"
	@echo "  docs           - Generate documentation"

# Rust commands
install:
	cargo install --path .

build:
	cargo build --release

test:
	cargo test

test-release: ## Run Rust tests in release mode
	cargo test --release

test-all: ## Run all tests (Rust, Python, Integration, Benchmarks)
	./scripts/run_tests.sh

test-rust: ## Run only Rust tests
	./scripts/run_tests.sh --rust-only

test-python: ## Run only Python tests
	./scripts/run_tests.sh --python-only

test-integration: ## Run integration tests
	./scripts/run_tests.sh --no-benchmarks --no-performance

test-benchmarks: ## Run benchmarks
	cargo bench

test-performance: ## Run performance tests
	python3 scripts/performance_test.py

test-coverage: ## Run tests with coverage
	./scripts/run_tests.sh --coverage

clean:
	cargo clean

run: build
	./target/release/vectra

dev:
	cargo run

# Docker commands
docker-build:
	docker build -t vectraedge/vectra:latest .

docker-run:
	docker-compose up -d

docker-stop:
	docker-compose down

docker-clean:
	docker-compose down -v --remove-orphans
	docker system prune -f

# Playground commands
playground-install:
	cd playground && npm install

playground-dev:
	cd playground && npm run dev

# Utility commands
format:
	cargo fmt

lint:
	cargo clippy

docs:
	cargo doc --open

# Development setup
setup: install playground-install
	@echo "Development environment setup complete!"
	@echo ""
	@echo "To start development:"
	@echo "  make dev          # Run Rust backend"
	@echo "  make playground-dev # Run Next.js frontend"
	@echo "  make docker-run   # Run with Docker"

# Quick start
start: docker-run
	@echo "VectraEdge is starting up..."
	@echo "  - Backend API: http://localhost:8080"
	@echo "  - Playground:  http://localhost:3000"
	@echo "  - Redpanda:    http://localhost:8081"
	@echo "  - Ollama:      http://localhost:11434"

# Production build
prod: clean build docker-build
	@echo "Production build complete!"
	@echo "Run with: make docker-run"

# Clean everything
clean-all: clean docker-clean
	@echo "All build artifacts and Docker resources cleaned!"
