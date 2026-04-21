.PHONY: build-fetchbib run-fetchbib test-fetchbib test-parserec build-fetchlex run-fetchlex build-parsenouns run-parsenouns build run format run-parserec build-parserec run-viztime lint build-viztime

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

build-parserec:
	cargo build --package parserec

run-parserec:
	cargo run --package parserec

test-parserec:
	cargo test --package parserec

build-parsenouns:
	poetry install

run-parsenouns:
	poetry run parsenouns

build-viztime:
	npx esbuild apps/viztime/browser/main.ts --minify --bundle --platform=browser --target=es2020 --format=iife --outfile=dist/viztime/script.js
	npx esbuild apps/viztime/server/build.ts --minify --bundle --platform=node --target=node18 --format=cjs --outfile=apps/viztime/dist/build.cjs
	node apps/viztime/dist/build.cjs
	cp apps/viztime/static/* dist/viztime/

serve-viztime:
	http-server dist/viztime

# Build all applications
build: build-fetchbib build-fetchlex build-filltag build-parsenouns build-parserec

# Run all applications (sequential)
run: run-fetchbib run-fetchlex run-filltag run-parsenouns run-parserec

# Run E2E tests (requires: cargo build + poetry install)
test: build
	poetry run pytest -v

# Format all code (Rust + Python)
format:
	cargo fmt --all
	poetry run ruff format
	npx prettier . --write

lint:
	cargo clippy --all -- -D warnings
	poetry run ruff check .
	npx eslint .