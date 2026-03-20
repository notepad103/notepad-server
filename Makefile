.PHONY: dev watch check

dev:
	@cargo run

check:
	@cargo check

watch:
	@command -v cargo-watch >/dev/null 2>&1 || (echo "cargo-watch not installed. run: cargo install cargo-watch" && exit 1)
	@cargo watch -w src -w migrations -x run

