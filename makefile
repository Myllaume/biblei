.PHONY: build-fetchbib run-fetchbib build-fetchlex run-fetchlex build-parsenouns run-parsenouns build run

init:
	./apps/init.sh

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

# Build parsenouns
build-parsenouns:
	poetry install

# Run parsenouns
run-parsenouns:
	poetry run parsenouns

# Build all applications
build: build-fetchbib build-fetchlex build-parsenouns

# Run all applications (sequential)
run: run-fetchbib run-fetchlex run-parsenouns
