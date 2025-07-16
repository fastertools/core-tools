# Core Tools - Makefile
# Convenient commands for building and managing tools

.PHONY: help build build-all build-changed clean test list install-deps

# Default target
help: ## Show this help message
	@echo "Core Tools - Available commands:"
	@echo ""
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  \033[36m%-15s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)
	@echo ""
	@echo "Examples:"
	@echo "  make build          # Build all tools"
	@echo "  make build-changed  # Build only changed tools"
	@echo "  make clean          # Clean build artifacts"
	@echo "  make test           # Run tests"

build: install-deps ## Build all tools
	./build_all.sh build

build-all: build ## Alias for build

build-changed: install-deps ## Build only tools that have changed since main
	./build_all.sh changed

build-debug: install-deps ## Build all tools in debug mode
	./build_all.sh --debug build

clean: ## Clean all build artifacts
	./build_all.sh clean

list: ## List all available tools
	./build_all.sh list

test: ## Run basic tests on the tools
	@echo "Starting test server..."
	./test_server start
	@sleep 3
	@echo "Running basic tool tests..."
	./curl.sh distance '{"lat1": 40.7128, "lon1": -74.0060, "lat2": 34.0522, "lon2": -118.2437}'
	./curl.sh add '{"a": 5, "b": 3}'
	@echo "Stopping test server..."
	./test_server stop
	@echo "Tests completed successfully!"

install-deps: ## Install required dependencies
	@echo "Checking dependencies..."
	@if ! command -v cargo >/dev/null 2>&1; then \
		echo "Error: Cargo not found. Please install Rust: https://rustup.rs/"; \
		exit 1; \
	fi
	@if ! rustup target list --installed | grep -q wasm32-wasip1; then \
		echo "Installing wasm32-wasip1 target..."; \
		rustup target add wasm32-wasip1; \
	fi
	@echo "Dependencies satisfied!"

# Development helpers
dev-setup: install-deps ## Set up development environment
	@echo "Setting up development environment..."
	@chmod +x build_all.sh test_server curl.sh
	@echo "Development environment ready!"

# CI helpers
ci-build: ## Build for CI (with caching optimizations)
	./build_all.sh --jobs 8 build

ci-test: ## Run CI tests
	./build_all.sh changed
	@if [ -f spin.pid ]; then ./test_server stop; fi
	./test_server start
	@sleep 5
	@timeout 30s ./curl.sh distance '{"lat1": 0, "lon1": 0, "lat2": 1, "lon2": 1}' || (echo "CI test failed" && exit 1)
	./test_server stop

# Release helpers
package: build ## Create release package
	@echo "Creating release package..."
	@rm -rf release-package
	@mkdir -p release-package
	@cp spin.toml release-package/
	@cp -r tools release-package/
	@find release-package/tools -name target -type d -exec rm -rf {} + 2>/dev/null || true
	@./build_all.sh build
	@tar -czf core-tools-$(shell date +%Y%m%d-%H%M%S).tar.gz -C release-package .
	@rm -rf release-package
	@echo "Release package created!"

# Documentation helpers
docs: ## Generate documentation
	@echo "Generating documentation..."
	@echo "# Core Tools - Available Tools" > TOOLS.md
	@echo "" >> TOOLS.md
	@echo "This document lists all available computational tools." >> TOOLS.md
	@echo "" >> TOOLS.md
	@./build_all.sh list | sed 's/^  /- /' >> TOOLS.md
	@echo "Documentation updated!"

# Statistics
stats: ## Show project statistics
	@echo "Core Tools Statistics:"
	@echo "  Total tools: $$(./build_all.sh list | wc -l)"
	@echo "  Categories:"
	@echo "    Basic Math: $$(./build_all.sh list | grep basic_math | wc -l)"
	@echo "    Geospatial: $$(./build_all.sh list | grep geospatial | wc -l)"
	@echo "    Math3D: $$(./build_all.sh list | grep math3d | wc -l)"
	@echo "    Statistics: $$(./build_all.sh list | grep statistics | wc -l)"
	@echo "  Total Rust files: $$(find tools -name '*.rs' | wc -l)"
	@echo "  Total lines of code: $$(find tools -name '*.rs' -exec cat {} \; | wc -l)"