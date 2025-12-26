# SM-NTFS Tool - Build System
# Makefile for building Rust and Swift components

# Variables
CARGO = cargo
SWIFT = swift
RUST_DIR = rust-driver
SWIFT_DIR = macos-app
BUILD_TYPE ?= debug

# Rust build flags
ifeq ($(BUILD_TYPE),release)
    CARGO_FLAGS = --release
else
    CARGO_FLAGS =
endif

# Colors for output
RED = \033[0;31m
GREEN = \033[0;32m
YELLOW = \033[0;33m
BLUE = \033[0;34m
NC = \033[0m # No Color

.PHONY: all help clean test build-rust build-swift check format lint

# Default target
all: build-rust

# Help
help:
	@echo "$(BLUE)SM-NTFS Tool - Build System$(NC)"
	@echo ""
	@echo "Available targets:"
	@echo "  $(GREEN)all$(NC)          - Build Rust components (default)"
	@echo "  $(GREEN)build-rust$(NC)   - Build Rust workspace"
	@echo "  $(GREEN)build-swift$(NC)  - Build Swift application (TODO)"
	@echo "  $(GREEN)test$(NC)         - Run all tests"
	@echo "  $(GREEN)check$(NC)        - Check Rust code without building"
	@echo "  $(GREEN)format$(NC)       - Format all code"
	@echo "  $(GREEN)lint$(NC)         - Run linters"
	@echo "  $(GREEN)clean$(NC)        - Clean build artifacts"
	@echo ""
	@echo "Variables:"
	@echo "  BUILD_TYPE   - 'debug' or 'release' (default: debug)"
	@echo ""
	@echo "Examples:"
	@echo "  make build-rust BUILD_TYPE=release"
	@echo "  make test"

# Build Rust workspace
build-rust:
	@echo "$(BLUE)Building Rust workspace ($(BUILD_TYPE))...$(NC)"
	cd $(RUST_DIR) && $(CARGO) build $(CARGO_FLAGS)
	@echo "$(GREEN)✓ Rust build complete$(NC)"

# Build Swift application
build-swift:
	@echo "$(BLUE)Building Swift application...$(NC)"
	cd $(SWIFT_DIR) && $(SWIFT) build -c $(BUILD_TYPE)
	@echo "$(GREEN)✓ Swift build complete$(NC)"

# Run tests
test:
	@echo "$(BLUE)Running Rust tests...$(NC)"
	cd $(RUST_DIR) && $(CARGO) test
	@echo "$(GREEN)✓ Rust tests passed$(NC)"
	@echo ""
	@echo "$(YELLOW)Swift tests TODO$(NC)"

# Check Rust code
check:
	@echo "$(BLUE)Checking Rust code...$(NC)"
	cd $(RUST_DIR) && $(CARGO) check --all-targets

# Format code
format:
	@echo "$(BLUE)Formatting Rust code...$(NC)"
	cd $(RUST_DIR) && $(CARGO) fmt
	@echo "$(GREEN)✓ Rust code formatted$(NC)"

# Run linters
lint:
	@echo "$(BLUE)Running Rust clippy...$(NC)"
	cd $(RUST_DIR) && $(CARGO) clippy -- -D warnings

# Clean build artifacts
clean:
	@echo "$(BLUE)Cleaning...$(NC)"
	cd $(RUST_DIR) && $(CARGO) clean
	rm -rf dist/
	@echo "$(GREEN)✓ Clean complete$(NC)"

# Development helpers
.PHONY: watch dev

# Watch mode for Rust
watch:
	@echo "$(BLUE)Watching Rust code for changes...$(NC)"
	@echo "$(YELLOW)Note: Requires 'cargo-watch' (cargo install cargo-watch)$(NC)"
	cd $(RUST_DIR) && cargo watch -x 'build --release'

# Development mode
dev:
	@echo "$(BLUE)Starting development environment...$(NC)"
	@echo "$(YELLOW)Running Rust CLI tool...$(NC)"
	cd $(RUST_DIR) && $(CARGO) run --bin sm-ntfs-cli -- --help
