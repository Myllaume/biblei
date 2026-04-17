.PHONY: build-fetchbib run-fetchbib build-fetchlex run-fetchlex build run

# Build fetchbib
build-fetchbib:
	cargo build --package fetchbib

# Run fetchbib
run-fetchbib:
	cargo run --package fetchbib

# Build fetchlex
build-fetchlex:
	cargo build --package fetchlex

# Run fetchlex
run-fetchlex:
	cargo run --package fetchlex

# Build all applications
build:
	cargo build --all

# Run all applications (sequential)
run: run-fetchbib run-fetchlex
