BIN    := passwd-tools
PREFIX ?= $(HOME)/.local
DESTDIR ?=

.DEFAULT_GOAL := help
.PHONY: help build release run test fmt fmt-check lint check ci doc clean install uninstall

help: ## Show this help
	@awk 'BEGIN {FS = ":.*## "} /^[a-z-]+:.*## / {printf "  %-10s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Debug build
	cargo build

release: ## Optimized build
	cargo build --release

run: ## Run the CLI (ARGS="min-len 8 words.txt")
	cargo run -- $(ARGS)

test: ## Run unit and integration tests
	cargo test

fmt: ## Format the code
	cargo fmt

fmt-check: ## Verify formatting
	cargo fmt --check

lint: ## Run clippy, warnings are errors
	cargo clippy --all-targets -- -D warnings

check: ## Type-check without building
	cargo check --all-targets

ci: fmt-check lint test ## Everything CI should run

doc: ## Build and open API docs
	cargo doc --no-deps --open

clean: ## Remove build artifacts
	cargo clean

install: release ## Install to $(PREFIX)/bin
	install -D -m 0755 target/release/$(BIN) $(DESTDIR)$(PREFIX)/bin/$(BIN)

uninstall: ## Remove the installed binary
	rm -f $(DESTDIR)$(PREFIX)/bin/$(BIN)
