
all: format build

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

test: build
	cargo test

format:
	cargo fmt --all -- --check

