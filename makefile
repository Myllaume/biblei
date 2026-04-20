.PHONY: build-fetchbib run-fetchbib test-fetchbib build-fetchlex run-fetchlex build-parsenouns run-parsenouns build run format

init:
	./apps/init.sh

build-fetchbib:
	cargo build --package fetchbib

run-fetchbib:
	cargo run --package fetchbib

test-fetchbib:
	cargo test --package fetchbib

build-fetchlex:
	cargo build --package fetchlex

run-fetchlex:
	cargo run --package fetchlex

build-filltag:
	cargo build --package filltag

run-filltag:
	cargo run --package filltag

build-parsenouns:
	poetry install

run-parsenouns:
	poetry run parsenouns

# Build all applications
build: build-fetchbib build-fetchlex build-filltag build-parsenouns

# Run all applications (sequential)
run: run-fetchbib run-fetchlex run-filltag run-parsenouns

# Run E2E tests (requires: cargo build + poetry install)
test: build
	poetry run pytest -v

# Format all code (Rust + Python)
format:
	cargo fmt --all
	poetry run ruff format
