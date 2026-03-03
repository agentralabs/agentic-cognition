.PHONY: build test clippy fmt check clean install

build:
	cargo build --all-features

test:
	cargo test --all-features

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt --all

check:
	cargo check --all-features

clean:
	cargo clean

install:
	cargo install --path crates/agentic-cognition-cli

mcp:
	cargo run --bin acog-mcp

release:
	cargo build --release --all-features

all: fmt clippy test build
