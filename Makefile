# Tfdiff - Terraform Diff CLI Tool
# Comprehensive Makefile for development, testing, and deployment

# Project configuration
PROJECT_NAME := tfdiff
VERSION := $(shell grep '^version' Cargo.toml | sed 's/.*"\(.*\)".*/\1/')
RUST_VERSION := $(shell rustc --version | cut -d' ' -f2)

# Directories
SRC_DIR := src
TEST_DIR := tests
BENCH_DIR := benches
TARGET_DIR := target
DIST_DIR := dist
DOCS_DIR := docs

# Binary targets
BINARY_NAME := tfdiff
DEBUG_BINARY := $(TARGET_DIR)/debug/$(BINARY_NAME)
RELEASE_BINARY := $(TARGET_DIR)/release/$(BINARY_NAME)

# Build configuration
CARGO := cargo
CARGO_FLAGS := 
RELEASE_FLAGS := --release
TEST_FLAGS := --all-features --verbose
BENCH_FLAGS := --all-features

# Cross-compilation targets
TARGETS := x86_64-unknown-linux-gnu x86_64-apple-darwin aarch64-apple-darwin x86_64-pc-windows-gnu

# Colors for output
RED := \033[0;31m
GREEN := \033[0;32m
YELLOW := \033[0;33m
BLUE := \033[0;34m
PURPLE := \033[0;35m
CYAN := \033[0;36m
WHITE := \033[0;37m
NC := \033[0m # No Color

# Default target
.DEFAULT_GOAL := help

##@ Help
.PHONY: help
help: ## Display this help message
	@echo "$(CYAN)Tfdiff $(VERSION) - Terraform Diff CLI Tool$(NC)"
	@echo "$(YELLOW)Rust Version: $(RUST_VERSION)$(NC)"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  $(CYAN)%-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n$(YELLOW)%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development
.PHONY: dev
dev: ## Start development mode with file watching
	@echo "$(GREEN)Starting development mode...$(NC)"
	$(CARGO) watch -x 'run -- --help'

.PHONY: run
run: ## Run the application in debug mode
	@echo "$(GREEN)Running $(PROJECT_NAME) in debug mode...$(NC)"
	$(CARGO) run $(CARGO_FLAGS) -- --help

.PHONY: run-release
run-release: ## Run the application in release mode
	@echo "$(GREEN)Running $(PROJECT_NAME) in release mode...$(NC)"
	$(CARGO) run $(RELEASE_FLAGS) -- --help

.PHONY: example
example: build ## Run example with simple plan
	@echo "$(GREEN)Running example with simple plan...$(NC)"
	@echo "Plan: 2 to add, 1 to change, 0 to destroy." | $(DEBUG_BINARY) --format terminal

##@ Building
.PHONY: build
build: ## Build the project in debug mode
	@echo "$(GREEN)Building $(PROJECT_NAME) in debug mode...$(NC)"
	$(CARGO) build $(CARGO_FLAGS)
	@echo "$(GREEN)Debug binary created: $(DEBUG_BINARY)$(NC)"

.PHONY: build-release
build-release: ## Build the project in release mode
	@echo "$(GREEN)Building $(PROJECT_NAME) in release mode...$(NC)"
	$(CARGO) build $(RELEASE_FLAGS)
	@echo "$(GREEN)Release binary created: $(RELEASE_BINARY)$(NC)"

.PHONY: build-all
build-all: build build-release ## Build both debug and release versions

.PHONY: install
install: build-release ## Install the binary to cargo bin directory
	@echo "$(GREEN)Installing $(PROJECT_NAME)...$(NC)"
	$(CARGO) install --path . --force
	@echo "$(GREEN)$(PROJECT_NAME) installed successfully!$(NC)"

.PHONY: uninstall
uninstall: ## Uninstall the binary from cargo bin directory
	@echo "$(YELLOW)Uninstalling $(PROJECT_NAME)...$(NC)"
	$(CARGO) uninstall $(PROJECT_NAME)

##@ Testing
.PHONY: test
test: ## Run all tests
	@echo "$(GREEN)Running all tests...$(NC)"
	$(CARGO) test $(TEST_FLAGS)

.PHONY: test-unit
test-unit: ## Run unit tests only
	@echo "$(GREEN)Running unit tests...$(NC)"
	$(CARGO) test $(TEST_FLAGS) --lib

.PHONY: test-integration
test-integration: ## Run integration tests only
	@echo "$(GREEN)Running integration tests...$(NC)"
	$(CARGO) test $(TEST_FLAGS) --test '*'

.PHONY: test-doc
test-doc: ## Run documentation tests
	@echo "$(GREEN)Running documentation tests...$(NC)"
	$(CARGO) test $(TEST_FLAGS) --doc

.PHONY: test-all
test-all: test test-doc ## Run all tests including documentation tests

.PHONY: test-watch
test-watch: ## Run tests in watch mode
	@echo "$(GREEN)Running tests in watch mode...$(NC)"
	$(CARGO) watch -x 'test $(TEST_FLAGS)'

.PHONY: test-coverage
test-coverage: ## Generate test coverage report
	@echo "$(GREEN)Generating test coverage report...$(NC)"
	@if command -v cargo-tarpaulin >/dev/null 2>&1; then \
		$(CARGO) tarpaulin --out Html --output-dir target/coverage --all-features; \
		echo "$(GREEN)Coverage report generated: target/coverage/tarpaulin-report.html$(NC)"; \
	else \
		echo "$(RED)cargo-tarpaulin not installed. Installing...$(NC)"; \
		$(CARGO) install cargo-tarpaulin; \
		$(CARGO) tarpaulin --out Html --output-dir target/coverage --all-features; \
	fi

##@ Code Quality
.PHONY: check
check: ## Run cargo check
	@echo "$(GREEN)Running cargo check...$(NC)"
	$(CARGO) check --all-targets --all-features

.PHONY: clippy
clippy: ## Run clippy linter
	@echo "$(GREEN)Running clippy linter...$(NC)"
	$(CARGO) clippy --all-targets --all-features -- -D warnings

.PHONY: fmt
fmt: ## Format code with rustfmt
	@echo "$(GREEN)Formatting code...$(NC)"
	$(CARGO) fmt --all

.PHONY: fmt-check
fmt-check: ## Check code formatting
	@echo "$(GREEN)Checking code formatting...$(NC)"
	$(CARGO) fmt --all -- --check

.PHONY: audit
audit: ## Run security audit
	@echo "$(GREEN)Running security audit...$(NC)"
	@if command -v cargo-audit >/dev/null 2>&1; then \
		$(CARGO) audit; \
	else \
		echo "$(YELLOW)cargo-audit not installed. Installing...$(NC)"; \
		$(CARGO) install cargo-audit; \
		$(CARGO) audit; \
	fi

.PHONY: outdated
outdated: ## Check for outdated dependencies
	@echo "$(GREEN)Checking for outdated dependencies...$(NC)"
	@if command -v cargo-outdated >/dev/null 2>&1; then \
		$(CARGO) outdated; \
	else \
		echo "$(YELLOW)cargo-outdated not installed. Installing...$(NC)"; \
		$(CARGO) install cargo-outdated; \
		$(CARGO) outdated; \
	fi

.PHONY: lint
lint: fmt-check clippy ## Run all linting checks

.PHONY: qa
qa: check lint test audit ## Run comprehensive quality assurance checks

##@ Benchmarking
.PHONY: bench
bench: ## Run benchmarks
	@echo "$(GREEN)Running benchmarks...$(NC)"
	$(CARGO) bench $(BENCH_FLAGS)

.PHONY: bench-save
bench-save: ## Run benchmarks and save baseline
	@echo "$(GREEN)Running benchmarks and saving baseline...$(NC)"
	$(CARGO) bench $(BENCH_FLAGS) -- --save-baseline main

.PHONY: bench-compare
bench-compare: ## Compare benchmarks with baseline
	@echo "$(GREEN)Comparing benchmarks with baseline...$(NC)"
	$(CARGO) bench $(BENCH_FLAGS) -- --baseline main

##@ Documentation
.PHONY: doc
doc: ## Generate documentation
	@echo "$(GREEN)Generating documentation...$(NC)"
	$(CARGO) doc --all-features --no-deps
	@echo "$(GREEN)Documentation generated: target/doc/$(PROJECT_NAME)/index.html$(NC)"

.PHONY: doc-open
doc-open: doc ## Generate and open documentation
	@echo "$(GREEN)Opening documentation...$(NC)"
	$(CARGO) doc --all-features --no-deps --open

.PHONY: doc-private
doc-private: ## Generate documentation including private items
	@echo "$(GREEN)Generating documentation (including private)...$(NC)"
	$(CARGO) doc --all-features --no-deps --document-private-items

##@ Test Fixtures
.PHONY: fixtures
fixtures: ## Generate test fixtures
	@echo "$(GREEN)Generating test fixtures...$(NC)"
	@if [ -f "$(SRC_DIR)/fixtures/generate_fixtures.rs" ]; then \
		$(CARGO) run --bin generate_fixtures || echo "$(YELLOW)Fixture generation script not found$(NC)"; \
	else \
		echo "$(YELLOW)Creating fixture directories...$(NC)"; \
		mkdir -p tests/fixtures/generated; \
	fi

.PHONY: test-fixtures
test-fixtures: fixtures test ## Generate fixtures and run tests

##@ Cross-compilation
.PHONY: install-targets
install-targets: ## Install cross-compilation targets
	@echo "$(GREEN)Installing cross-compilation targets...$(NC)"
	@for target in $(TARGETS); do \
		echo "Installing target: $$target"; \
		rustup target add $$target; \
	done

.PHONY: cross-build
cross-build: install-targets ## Build for all targets
	@echo "$(GREEN)Cross-compiling for all targets...$(NC)"
	@mkdir -p $(DIST_DIR)
	@for target in $(TARGETS); do \
		echo "Building for $$target..."; \
		$(CARGO) build --release --target $$target; \
		if [ $$? -eq 0 ]; then \
			echo "$(GREEN)✓ Built for $$target$(NC)"; \
		else \
			echo "$(RED)✗ Failed to build for $$target$(NC)"; \
		fi; \
	done

##@ Distribution
.PHONY: dist-clean
dist-clean: ## Clean distribution directory
	@echo "$(YELLOW)Cleaning distribution directory...$(NC)"
	rm -rf $(DIST_DIR)
	mkdir -p $(DIST_DIR)

.PHONY: dist
dist: dist-clean cross-build ## Create distribution packages
	@echo "$(GREEN)Creating distribution packages...$(NC)"
	@for target in $(TARGETS); do \
		if [ -f "$(TARGET_DIR)/$$target/release/$(BINARY_NAME)" ] || [ -f "$(TARGET_DIR)/$$target/release/$(BINARY_NAME).exe" ]; then \
			echo "Packaging $$target..."; \
			mkdir -p "$(DIST_DIR)/$(PROJECT_NAME)-$(VERSION)-$$target"; \
			if [ -f "$(TARGET_DIR)/$$target/release/$(BINARY_NAME).exe" ]; then \
				cp "$(TARGET_DIR)/$$target/release/$(BINARY_NAME).exe" "$(DIST_DIR)/$(PROJECT_NAME)-$(VERSION)-$$target/"; \
			else \
				cp "$(TARGET_DIR)/$$target/release/$(BINARY_NAME)" "$(DIST_DIR)/$(PROJECT_NAME)-$(VERSION)-$$target/"; \
			fi; \
			cp README.md "$(DIST_DIR)/$(PROJECT_NAME)-$(VERSION)-$$target/" 2>/dev/null || true; \
			cp LICENSE "$(DIST_DIR)/$(PROJECT_NAME)-$(VERSION)-$$target/" 2>/dev/null || true; \
			cd $(DIST_DIR) && tar -czf "$(PROJECT_NAME)-$(VERSION)-$$target.tar.gz" "$(PROJECT_NAME)-$(VERSION)-$$target"; \
			cd ..; \
		fi; \
	done
	@echo "$(GREEN)Distribution packages created in $(DIST_DIR)$(NC)"

##@ Docker
.PHONY: docker-build
docker-build: ## Build Docker image
	@echo "$(GREEN)Building Docker image...$(NC)"
	docker build -t $(PROJECT_NAME):$(VERSION) .
	docker tag $(PROJECT_NAME):$(VERSION) $(PROJECT_NAME):latest

.PHONY: docker-run
docker-run: docker-build ## Run Docker container
	@echo "$(GREEN)Running Docker container...$(NC)"
	docker run --rm -it $(PROJECT_NAME):$(VERSION) --help

.PHONY: docker-shell
docker-shell: docker-build ## Run Docker container with shell
	@echo "$(GREEN)Starting Docker shell...$(NC)"
	docker run --rm -it --entrypoint /bin/sh $(PROJECT_NAME):$(VERSION)

##@ Maintenance
.PHONY: clean
clean: ## Clean build artifacts
	@echo "$(YELLOW)Cleaning build artifacts...$(NC)"
	$(CARGO) clean
	rm -rf $(DIST_DIR)
	rm -rf target/coverage

.PHONY: clean-all
clean-all: clean ## Clean all artifacts including dependencies
	@echo "$(YELLOW)Cleaning all artifacts...$(NC)"
	rm -rf ~/.cargo/registry/cache
	rm -rf ~/.cargo/git/db

.PHONY: update
update: ## Update dependencies
	@echo "$(GREEN)Updating dependencies...$(NC)"
	$(CARGO) update
	@echo "$(GREEN)Dependencies updated!$(NC)"

.PHONY: deps
deps: ## Install development dependencies
	@echo "$(GREEN)Installing development dependencies...$(NC)"
	@echo "Installing cargo tools..."
	$(CARGO) install cargo-watch cargo-tarpaulin cargo-audit cargo-outdated criterion || true
	@echo "$(GREEN)Development dependencies installed!$(NC)"

##@ CI/CD
.PHONY: ci-check
ci-check: check lint test ## Run CI checks (no build artifacts)

.PHONY: ci-test
ci-test: ## Run full CI test suite
	@echo "$(GREEN)Running full CI test suite...$(NC)"
	$(MAKE) check
	$(MAKE) lint  
	$(MAKE) test-all
	$(MAKE) bench
	$(MAKE) audit

.PHONY: ci-build
ci-build: ## Run CI build process
	@echo "$(GREEN)Running CI build process...$(NC)"
	$(MAKE) ci-test
	$(MAKE) build-release
	$(MAKE) dist

##@ Release
.PHONY: version
version: ## Show current version
	@echo "$(CYAN)Project: $(PROJECT_NAME)$(NC)"
	@echo "$(CYAN)Version: $(VERSION)$(NC)"
	@echo "$(CYAN)Rust Version: $(RUST_VERSION)$(NC)"

.PHONY: tag
tag: ## Create git tag for current version
	@echo "$(GREEN)Creating git tag v$(VERSION)...$(NC)"
	git tag -a v$(VERSION) -m "Release v$(VERSION)"
	@echo "$(GREEN)Tag v$(VERSION) created. Push with: git push origin v$(VERSION)$(NC)"

.PHONY: release-check
release-check: ## Check if ready for release
	@echo "$(GREEN)Checking release readiness...$(NC)"
	@echo "Current version: $(VERSION)"
	@git status --porcelain | grep -q . && echo "$(RED)Working directory not clean$(NC)" || echo "$(GREEN)✓ Working directory clean$(NC)"
	@$(MAKE) ci-test >/dev/null 2>&1 && echo "$(GREEN)✓ All tests pass$(NC)" || echo "$(RED)✗ Tests failing$(NC)"
	@$(CARGO) package --dry-run >/dev/null 2>&1 && echo "$(GREEN)✓ Package ready$(NC)" || echo "$(RED)✗ Package issues$(NC)"

.PHONY: release
release: release-check dist ## Create release package
	@echo "$(GREEN)Creating release for v$(VERSION)...$(NC)"
	@echo "$(GREEN)Release artifacts created in $(DIST_DIR)$(NC)"
	@echo "$(YELLOW)Next steps:$(NC)"
	@echo "  1. Review artifacts in $(DIST_DIR)"
	@echo "  2. Run 'make tag' to create git tag"
	@echo "  3. Push tag: git push origin v$(VERSION)"
	@echo "  4. Create GitHub release with artifacts"

##@ Performance
.PHONY: perf
perf: build-release ## Run basic performance test
	@echo "$(GREEN)Running performance test...$(NC)"
	@echo "Plan: 100 to add, 50 to change, 25 to destroy." | time $(RELEASE_BINARY) --format json >/dev/null

.PHONY: profile
profile: ## Profile the application (requires cargo-flamegraph)
	@echo "$(GREEN)Profiling application...$(NC)"
	@if command -v cargo-flamegraph >/dev/null 2>&1; then \
		echo "Plan: 1000 to add, 500 to change, 250 to destroy." | $(CARGO) flamegraph --bin $(BINARY_NAME) -- --format terminal >/dev/null; \
		echo "$(GREEN)Flamegraph generated: flamegraph.svg$(NC)"; \
	else \
		echo "$(YELLOW)cargo-flamegraph not installed. Install with: cargo install flamegraph$(NC)"; \
	fi

##@ Information
.PHONY: info
info: version ## Show project information
	@echo ""
	@echo "$(YELLOW)Build Information:$(NC)"
	@echo "  Debug binary:   $(DEBUG_BINARY)"
	@echo "  Release binary: $(RELEASE_BINARY)"
	@echo "  Target dir:     $(TARGET_DIR)"
	@echo ""
	@echo "$(YELLOW)Project Structure:$(NC)"
	@echo "  Source:         $(SRC_DIR)/"
	@echo "  Tests:          $(TEST_DIR)/"
	@echo "  Benchmarks:     $(BENCH_DIR)/"
	@echo "  Documentation:  $(DOCS_DIR)/"
	@echo ""
	@echo "$(YELLOW)Available Targets:$(NC)"
	@for target in $(TARGETS); do echo "  $$target"; done

.PHONY: size
size: build-release ## Show binary sizes
	@echo "$(GREEN)Binary sizes:$(NC)"
	@if [ -f "$(DEBUG_BINARY)" ]; then \
		echo "  Debug:   $$(ls -lh $(DEBUG_BINARY) | awk '{print $$5}')"; \
	fi
	@if [ -f "$(RELEASE_BINARY)" ]; then \
		echo "  Release: $$(ls -lh $(RELEASE_BINARY) | awk '{print $$5}')"; \
	fi

# Phony target declaration for all targets
.PHONY: all
all: build-all test doc ## Build everything