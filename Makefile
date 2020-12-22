.PHONY: test check

test: check
	cargo test

check:
	cargo fmt -- --check
	cargo clippy