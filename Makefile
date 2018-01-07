run: target/release/6x6
	bash ./script/cli.sh

build: target/release/6x6

target/release/6x6:
	cargo build --release
