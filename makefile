.PHONY: all release run clean

all: 
	@cargo build
	@rm -rf build
	@mkdir -p build
	@mv target/debug/RevealNet build/

release:
	@cargo build --release

run:
	@cargo run

clean:
	@cargo clean 
