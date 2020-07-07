RUST_SOURCES = $(wildcard rust/src/**/*.rs) rust/Cargo.toml rust/Cargo.lock


.PHONY: all

all: build/test_bin

clean:
	rm -rf build
	rm -rf rust/target

build/test_bin: main.cpp rust/target/debug/librust_web_api_caller.a
	mkdir -p build
	g++ -Wall -g -o build/test_bin -Lrust/target/debug main.cpp -pthread -lrust_web_api_caller -Wl,--no-as-needed -ldl

main.cpp: rust_web_api_caller.h

rust/target/debug/librust_web_api_caller.a: $(RUST_SOURCES)
	cd rust && \
		cargo build

rust_web_api_caller.h: $(RUST_SOURCES)
	cd rust && cbindgen -o ../rust_web_api_caller.h

