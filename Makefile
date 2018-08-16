.PHONY: clean build run

clean:
	cargo clean

run:
	cargo run --release

build:
	cargo build --release
