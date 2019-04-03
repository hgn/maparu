
all: format build test

build:
	cargo build

release:
	cargo build --release

run:
	cargo run

test:
	cargo test

format:
	cargo fmt --all -- --check

