.PHONY: build release test run fmt clippy clean

build:
	cargo build

release:
	cargo build --release

test:
	cargo test --verbose

run:
	cargo run -- $(ARGS)

fmt:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

clean:
	cargo clean
