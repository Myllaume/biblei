.PHONY: init build build-debug run test test-e2e format lint

# ── Output files ───────────────────────────────────────────────────────────────
BIB_OUTPUT    = ./assets/bib.json
LEX_OUTPUT    = ./assets/lexique.csv
LEX_ERROR     = ./dist/lex_errors.csv
TAG_OUTPUT    = ./assets/tags.json
TAG_MATCH     = ./dist/tag_match.csv
NOUNS_OUTPUT  = ./assets/nouns.csv
RECORD_OUTPUT = ./dist/records.json

# ── Rust binaries ──────────────────────────────────────────────────────────────
FETCHBIB = ./target/release/fetchbib
FETCHLEX = ./target/release/fetchlex
FILLTAG  = ./target/release/filltag
PARSEREC = ./target/release/parserec

# ── Init ───────────────────────────────────────────────────────────────────────
init:
	./apps/init.sh

# ── Build all (compile Rust binaries + install Python deps) ────────────────────
build: $(FETCHBIB) $(FETCHLEX) $(FILLTAG) $(PARSEREC)
	poetry install

# Build a single Rust app in debug mode: make build-debug APP=fetchbib
build-debug:
	cargo build --package $(APP)

$(FETCHBIB): $(wildcard apps/fetchbib/src/*.rs) apps/fetchbib/Cargo.toml
	cargo build --package fetchbib --release

$(FETCHLEX): $(wildcard apps/fetchlex/src/*.rs) apps/fetchlex/Cargo.toml
	cargo build --package fetchlex --release

$(FILLTAG): $(wildcard apps/filltag/src/*.rs) apps/filltag/Cargo.toml
	cargo build --package filltag --release

$(PARSEREC): $(wildcard apps/parserec/src/*.rs) apps/parserec/Cargo.toml
	cargo build --package parserec --release

# ── Run all (produce all output files) ────────────────────────────────────────
run: $(BIB_OUTPUT) $(LEX_OUTPUT) $(TAG_OUTPUT) $(NOUNS_OUTPUT) $(RECORD_OUTPUT)

# fetchbib → assets/bib.json
$(BIB_OUTPUT): $(FETCHBIB)
	cargo run --package fetchbib --release

# fetchlex → assets/lexique.csv  dist/lex_errors.csv
$(LEX_OUTPUT) $(LEX_ERROR) &: $(FETCHLEX)
	cargo run --package fetchlex --release

# filltag  → assets/tags.json  dist/tag_match.csv
$(TAG_OUTPUT) $(TAG_MATCH) &: $(FILLTAG) $(LEX_OUTPUT)
	cargo run --package filltag --release

# parsenouns → assets/nouns.csv
$(NOUNS_OUTPUT):
	poetry run parsenouns

# parserec → dist/records.json
$(RECORD_OUTPUT): $(PARSEREC) $(TAG_OUTPUT)
	cargo run --package parserec --release

# ── Tests ──────────────────────────────────────────────────────────────────────
test:
	@if [ -z "$(APP)" ]; then echo "Usage: make test APP=<fetchbib|fetchlex|filltag|parserec|parsenouns>"; exit 1; fi
	@if [ "$(APP)" = "parsenouns" ]; then poetry run pytest -v apps/parsenouns; else cargo test --package $(APP); fi

test-e2e: build
	poetry run pytest -v

# ── Format & Lint ──────────────────────────────────────────────────────────────
format:
	cargo fmt --all
	poetry run ruff format

lint:
	cargo clippy --all -- -D warnings
	poetry run ruff check .